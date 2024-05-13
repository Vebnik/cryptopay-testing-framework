use alloy::{network::EthereumSigner, providers::ProviderBuilder, signers::wallet::LocalWallet};
use colored::Colorize;
use sqlx::postgres::PgPoolOptions;
use std::{io, process::exit, sync::Arc};
use tokio::net::TcpStream;

use crate::{
    cmd,
    config::{Config, ProviderType},
    Result,
};

pub async fn check_service_exists(config: Arc<Config>) -> Result<()> {
    let mut errors: Vec<i8> = Vec::with_capacity(3);

    // anvil
    let url = config.anvil_endpoint.clone().replace("http://", "");
    let stream = TcpStream::connect(url).await;
    match stream {
        Ok(_) => println!("{} Anvil -> {}", "[HEALTH]".blue(), "OK".green()),
        Err(err) => {
            println!("{} Anvil -> {} ({err})", "[HEALTH]".blue(), "ERR".red());
            errors.push(1);
        }
    };

    // cryptopay
    let url = config.cryptopay_url.clone().replace("http://", "");
    let stream = TcpStream::connect(url).await;
    match stream {
        Ok(_) => println!("{} CryptoPay -> {}", "[HEALTH]".blue(), "OK".green()),
        Err(err) => {
            println!("{} CryptoPay -> {} ({err})", "[HEALTH]".blue(), "ERR".red());
            errors.push(1);
        }
    };

    // postgres
    let stream = TcpStream::connect(config.db_host.clone()).await;
    match stream {
        Ok(_) => println!("{} Postgres -> {}", "[HEALTH]".blue(), "OK".green()),
        Err(err) => {
            println!("{} Postgres -> {} ({err})", "[HEALTH]".blue(), "ERR".red());
            errors.push(1);
        }
    };

    if !errors.is_empty() {
        exit(0)
    }

    Ok(())
}

pub async fn get_provider(config: Arc<Config>) -> Result<ProviderType> {
    let wallet = config.core_key.parse::<LocalWallet>()?;

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(EthereumSigner::from(wallet.clone()))
        .on_builtin(&config.anvil_endpoint)
        .await?;

    Ok(provider)
}

pub async fn get_db(config: Arc<Config>) -> Result<sqlx::Pool<sqlx::Postgres>> {
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&config.db_connect_url)
        .await?;

    Ok(db)
}

pub async fn get_config() -> Result<Arc<Config>> {
    Ok(Arc::new(Config::default()))
}

pub async fn check(config: Arc<Config>) -> Result<()> {
    check_service_exists(Arc::clone(&config)).await?;
    cmd::db::utils::check_db_exists(Arc::clone(&config)).await?;
    cmd::api::user::utils::check_admin_exists(Arc::clone(&config)).await?;

    Ok(())
}

pub async fn await_restart() -> Result<()> {

    let mut confirm = String::new();
    println!(
        "{} Await for restart cryptopay ... (press enter)",
        "[SERVICE]".blue()
    );
    io::stdin().read_line(&mut confirm).unwrap();
    println!("{} Restarted ...", "[SERVICE]".blue());

    Ok(())
}