use std::sync::Arc;

use crate::{cli::EvmCommands, cmd::evm, config::Config, Result};

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
    }

    Ok(())
}
