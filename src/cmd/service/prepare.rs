use std::sync::Arc;

use alloy::signers::wallet::LocalWallet;
use colored::Colorize;
use ethers::providers::{Provider, Ws};
use std::io;

use crate::{
    cmd::api::utils,
    config::{Config, TEST_WALLETS},
    Result,
};

pub async fn exec(config: Arc<Config>) -> Result<()> {
    utils::user::check_admin_exists(Arc::clone(&config)).await?;
    utils::user::check_tester_exists(Arc::clone(&config)).await?;

    Ok(())
}
