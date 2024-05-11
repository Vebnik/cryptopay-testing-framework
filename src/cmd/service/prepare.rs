use std::{error::Error, sync::Arc};

use alloy::signers::wallet::LocalWallet;
use colored::Colorize;
use ethers::providers::{Provider, Ws};
use std::io;

use crate::config::{Config, TEST_WALLETS};

pub async fn exec(config: Arc<Config>) -> Result<(), Box<dyn Error>> {
    Ok(())
}
