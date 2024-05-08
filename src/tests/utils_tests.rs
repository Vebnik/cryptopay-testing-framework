#[tokio::test]
pub async fn exist_db_test() {
    use std::sync::Arc;

    use crate::cmd::db::utils::check_exist_db;
    use crate::config;

    let config = Arc::new(config::Config::default());

    check_exist_db(Arc::clone(&config)).await.unwrap();

    assert!(true)
}

#[tokio::test]
pub async fn stdin_await_test() {
    use colored::Colorize;
    use std::io;

    let mut confirm = String::new();

    println!(
        "{} Await for restart cryptopay ... (press enter)",
        "[SERVICE]".blue()
    );

    io::stdin().read_line(&mut confirm).unwrap();

    println!("{} Restarted ...", "[SERVICE]".blue());

    assert!(true)
}
