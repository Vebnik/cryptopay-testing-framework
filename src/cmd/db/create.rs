use colored::Colorize;
use std::{error::Error, sync::Arc};

use crate::{cmd::db::utils::check_exist_db, config::Config, utils};

pub async fn exec(config: Arc<Config>) -> Result<(), Box<dyn Error>> {
    check_exist_db(Arc::clone(&config)).await?;

    let db = utils::get_db(Arc::clone(&config)).await?;

    println!("{} Try to migrate", "[DB]".blue());
    sqlx::migrate!("./migrations").run(&db).await?;
    println!("{} Success migrated", "[DB]".blue());

    Ok(())
}
