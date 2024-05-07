use std::error::Error;
use std::sync::Arc;

use crate::{cli::DbCommands, cmd::db, config::Config};

pub async fn exec(cmd: DbCommands, config: Arc<Config>) -> Result<(), Box<dyn Error>> {
    match cmd {
        DbCommands::Drop => db::drop::exec(Arc::clone(&config)).await?,
        DbCommands::Create => db::create::exec(Arc::clone(&config)).await?,
    }

    Ok(())
}
