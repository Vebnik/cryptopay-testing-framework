use colored::Colorize;
use sqlx::migrate::MigrateDatabase;
use std::{error::Error, sync::Arc};

// use crate::{config::Config, utils};
use crate::config::Config;

pub async fn check_exist_db(config: Arc<Config>) -> Result<(), Box<dyn Error>> {
    let is_exist = sqlx::Postgres::database_exists(&config.db_connect_url)
        .await
        .expect("Error in check exist db");

    match is_exist {
        false => {
            println!("{} Database not exist, try to create", "[DB]".blue());
            sqlx::Postgres::create_database(&config.db_connect_url).await?;
            println!("{} Success create database", "[DB]".blue());

            // println!("{} Try to migrate", "[DB]".blue());
            // let db = utils::get_db(Arc::clone(&config)).await?;
            // sqlx::migrate!("./migrations").run(&db).await?;
            // println!("{} Success migrated", "[DB]".blue());

            // @TODO maybe overhead
            // drop(db)
        }
        true => {
            println!("{} Database exist", "[DB]".blue())
        }
    }

    Ok(())
}
