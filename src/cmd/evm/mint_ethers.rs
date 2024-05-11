// use alloy::{
//     network::EthereumSigner,
//     primitives::{Address, U256},
//     providers::ProviderBuilder,
//     signers::wallet::LocalWallet,
//     sol,
// };
use colored::Colorize;
use ethers::{
    contract::abigen,
    middleware::SignerMiddleware,
    providers::{Provider, Ws},
    signers::{LocalWallet, Signer},
    types::{Address, U256},
};
use std::{str::FromStr, sync::Arc};

use crate::{Config, Error, Result};

// sol! {
//     #[allow(missing_docs)]
//     #[sol(rpc)]
//     ERC20,
//     "contracts/artifacts/ERC20.json"
// }

abigen!(
    IERC20,
    r#"[
        function totalSupply() external view returns (uint256)
        function balanceOf(address account) external view returns (uint256)
        function transfer(address recipient, uint256 amount) external returns (bool)
        function allowance(address owner, address spender) external view returns (uint256)
        function approve(address spender, uint256 amount) external returns (bool)
        function transferFrom(address sender, address recipient, uint256 amount) external returns (bool)
        function mint(address recipient, uint256 amount) external returns (bool)
        event Transfer(address indexed from, address indexed to, uint256 value)
        event Approval(address indexed owner, address indexed spender, uint256 value)
    ]"#,
);

pub async fn exec(
    config: Arc<Config>,
    address: String,
    contract: String,
    amount: u32,
) -> Result<()> {
    let wallet = config
        .core_key
        .parse::<LocalWallet>()?
        .with_chain_id(31337u64);
    let contract_addr = Address::from_str(&contract).map_err(|_| Error::EthersHex)?;
    let target_addr = Address::from_str(&address).map_err(|_| Error::EthersHex)?;
    let decimals = U256::from(10).checked_pow(U256::from(18)).unwrap();

    for port in 8545..(8545 + config.anvil_nodes as i32) {
        let provider = Provider::<Ws>::connect(format!("ws://127.0.0.1:{port}"))
            .await
            .unwrap();
        let client = Arc::new(SignerMiddleware::new(Arc::new(provider), wallet.clone()));
        // let provide = ProviderBuilder::new()
        //     .with_recommended_fillers()
        //     .signer(EthereumSigner::from(wallet.clone()))
        //     .on_builtin(&format!("http://localhost:{}", port))
        //     .await?;

        let contract = IERC20::new(contract_addr, client);
        let amount = U256::from(amount).checked_mul(decimals).unwrap();

        let _ = contract.mint(target_addr, amount).send().await?;

        println!(
            "{} Minted {amount} at address: {:?}",
            "[EVM]".blue(),
            target_addr.clone()
        );
    }

    Ok(())
}
