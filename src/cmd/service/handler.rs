use std::error::Error;
use std::sync::Arc;

use crate::{cli::ServiceCommands, cmd::service, config::State};

pub async fn exec(cmd: ServiceCommands, state: Arc<State>) -> Result<(), Box<dyn Error>> {
    match cmd {
        ServiceCommands::Full => service::full_flow::exec(state).await?,
    };

    Ok(())
}
