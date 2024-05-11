use std::sync::Arc;

use crate::{cli::ServiceCommands, cmd::service, config::Config, Result};

pub async fn exec(cmd: ServiceCommands, config: Arc<Config>) -> Result<()> {
    match cmd {
        ServiceCommands::Full => service::full_flow::exec(config).await?,
    };

    Ok(())
}
