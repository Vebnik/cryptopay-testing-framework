use alloy::{
    primitives::{Address, U256},
    providers::WalletProvider,
    sol,
};
use colored::Colorize;
use std::error::Error;

use crate::config::State;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20,
    "contracts/artifacts/ERC20.json"
}

pub async fn exec(
    state: State,
    name: String,
    symbol: String,
    amount: u32,
) -> Result<(), Box<dyn Error>> {
    let contract = ERC20::deploy(
        state.provider.clone(),
        name.clone().into(),
        symbol.clone().into(),
        18,
        U256::from(amount),
    )
    .await?;

    println!(
        "{} Deployed contract at address: {:?}",
        "[EVM]".blue(),
        contract.clone().address()
    );
    println!(
        "{} Minted {amount} {name} ({symbol}) to address: {:?}",
        "[EVM]".blue(),
        state.provider.signer_addresses().collect::<Vec<Address>>()
    );

    Ok(())
}
