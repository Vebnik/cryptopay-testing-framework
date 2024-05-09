use alloy::{
    primitives::{Address, U256},
    sol,
};
use colored::Colorize;
use std::{error::Error, str::FromStr, sync::Arc};

use crate::config::{Config, ProviderType};

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
    provider: ProviderType,
) -> Result<(), Box<dyn Error>> {
    let recipient = Address::from_str(&recipient)?;
    let contract = Address::from_str(&contract)?;
    let sender = Address::from_str(&sender)?;

    let decimals = U256::from(10).checked_pow(U256::from(18)).unwrap();
    let amount = U256::from(amount).checked_mul(decimals).unwrap();

    let contract = ERC20::new(contract, provider);

    println!("{}", amount);

    let receipt = contract
        .transfer(recipient, amount)
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
