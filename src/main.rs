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
use utils::{check, get_config};

use cli::Scope;
pub use config::Config;
pub use error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // init data
    let args = cli::Args::parse();
    let config = get_config().await?;

    match &args.scope {
        Scope::Evm { cmd } => cmd::evm::handler::exec(cmd.clone(), Arc::clone(&config)).await?,
        Scope::Api { cmd } => {
            check(Arc::clone(&config)).await?;

            cmd::api::handler::exec(cmd.clone(), Arc::clone(&config)).await?
        }
        Scope::Service { cmd } => {
            cmd::service::handler::exec(cmd.clone(), Arc::clone(&config)).await?
        }
        Scope::Db { cmd } => cmd::db::handler::exec(cmd.clone(), Arc::clone(&config)).await?,
    };

    Ok(())
}
