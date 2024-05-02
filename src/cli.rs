use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug, Clone)]
pub enum EvmCommands {
    /// Deploy new contract
    Deploy {
        #[arg(default_value = "Test USDT")]
        name: String,
        #[arg(default_value = "USDT")]
        symbol: String,
        #[arg(default_value = "100")]
        amount: u32,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum NetworkCommands {
    /// Create new network
    Create {
        #[arg(default_value = "Local ETH")]
        name: String,
        #[arg(default_value = "EVM")]
        kind: String,
        #[arg(default_value = "ws://127.0.0.1:8545")]
        endpoint: String,
    },
}


#[derive(Subcommand, Debug, Clone)]
pub enum ApiCommands {
    /// Create user -> login -> create wallet ->  with wallet
    UserFlow,
    /// Network api scope command
    Network {
        #[command(subcommand)]
        cmd: NetworkCommands,        
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum DbCommands {
    /// Drop all data
    Drop,
    /// Migrate all data
    Create,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ProcessType {
    /// EVM scope command
    Evm {
        #[command(subcommand)]
        cmd: EvmCommands,
    },
    /// API scope command
    Api {
        #[command(subcommand)]
        cmd: ApiCommands,
    },
    /// Datebase scope command
    Db {
        #[command(subcommand)]
        cmd: DbCommands,
    }
}

/// CLI Test framework for cryptopay
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Test process type
    #[command(subcommand)]
    pub process: ProcessType,
}
