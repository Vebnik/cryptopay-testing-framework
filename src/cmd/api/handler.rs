use std::error::Error;
use std::sync::Arc;

use crate::{
    cli::{ApiCommands, NetworkCommands, UserCommands},
    cmd::api,
    config::Config,
};

pub async fn exec(cmd: ApiCommands, config: Arc<Config>) -> Result<(), Box<dyn Error>> {
    match cmd {
        ApiCommands::User { cmd } => match cmd {
            UserCommands::Create { name, admin, email } => {
                api::user::create::exec(Arc::clone(&config), name, admin, email).await?;
            }
        },
        ApiCommands::Network { cmd } => match cmd {
            NetworkCommands::Create { name, kind } => {
                api::network::create::exec(Arc::clone(&config), name, kind).await?;
            }
        },
    };

    Ok(())
}
