use colored::Colorize;
use std::sync::Arc;

use crate::{
    cmd::{self, db::utils::check_db_exists},
    config::Config,
    utils, Result,
};

pub async fn exec(config: Arc<Config>) -> Result<()> {
    check_db_exists(Arc::clone(&config)).await?;

    println!("{} Dropping the database", "[DB]".blue());

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
            println!("{} Successfully dropped", "[DB]".blue());
            cmd::db::create::exec(Arc::clone(&config)).await?;
        }
        Err(err) => println!("{} Error in drop: {}", "[DB]".blue(), err.to_string().red()),
    };

    Ok(())
}
