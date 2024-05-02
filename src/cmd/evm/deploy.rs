use colored::Colorize;
use std::error::Error;
use alloy::{
    network::EthereumSigner, primitives::U256, providers::ProviderBuilder, signers::wallet::LocalWallet, sol
};

use crate::config::State;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20,
    "contracts/artifacts/ERC20.json"
}

pub async fn exec(state: State, name: String, symbol: String, amount: u32) -> Result<(), Box<dyn Error>> {
    let signer: LocalWallet = state.anvil.keys()[0].clone().into();

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(EthereumSigner::from(signer.clone()))
        .on_provider(state.provider);

    let contract = ERC20::deploy(
        provider,
        name.clone().into(),
        symbol.clone().into(),
        18,
        U256::from(amount),
    ).await?;

    println!("{} Deployed contract at address: {:?}", "[EVM]".blue(), contract.clone().address());
    println!("{} Minted {amount} {name} ({symbol}) to address: {:?}", "[EVM]".blue(), signer.address());

    Ok(())
}