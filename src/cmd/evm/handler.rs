use std::error::Error;
use std::sync::Arc;

use crate::{cli::EvmCommands, cmd::evm, config::State};

pub async fn exec(cmd: EvmCommands, state: Arc<State>) -> Result<(), Box<dyn Error>> {
    match cmd {
        EvmCommands::Deploy {
            name,
            symbol,
            amount,
        } => {
            evm::deploy::exec(
                Arc::clone(&state),
                name.clone(),
                symbol.clone(),
                amount.clone(),
            )
            .await?;
        }
        EvmCommands::Spawn { amount } => {
            evm::spawn::exec(
                Arc::clone(&state),
                amount.clone(),
            )
            .await?;
        }
    }

    Ok(())
}
