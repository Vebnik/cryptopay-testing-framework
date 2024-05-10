#[tokio::test]
pub async fn create_provider_test() {
    use alloy::sol;
    use std::sync::Arc;

    use ethers::{
        contract::abigen,
        middleware::SignerMiddleware,
        providers::{Provider, Ws},
        signers::{LocalWallet, Signer},
        types::{
            Address, U256,
        },
    };

    use crate::config::TEST_WALLETS;

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

    let contract_addr = "0x5fbdb2315678afecb367f032d93f642f64180aa3"
        .parse::<Address>()
        .unwrap();
    let wallet = TEST_WALLETS[0]
        .1
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(31337u64);
    let provider = Provider::<Ws>::connect("ws://127.0.0.1:8545")
        .await
        .unwrap();
    let client = SignerMiddleware::new(Arc::new(provider), wallet);

    let recipient = TEST_WALLETS[1].0.parse::<Address>().unwrap();

    let contract = IERC20::new(contract_addr, Arc::new(&client));

    contract
        .transfer(recipient, U256::from(10u32))
        .send()
        .await
        .unwrap();
}
