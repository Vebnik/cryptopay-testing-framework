use std::error::Error;

use crate::cmd;
use crate::{
    cli::{ApiCommands, NetworkCommands},
    config::State,
};

pub async fn exec(cmd: ApiCommands, state: State) -> Result<(), Box<dyn Error>> {
    match cmd {
        ApiCommands::UserFlow => {
            println!("ApiCommands::UserFlow");
        }
        ApiCommands::Network { cmd } => match cmd {
            NetworkCommands::Create {
                name,
                kind,
                endpoint,
            } => {
                cmd::api::network::create::exec(name, kind, endpoint, state).await?;
            }
        },
    };

    Ok(())
}
