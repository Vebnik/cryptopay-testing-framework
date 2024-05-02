use std::error::Error;

use crate::config::State;

pub async fn exec(state: State) -> Result<(), Box<dyn Error>> {
    sqlx::migrate!("./migrations").run(&state.db).await?;
    println!("[DB] Success migrate");

    Ok(())
}