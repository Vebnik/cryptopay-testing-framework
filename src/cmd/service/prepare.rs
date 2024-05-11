use std::sync::Arc;

use alloy::signers::wallet::LocalWallet;
use colored::Colorize;
use ethers::providers::{Provider, Ws};
use std::io;

use crate::{
    cmd,
    config::{Config, TEST_WALLETS},
    Result,
};

pub async fn exec(config: Arc<Config>) -> Result<()> {
    cmd::db::utils::check_db_exists(Arc::clone(&config)).await?;
    crate::utils::check_exist_service(Arc::clone(&config)).await?;

    cmd::api::utils::user::check_admin_exists(Arc::clone(&config)).await?;
    cmd::api::utils::user::check_tester_exists(Arc::clone(&config)).await?;

    Ok(())
}
