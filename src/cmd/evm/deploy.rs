use alloy::{
    network::EthereumSigner,
    primitives::{Address, U256},
    providers::{ProviderBuilder, WalletProvider},
    signers::wallet::LocalWallet,
    sol,
};
use colored::Colorize;
use std::{error::Error, sync::Arc};

use crate::config::State;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20,
    "contracts/artifacts/ERC20.json"
}

pub async fn exec(
    state: Arc<State>,
    name: String,
    symbol: String,
    amount: u32,
) -> Result<Vec<String>, Box<dyn Error>> {
    let wallet = state.config.core_priv_key.parse::<LocalWallet>()?;

    let mut contracts_addresses: Vec<String> =
        Vec::with_capacity(state.config.anvil_nodes as usize);

    for port in 8545..(8545 + state.config.anvil_nodes as i32) {
        let provide = ProviderBuilder::new()
            .with_recommended_fillers()
            .signer(EthereumSigner::from(wallet.clone()))
            .on_builtin(&format!("http://localhost:{}", port))
            .await?;

        let contract = ERC20::deploy(
            provide.clone(),
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
            provide.signer_addresses().collect::<Vec<Address>>()
        );

        contracts_addresses.push(contract.clone().address().to_string())
    }

    Ok(contracts_addresses)
}
