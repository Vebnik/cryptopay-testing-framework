use colored::Colorize;
use std::sync::Arc;

use crate::{
    cmd::{db, service},
    config::Config,
    Result,
};

pub async fn exec(config: Arc<Config>) -> Result<()> {
    db::utils::check_db_exists(Arc::clone(&config)).await?;

    let db = service::utils::get_db(Arc::clone(&config)).await?;

    println!("{} Migrating the database", "[DB]".blue());
    sqlx::migrate!("./migrations").run(&db).await?;
    println!("{} Successfully migrated", "[DB]".blue());

    Ok(())
}
