use alloy::{
    primitives::{Address, U256},
    providers::RootProvider,
    sol,
    transports::BoxTransport,
};
use colored::Colorize;
use std::{error::Error, sync::Arc};

use crate::config::Config;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20,
    "contracts/artifacts/ERC20.json"
}

pub async fn exec(
    _config: Arc<Config>,
    recipient: String,
    sender: String,
    contract: String,
    amount: u32,
    provider: RootProvider<BoxTransport>,
) -> Result<(), Box<dyn Error>> {
    let recipient = Address::parse_checksummed(recipient, Some(31337))?;
    let sender = Address::parse_checksummed(sender, Some(31337))?;
    let contract = Address::parse_checksummed(contract, Some(31337))?;

    let decimals = U256::from(10).checked_pow(U256::from(18)).unwrap();

    let contract = ERC20::new(contract, provider);
    let amount = U256::from(amount).checked_mul(decimals).unwrap();

    let receipt = contract
        .transferFrom(sender, recipient, amount)
        .send()
        .await?
        .get_receipt()
        .await?;

    println!(
        "{} Send transaction: {}",
        "[EVM]".blue(),
        receipt.transaction_hash
    );

    println!(
        "{} Transfer {amount} from {} to {}",
        "[EVM]".blue(),
        sender.clone(),
        recipient.clone()
    );

    Ok(())
}
