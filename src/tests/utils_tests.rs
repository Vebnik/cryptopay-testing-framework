use reqwest::Url;


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

#[tokio::test]
pub async fn transfer_test() {
    use alloy::{
        network::EthereumSigner, providers::ProviderBuilder, signers::wallet::LocalWallet,
    };
    use std::sync::Arc;
    use colored::Colorize;

    use crate::cmd::evm::transfer;
    use crate::config::{Config, TEST_WALLETS};

    let priv_key = TEST_WALLETS[0].1;
    let config = Config::default();
    let wallet = priv_key.parse::<LocalWallet>().unwrap();
    let sender = wallet.address().to_string();
    let contract = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
    let recipient = "0xed6a4772ab0fde25b19f2726c3a2ff6bc091ab32";

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(EthereumSigner::from(wallet.clone()))
        .on_builtin(&format!("http://localhost:{}", "8545"))
        .await
        .unwrap();

    transfer::exec(
        Arc::new(config),
        recipient.into(),
        sender.clone(),
        contract.into(),
        420,
        provider,
    )
    .await
    .unwrap();

    println!("{} recipient: {}", "[SERVICE]".blue(), sender,);

    assert!(true)
}


#[tokio::test]
pub async fn balance_test() {
    use alloy::{primitives::Address, sol};
    use alloy::{
        network::EthereumSigner, providers::ProviderBuilder, signers::wallet::LocalWallet,
    };
    use std::str::FromStr;
    use alloy::primitives::U256;
    use std::sync::Arc;
    use colored::Colorize;

    use crate::cmd::evm::transfer;
    use crate::config::{Config, TEST_WALLETS};

    sol! {
        #[allow(missing_docs)]
        #[sol(rpc)]
        ERC20,
        "contracts/artifacts/ERC20.json"
    }

    // let priv_key = TEST_WALLETS[0].1;
    let config = Config::default();
    // let wallet = priv_key.parse::<LocalWallet>().unwrap();
    let wallet = config.core_priv_key.parse::<LocalWallet>().unwrap();
    let core = wallet.address().to_string();
    let contract_address = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
    // let recipient = "0xed6a4772ab0fde25b19f2726c3a2ff6bc091ab32";

    let provider = ProviderBuilder::new()
        // .with_recommended_fillers()
        // .signer(EthereumSigner::from(wallet.clone()))
        .on_http(format!("http://localhost:{}", "8545").parse::<Url>().unwrap());

    let contract = ERC20::new(Address::from_str(contract_address).unwrap(), provider);

    let balance = contract.balanceOf(Address::from_str(&core).unwrap()).call().await.unwrap();
    println!("Before core: {}", balance._0);

    let balance = contract.balanceOf(Address::from_str(TEST_WALLETS[0].0).unwrap()).call().await.unwrap();
    println!("Before sender: {}", balance._0);

    let balance = contract.balanceOf(Address::from_str(TEST_WALLETS[1].0).unwrap()).call().await.unwrap();
    println!("Before recipient: {}", balance._0);

    let tx = contract.transferFrom(
        Address::from_str(TEST_WALLETS[0].0).unwrap(),
        Address::from_str(TEST_WALLETS[1].0).unwrap(),
        U256::from(10)
    );

    tx.send().await.unwrap().get_receipt().await.unwrap();
    println!("{}", "-".repeat(50));

    let balance = contract.balanceOf(Address::from_str(&core).unwrap()).call().await.unwrap();
    println!("After core: {}", balance._0);

    let balance = contract.balanceOf(Address::from_str(TEST_WALLETS[0].0).unwrap()).call().await.unwrap();
    println!("After sender: {}", balance._0);

    let balance = contract.balanceOf(Address::from_str(TEST_WALLETS[1].0).unwrap()).call().await.unwrap();
    println!("After recipient: {}", balance._0);

    assert!(true)
}