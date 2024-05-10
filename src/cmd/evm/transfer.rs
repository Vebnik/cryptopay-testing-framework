use alloy::{primitives, sol};
use colored::Colorize;
use ethers::{
    contract::abigen,
    middleware::SignerMiddleware,
    providers::{Provider, Ws},
    signers::{LocalWallet, Signer},
    types,
};
use std::{error::Error, str::FromStr, sync::Arc};

use crate::config::{Config, ProviderType};

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20,
    "contracts/artifacts/ERC20.json"
}

abigen!(
    IERC20,
    r#"[
        function totalSupply() external view returns (uint256)
        function balanceOf(address account) external view returns (uint256)
        function transfer(address recipient, uint256 amount) external returns (bool)
        function allowance(address owner, address spender) external view returns (uint256)
        function approve(address spender, uint256 amount) external returns (bool)
        function transferFrom( address sender, address recipient, uint256 amount) external returns (bool)
        event Transfer(address indexed from, address indexed to, uint256 value)
        event Approval(address indexed owner, address indexed spender, uint256 value)
    ]"#,
);

pub async fn exec(
    _config: Arc<Config>,
    recipient: String,
    sender: String,
    contract: String,
    amount: u32,
    provider: ProviderType,
) -> Result<(), Box<dyn Error>> {
    let recipient = primitives::Address::from_str(&recipient)?;
    let contract = primitives::Address::from_str(&contract)?;
    let sender = primitives::Address::from_str(&sender)?;

    let decimals = primitives::U256::from(10)
        .checked_pow(primitives::U256::from(18))
        .unwrap();
    let amount = primitives::U256::from(amount)
        .checked_mul(decimals)
        .unwrap();

    let contract = ERC20::new(contract, provider);

    println!("{}", amount);

    let receipt = contract
        .transfer(recipient, amount)
        .send()
        .await?
        .get_receipt()
        .await?;

    println!(
        "{} Send transaction: {}",
        "[EVM]".blue(),
        receipt.transaction_hash
    );

    println!(
        "{} Transfer {amount} from {} to {}",
        "[EVM]".blue(),
        sender.clone(),
        recipient.clone()
    );

    Ok(())
}

pub async fn exec_ethers(
    _config: Arc<Config>,
    recipient: String,
    sender_key: String,
    contract_addr: String,
    amount: u32,
    provider: Provider<Ws>,
) -> Result<(), Box<dyn Error>> {
    let contract_addr = contract_addr.parse::<types::Address>().unwrap();
    let wallet = sender_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(31337u64);
    let recipient_addr = recipient.parse::<types::Address>().unwrap();

    let decimals = types::U256::from(10)
        .checked_pow(types::U256::from(18))
        .unwrap();
    let amount = types::U256::from(amount).checked_mul(decimals).unwrap();

    let client = SignerMiddleware::new(Arc::new(provider), wallet.clone());
    let contract = IERC20::new(contract_addr, Arc::new(&client));

    contract
        .transfer(recipient_addr, types::U256::from(amount))
        .send()
        .await
        .unwrap();

    println!("{} Send transaction", "[EVM]".blue(),);

    println!(
        "{} Transfer {amount} from {} to {}",
        "[EVM]".blue(),
        wallet.address(),
        recipient.clone()
    );

    Ok(())
}
