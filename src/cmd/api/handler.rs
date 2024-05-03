use std::error::Error;
use std::sync::Arc;

use crate::cmd::api;
use crate::{
    cli::{ApiCommands, NetworkCommands, UserCommands},
    config::State,
};

pub async fn exec(cmd: ApiCommands, state: Arc<State>) -> Result<(), Box<dyn Error>> {
    match cmd {
        ApiCommands::User { cmd } => match cmd {
            UserCommands::Create { name, admin, email} => {
                api::user::create::exec(state, name, admin, email).await?;
            },
        },
        ApiCommands::Network { cmd } => match cmd {
            NetworkCommands::Create {
                name,
                kind,
                endpoint,
            } => api::network::create::exec(name, kind, endpoint, state).await?,
        },
    };

    Ok(())
}
