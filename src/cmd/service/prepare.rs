use std::sync::Arc;

use alloy::signers::wallet::LocalWallet;
use colored::Colorize;
use ethers::providers::{Provider, Ws};
use std::io;

use crate::{
    config::{Config, TEST_WALLETS},
    Result,
};

pub async fn exec(config: Arc<Config>) -> Result<()> {
    Ok(())
}
