use std::sync::Arc;

use crate::{
    cli::{ApiCommands, NetworkCommands, UserCommands},
    cmd::api,
    config::Config,
    Result,
};

pub async fn exec(cmd: ApiCommands, config: Arc<Config>) -> Result<()> {
    match cmd {
        ApiCommands::User { cmd } => match cmd {
            UserCommands::Create { name, admin, email } => {
                api::user::create::exec(Arc::clone(&config), name, email, admin).await?;
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
