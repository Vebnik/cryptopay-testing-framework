use colored::Colorize;
use sqlx::migrate::MigrateDatabase;
use std::sync::Arc;

// use crate::{config::Config, utils};
use crate::{config::Config, Result};

pub async fn check_db_exists(config: Arc<Config>) -> Result<()> {
    let does_exist = sqlx::Postgres::database_exists(&config.db_connect_url)
        .await
        .expect("Error in check exist db");

    match does_exist {
        false => {
            println!("{} Database does not exist, trying to create", "[DB]".blue());
            sqlx::Postgres::create_database(&config.db_connect_url).await?;
            println!("{} Success creating database", "[DB]".blue());

            // println!("{} Try to migrate", "[DB]".blue());
            // let db = utils::get_db(Arc::clone(&config)).await?;
            // sqlx::migrate!("./migrations").run(&db).await?;
            // println!("{} Success migrated", "[DB]".blue());

            // @TODO maybe overhead
            // drop(db)
        }
        true => {
            println!("{} Database exists", "[DB]".blue())
        }
    }

    Ok(())
}
