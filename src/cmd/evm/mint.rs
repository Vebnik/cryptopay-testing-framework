use alloy::{
    primitives::{Address, U256},
    providers::ProviderBuilder,
    sol,
};
use colored::Colorize;
use std::{str::FromStr, sync::Arc};

use crate::{config::Config, Result};

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20,
    "contracts/artifacts/ERC20.json"
}

pub async fn exec(
    config: Arc<Config>,
    address: String,
    contract: String,
    amount: u32,
) -> Result<()> {
    let contract_addr = Address::from_str(&contract)?;
    let target_addr = Address::from_str(&address)?;
    let decimals = U256::from(10).checked_pow(U256::from(18)).unwrap();

    for port in 8545..(8545 + config.anvil_nodes as i32) {
        let provide = ProviderBuilder::new()
            .on_builtin(&format!("http://localhost:{}", port))
            .await?;

        let contract = ERC20::new(contract_addr, provide);
        let amount = U256::from(amount).checked_mul(decimals).unwrap();

        let receipt = contract.mint(target_addr, amount).send().await?.get_receipt().await?;

        println!(
            "{} Minted {amount} at address: {:?} ({})",
            "[EVM]".blue(),
            target_addr.clone(),
            receipt.transaction_hash
        );
    }

    Ok(())
}
