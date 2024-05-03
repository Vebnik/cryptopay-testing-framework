#![warn(clippy::all, clippy::pedantic)]

pub mod cli;
pub mod cmd;
pub mod config;
pub mod utils;

use alloy::{network::EthereumSigner, providers::ProviderBuilder, signers::wallet::LocalWallet};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use std::error::Error;

use cli::{DbCommands, EvmCommands, ProcessType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // init data
    let args = cli::Args::parse();
    let config = config::Config::default();

    // start utils
    utils::check_exist_service(config.clone()).await?;
    cmd::db::utils::check_exist_db().await?;

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

    let state = config::State {
        config,
        provider,
        db,
    };

    match &args.process {
        ProcessType::Evm { cmd } => match cmd {
            EvmCommands::Deploy {
                name,
                symbol,
                amount,
            } => {
                cmd::evm::deploy::exec(state, name.clone(), symbol.clone(), amount.clone()).await?;
            }
        },
        ProcessType::Api { cmd } => cmd::api::handler::exec(cmd.clone(), state).await?,
        ProcessType::Db { cmd } => match cmd {
            DbCommands::Drop => cmd::db::drop::exec(state).await?,
            DbCommands::Create => cmd::db::create::exec(state).await?,
        },
    }

    Ok(())
}
