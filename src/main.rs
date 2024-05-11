#![warn(clippy::all, clippy::pedantic)]

pub mod cli;
pub mod cmd;
pub mod config;
pub mod error;
pub mod tests;
pub mod utils;

use clap::Parser;
use colored::Colorize;
use std::sync::Arc;
use utils::get_config;

use cli::ProcessType;
pub use config::Config;
pub use error::{Error, Result};

async fn check(config: Arc<Config>) -> Result<()> {
    utils::check_exist_service(Arc::clone(&config)).await?;
    cmd::db::utils::check_db_exists(Arc::clone(&config)).await?;
    cmd::api::utils::user::check_admin_exists(Arc::clone(&config)).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // init data
    let args = cli::Args::parse();
    let config = get_config().await?;

    match &args.process {
        ProcessType::Evm { cmd } => {
            cmd::evm::handler::exec(cmd.clone(), Arc::clone(&config)).await?
        }
        ProcessType::Api { cmd } => {
            check(Arc::clone(&config)).await?;

            cmd::api::handler::exec(cmd.clone(), Arc::clone(&config)).await?
        }
        ProcessType::Service { cmd } => {
            check(Arc::clone(&config)).await?;

            cmd::service::handler::exec(cmd.clone(), Arc::clone(&config)).await?
        }
        ProcessType::Db { cmd } => cmd::db::handler::exec(cmd.clone(), Arc::clone(&config)).await?,
    };

    Ok(())
}
