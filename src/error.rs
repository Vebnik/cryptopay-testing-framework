use ethers::{
    middleware::SignerMiddleware, providers::Provider, providers::Ws, signers::LocalWallet,
};
use std::sync::Arc;

#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Anyhow errors.
    #[error("anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),

    /// SQLX errors.
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),

    /// Migration errors.
    #[error("migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    /// Request errors
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// Wallet errors.
    #[error("wallet error: {0}")]
    Wallet(#[from] alloy::signers::wallet::WalletError),

    /// Ethers wallet errors.
    #[error("ethers wallet error: {0}")]
    EthersWallet(#[from] ethers::signers::WalletError),

    /// Transport layer errors.
    #[error("transport error: {0}")]
    Transport(#[from] alloy::transports::RpcError<alloy::transports::TransportErrorKind>),

    /// Contract errors.
    #[error("contract error: {0}")]
    Contract(#[from] alloy::contract::Error),

    /// Ethers contract errors.
    #[error("ethers contract error: {0}")]
    EthersContract(
        #[from] ethers::contract::ContractError<SignerMiddleware<Arc<Provider<Ws>>, LocalWallet>>,
    ),

    /// Hex errors.
    #[error("hex error: {0}")]
    Hex(#[from] alloy::hex::FromHexError),

    /// Ethers hex errors.
    #[error("ethers hex error")]
    EthersHex,

    /// Password hashing error.
    #[error("error hashing password with argon2")]
    PasswordHash,
}

/// A convenience type alias for `Result<T, Error>`.
pub type Result<T, E = Error> = anyhow::Result<T, E>;
