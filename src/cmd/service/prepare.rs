use std::sync::Arc;

use colored::Colorize;

use crate::{cmd::{self, service::utils}, config::Config, Result};

/// Prepare the service for development
/// -> Check if the database exists ✅
/// -> Check if the API is running ✅
/// -> Reset the database ✅
/// -> Check and create admin user ✅
/// -> Check and create tester user ✅
/// -> Check and create networks ✅
/// -> Check and deploy tokens to core wallet ✅
/// -> Check and add assets to networks ✅
/// -> Check and create wallet for tester user ✅
pub async fn exec(config: Arc<Config>) -> Result<()> {
    cmd::db::utils::check_db_exists(Arc::clone(&config)).await?;
    cmd::service::utils::check_service_exists(Arc::clone(&config)).await?;

    // cmd::db::reset::exec(Arc::clone(&config)).await?;

    cmd::api::user::utils::check_admin_exists(Arc::clone(&config)).await?;
    cmd::api::user::utils::check_tester_exists(Arc::clone(&config)).await?;

    let networks = cmd::api::network::utils::check_networks_exist(Arc::clone(&config)).await?;

    dbg!(networks.len());

    utils::await_restart().await?;

    let contracts = cmd::evm::deploy::check_contracts_exist(Arc::clone(&config)).await?;

    dbg!(contracts.len());

    let _ = cmd::api::asset::check_assets_exist(Arc::clone(&config), networks.clone(), contracts)
        .await?;

    let _ = cmd::api::wallet::check_wallets_exist(Arc::clone(&config), networks).await?;

    // let _ = cmd::api::hot_wallet::check_wallets_exist(Arc::clone(&config)).await?;

    println!(
        "{} Service prepared for development",
        "[SERVICE - PREPARE]".blue()
    );

    Ok(())
}
