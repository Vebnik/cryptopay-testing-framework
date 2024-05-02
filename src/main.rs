#![warn(clippy::all, clippy::pedantic)] 

use std::error::Error;
use alloy::{node_bindings::Anvil, providers::ProviderBuilder};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;

use cli::{EvmCommands, DbCommands, ProcessType};

pub mod config;
pub mod cmd;
pub mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();

    let config = config::Config::default();

    cmd::db::utils::check_exist_service(config.clone()).await?;
    cmd::db::utils::check_exist_db().await?;

    let anvil = Anvil::new().try_spawn()?;

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_builtin(&config.test_network_endpoint)
        .await?;

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&config.database_url)
        .await?;

    let state = config::State { anvil, config, provider, db };

    match &args.process {
        ProcessType::Evm { cmd } => {
            match cmd {
                EvmCommands::Deploy {name, symbol, amount} => {
                    cmd::evm::deploy::exec(state, name.clone(), symbol.clone(), amount.clone()).await?;
                },
            }
        },
        ProcessType::Api { cmd } => {
            cmd::api::handler::exec(cmd.clone(), state).await?
        },
        ProcessType::Db { cmd } => {
            match cmd {
                DbCommands::Drop => cmd::db::drop::exec(state).await?,
                DbCommands::Create => cmd::db::create::exec(state).await?,
            }
        },
    }

    Ok(())
}
