use std::sync::Arc;

use crate::{Result, cli::DbCommands, cmd::db, config::Config};

pub async fn exec(cmd: DbCommands, config: Arc<Config>) -> Result<()> {
    match cmd {
        DbCommands::Reset => db::reset::exec(Arc::clone(&config)).await?,
        DbCommands::Create => db::create::exec(Arc::clone(&config)).await?,
    }

    Ok(())
}
