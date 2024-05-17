use std::sync::Arc;

use bigdecimal::Zero;
use ethers::providers::{Provider, Ws};

use crate::{
    cli::EvmCommands,
    cmd::evm,
    config::{Config, TEST_WALLETS},
    Result,
};

pub async fn exec(cmd: EvmCommands, config: Arc<Config>) -> Result<()> {
    match cmd {
        EvmCommands::Deploy {
            name,
            symbol,
            amount,
        } => {
            evm::deploy::exec(
                Arc::clone(&config),
                name.clone(),
                symbol.clone(),
                18,
                amount.clone(),
            )
            .await?;
        }
        EvmCommands::Mint {
            contract,
            address,
            amount,
        } => {
            evm::mint::exec(Arc::clone(&config), address, contract, amount.clone()).await?;
        }
        EvmCommands::Spawn { amount } => {
            evm::spawn::exec(Arc::clone(&config), amount.clone()).await?;
        }
        EvmCommands::Tx {
            chain_idx,
            contract,
            sender_idx,
            recipient,
            amount,
        } => {
            let sender_key = if sender_idx.is_zero() {
                config.core_key.clone()
            } else {
                TEST_WALLETS[sender_idx as usize].clone().1.to_owned()
            };

            let port = config.evm_port + chain_idx;
            let provider = Provider::<Ws>::connect(format!("ws://127.0.0.1:{port}"))
                .await
                .unwrap();

            evm::transfer::exec_ethers(
                Arc::clone(&config),
                recipient,
                sender_key,
                contract,
                amount.clone(),
                provider,
            )
            .await?;
        }
    }

    Ok(())
}
