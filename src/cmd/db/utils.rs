use colored::Colorize;
use alloy::providers::ProviderBuilder;
use std::{error::Error, process::{exit, ExitCode, Stdio}};
use tokio::{net::TcpStream, process::Command};

use crate::config::Config;

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


pub async fn check_exist_service(_config: Config) -> Result<(), Box<dyn Error>> {
    let mut errors: Vec<i8> = Vec::with_capacity(3);

    // anvil
    let stream = TcpStream::connect("127.0.0.1:8545").await;
    match stream {
        Ok(_) => println!("{} Anvil -> {}", "[HEALTH]".blue(), "OK".green()),
        Err(err) => {
            println!("{} Anvil -> {} ({err})", "[HEALTH]".blue(), "ERR".red());
            errors.push(1);
        },
    };

    // cryptopay
    let stream = TcpStream::connect("127.0.0.1:9999").await;
    match stream {
        Ok(_) => println!("{} CryptoPay -> {}", "[HEALTH]".blue(), "OK".green()),
        Err(err) => {
            println!("{} CryptoPay -> {} ({err})", "[HEALTH]".blue(), "ERR".red());
            errors.push(1);
        },
    };

    // postgres
    let stream = TcpStream::connect("127.0.0.1:5432").await;
    match stream {
        Ok(_) => println!("{} Postgres -> {}", "[HEALTH]".blue(), "OK".green()),
        Err(err) => {
            println!("{} Postgres -> {} ({err})", "[HEALTH]".blue(), "ERR".red());
            errors.push(1);
        },
    };

    if !errors.is_empty() {
        exit(0)
    }

    Ok(())
}