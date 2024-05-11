use colored::Colorize;
use std::sync::Arc;

use crate::{cmd::db::utils::check_db_exists, config::Config, utils, Result};

pub async fn exec(config: Arc<Config>) -> Result<()> {
    check_db_exists(Arc::clone(&config)).await?;

    let db = utils::get_db(Arc::clone(&config)).await?;

    println!("{} Migrating the database", "[DB]".blue());
    sqlx::migrate!("./migrations").run(&db).await?;
    println!("{} Successfully migrated", "[DB]".blue());

    Ok(())
}
