use std::error::Error;
use std::sync::Arc;

use crate::{cli::ServiceCommands, cmd::service, config::Config};

pub async fn exec(cmd: ServiceCommands, config: Arc<Config>) -> Result<(), Box<dyn Error>> {
    match cmd {
        ServiceCommands::Full => service::full_flow::exec(config).await?,
    };

    Ok(())
}
