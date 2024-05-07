#![warn(clippy::all, clippy::pedantic)]

pub mod cli;
pub mod cmd;
pub mod config;
pub mod tests;
pub mod utils;

use clap::Parser;
use colored::Colorize;
use std::{error::Error, sync::Arc};
use utils::get_config;

use cli::ProcessType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // init data
    let args = cli::Args::parse();
    let config = get_config().await?;

    // start utils
    if !args.skip {
        utils::check_exist_service(Arc::clone(&config)).await?;
        cmd::db::utils::check_exist_db(Arc::clone(&config)).await?;
        cmd::api::utils::user::check_exist_system_user(Arc::clone(&config)).await?;
    } else {
        println!("{} Skip all check", "[SERVICE]".blue());
    }

    match &args.process {
        ProcessType::Evm { cmd } => {
            cmd::evm::handler::exec(cmd.clone(), Arc::clone(&config)).await?
        }
        ProcessType::Api { cmd } => {
            cmd::api::handler::exec(cmd.clone(), Arc::clone(&config)).await?
        }
        ProcessType::Service { cmd } => {
            cmd::service::handler::exec(cmd.clone(), Arc::clone(&config)).await?
        }
        ProcessType::Db { cmd } => cmd::db::handler::exec(cmd.clone(), Arc::clone(&config)).await?,
    };

    Ok(())
}
