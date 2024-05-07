#[tokio::test]
pub async fn exist_db_test() {
    use std::sync::Arc;

    use crate::cmd::db::utils::check_exist_db;
    use crate::config;

    let config = Arc::new(config::Config::default());

    check_exist_db(Arc::clone(&config)).await.unwrap();

    assert!(true)
}
