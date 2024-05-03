use std::error::Error;
use std::sync::Arc;

use crate::{cli::DbCommands, cmd::db, config::State};

pub async fn exec(cmd: DbCommands, state: Arc<State>) -> Result<(), Box<dyn Error>> {
    match cmd {
        DbCommands::Drop => db::drop::exec(Arc::clone(&state)).await?,
        DbCommands::Create => db::create::exec(Arc::clone(&state)).await?,
    }

    Ok(())
}
