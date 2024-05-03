use colored::Colorize;
use std::{error::Error, process::exit};
use tokio::net::TcpStream;

use crate::config::Config;

pub async fn check_exist_service(config: Config) -> Result<(), Box<dyn Error>> {
    let mut errors: Vec<i8> = Vec::with_capacity(3);

    // anvil
    let url = config.anvil_endpoint.clone().replace("http://", "");
    let stream = TcpStream::connect(url).await;
    match stream {
        Ok(_) => println!("{} Anvil -> {}", "[HEALTH]".blue(), "OK".green()),
        Err(err) => {
            println!("{} Anvil -> {} ({err})", "[HEALTH]".blue(), "ERR".red());
            errors.push(1);
        }
    };

    // cryptopay
    let url = config.cryptopay_url.clone().replace("http://", "");
    let stream = TcpStream::connect(url).await;
    match stream {
        Ok(_) => println!("{} CryptoPay -> {}", "[HEALTH]".blue(), "OK".green()),
        Err(err) => {
            println!("{} CryptoPay -> {} ({err})", "[HEALTH]".blue(), "ERR".red());
            errors.push(1);
        }
    };

    // postgres
    let stream = TcpStream::connect(config.db_host).await;
    match stream {
        Ok(_) => println!("{} Postgres -> {}", "[HEALTH]".blue(), "OK".green()),
        Err(err) => {
            println!("{} Postgres -> {} ({err})", "[HEALTH]".blue(), "ERR".red());
            errors.push(1);
        }
    };

    if !errors.is_empty() {
        exit(0)
    }

    Ok(())
}
