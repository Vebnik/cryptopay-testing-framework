#![warn(clippy::all, clippy::pedantic)]

pub mod cli;
pub mod cmd;
pub mod config;
pub mod utils;

use alloy::{network::EthereumSigner, providers::ProviderBuilder, signers::wallet::LocalWallet};
use clap::Parser;
use colored::Colorize;
use sqlx::postgres::PgPoolOptions;
use std::{error::Error, sync::Arc};

use cli::ProcessType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // init data
    let args = cli::Args::parse();
    let config = config::Config::default();

    // start utils
    if !args.skip {
        utils::check_exist_service(config.clone()).await?;
        cmd::db::utils::check_exist_db().await?;
    } else {
        println!("{} Skip all check", "[SERVICE]".blue());
    }

    // evm provider
    let wallet = config.core_priv_key.parse::<LocalWallet>()?;
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(EthereumSigner::from(wallet.clone()))
        .on_builtin(&config.anvil_endpoint)
        .await?;

    // database
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&config.db_connect_url)
        .await?;

    let state = Arc::new(config::State {
        args: args.clone(),
        config,
        provider,
        db,
    });

    // other cheks
    cmd::api::utils::user::check_exist_system_user(Arc::clone(&state)).await?;

    match &args.process {
        ProcessType::Evm { cmd } => {
            cmd::evm::handler::exec(cmd.clone(), Arc::clone(&state)).await?
        }
        ProcessType::Api { cmd } => {
            cmd::api::handler::exec(cmd.clone(), Arc::clone(&state)).await?
        }
        ProcessType::Db { cmd } => cmd::db::handler::exec(cmd.clone(), Arc::clone(&state)).await?,
    }

    Ok(())
}
