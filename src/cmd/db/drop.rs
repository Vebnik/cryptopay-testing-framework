use colored::Colorize;
use std::error::Error;
use std::sync::Arc;

use crate::cmd;
use crate::config::State;

pub async fn exec(state: Arc<State>) -> Result<(), Box<dyn Error>> {
    let result = sqlx::raw_sql(
        r#"
        drop schema public cascade;
        create schema public;
    "#,
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => {
            println!("{} Success drop", "[DB]".blue());
            cmd::db::create::exec(state).await?;
        }
        Err(err) => println!("{} Error in drop: {}", "[DB]".blue(), err.to_string().red()),
    };

    Ok(())
}
