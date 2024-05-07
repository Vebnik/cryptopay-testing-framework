use std::{cell::RefCell, default::Default};

use crate::cli::Args;

pub type ProviderType = FillProvider<
    JoinFill<
        JoinFill<JoinFill<JoinFill<Identity, GasFiller>, NonceFiller>, ChainIdFiller>,
        SignerFiller<EthereumSigner>,
    >,
    RootProvider<BoxTransport>,
    BoxTransport,
    Ethereum,
>;

use alloy::{
    network::{Ethereum, EthereumSigner},
    providers::{
        fillers::{ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, SignerFiller},
        Identity, RootProvider,
    },
    transports::BoxTransport,
};
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct Config {
    pub anvil_endpoint: String,
    pub cryptopay_url: String,
    pub db_host: String,
    pub db_connect_url: String,
    pub core_priv_key: String,
    pub anvil_nodes: i8,

    pub test_priv_key: String,
    pub test_address: String,
}

#[derive(Debug)]
pub struct State {
    pub config: Config,
    pub provider: ProviderType,
    pub db: Pool<Postgres>,
    pub args: Args,
    pub system_user_token: RefCell<Option<String>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cryptopay_url: "http://127.0.0.1:9999".into(),
            anvil_endpoint: "http://127.0.0.1:8545".into(),
            db_host: "127.0.0.1:5432".into(),
            db_connect_url: "postgres://postgres:postgres@localhost:5432/test".into(),
            core_priv_key: "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
                .into(),
            anvil_nodes: 2,

            test_priv_key: "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d"
                .into(),
            test_address: "0x70997970C51812dc3A010C7d01b50e0d17dc79C8".into(),
        }
    }
}

// impl Config {
//     fn new(
//         cryptopay_url: String,
//         anvil_endpoint: String,
//         db_host: String,
//         db_connect_url: String,
//         core_priv_key: String,
//     ) -> Self {
//         todo!("Implement me!")
//     }
// }

impl State {
    pub fn set_user_token(self, token: String) {
        *self.system_user_token.borrow_mut() = Some(token)
    }
}
