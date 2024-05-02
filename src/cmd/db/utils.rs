use colored::Colorize;
use std::{error::Error, process::Stdio};
use tokio::process::Command;

pub async fn check_exist_db() -> Result<(), Box<dyn Error>> {

    let exit_code = Command::new("createdb")
        .arg("-U")
        .arg("postgres")
        .arg("test")
        .stderr(Stdio::null())
        .spawn()
        .expect("Error in check exist db")
        .wait()
        .await?;

    match exit_code.code().unwrap() {
        0 => println!("{} Success create database", "[DB]".blue()),
        1 => println!("{} Database exist", "[DB]".blue()),
        _ => println!("{} Not normal status code", "[DB]".blue()),
    };

    Ok(())
}