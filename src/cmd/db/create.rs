use colored::Colorize;
use std::{error::Error, sync::Arc};

use crate::config::State;

pub async fn exec(state: Arc<State>) -> Result<(), Box<dyn Error>> {
    sqlx::migrate!("./migrations").run(&state.db).await?;
    println!("{} Success migrate", "[DB]".blue());

    Ok(())
}
