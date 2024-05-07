use colored::Colorize;
use std::error::Error;
use std::sync::Arc;

use crate::{
    cmd::{self, db::utils::check_exist_db},
    config::Config,
    utils,
};

pub async fn exec(config: Arc<Config>) -> Result<(), Box<dyn Error>> {
    check_exist_db(Arc::clone(&config)).await?;

    println!("{} Try to drop", "[DB]".blue());

    let db = utils::get_db(Arc::clone(&config)).await?;

    let result = sqlx::raw_sql(
        r#"
        drop schema public cascade;
        create schema public;
    "#,
    )
    .execute(&db)
    .await;

    match result {
        Ok(_) => {
            println!("{} Success drop", "[DB]".blue());
            cmd::db::create::exec(Arc::clone(&config)).await?;
        }
        Err(err) => println!("{} Error in drop: {}", "[DB]".blue(), err.to_string().red()),
    };

    Ok(())
}
