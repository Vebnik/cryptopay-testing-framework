use alloy::{
    network::EthereumSigner,
    primitives::{Address, U256},
    providers::ProviderBuilder,
    signers::wallet::LocalWallet,
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
    let wallet = config.core_priv_key.parse::<LocalWallet>()?;
    let contract_addr = Address::from_str(&contract)?;
    let target_addr = Address::from_str(&address)?;
    let decimals = U256::from(10).checked_pow(U256::from(18)).unwrap();

    for port in 8545..(8545 + config.anvil_nodes as i32) {
        let provide = ProviderBuilder::new()
            .with_recommended_fillers()
            .signer(EthereumSigner::from(wallet.clone()))
            .on_builtin(&format!("http://localhost:{}", port))
            .await?;

        let contract = ERC20::new(contract_addr, provide);
        let amount = U256::from(amount).checked_mul(decimals).unwrap();

        let _ = contract.mint(target_addr, amount).send().await?;

        println!(
            "{} Minted {amount} at address: {:?}",
            "[EVM]".blue(),
            target_addr.clone()
        );
    }

    Ok(())
}
