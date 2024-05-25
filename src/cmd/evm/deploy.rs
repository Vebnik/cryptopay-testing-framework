use alloy::{
    network::EthereumSigner,
    primitives::{Address, U256},
    providers::{ProviderBuilder, WalletProvider},
    signers::wallet::LocalWallet,
    sol,
};
use colored::Colorize;
use std::sync::Arc;

use crate::{
    config::{Config, TEST_TOKENS},
    Result,
};

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20,
    "contracts/artifacts/ERC20.json"
}

pub async fn exec(
    config: Arc<Config>,
    name: String,
    symbol: String,
    decimals: u8,
    amount: u32,
) -> Result<Vec<String>> {
    let wallet = config.core_key.parse::<LocalWallet>()?;

    let mut contracts_addresses: Vec<String> = Vec::with_capacity(config.evm_nodes as usize);

    for port in 8545..(8545 + config.evm_nodes as i32) {
        let provide = ProviderBuilder::new()
            .with_recommended_fillers()
            .signer(EthereumSigner::from(wallet.clone()))
            .on_builtin(&format!("http://localhost:{}", port))
            .await?;

        let contract = ERC20::deploy(
            provide.clone(),
            name.clone().into(),
            symbol.clone().into(),
            decimals,
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

pub async fn check_contracts_exist(config: Arc<Config>) -> Result<Vec<Vec<String>>> {
    // Deploy contracts
    let mut contracts: Vec<Vec<String>> = Vec::with_capacity(TEST_TOKENS.len());

    for (name, symbol, decimals) in TEST_TOKENS {
        let addresses = exec(
            Arc::clone(&config),
            name.into(),
            symbol.into(),
            decimals,
            10000,
        )
        .await?;

        contracts.push(addresses)
    }

    return Ok(contracts);
}
