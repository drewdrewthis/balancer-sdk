use dotenv;
use std::env;

pub mod errors;
pub mod tokens;

pub use tokens::*;

use crate::vault::Vault;
use crate::Web3;

/// Builds an instance of [Web3] that allows the contract instances to interact with the ethereum node.
pub fn build_web3(rpc_endpoint: &str) -> Web3 {
    let transport = ethcontract::web3::transports::Http::new(rpc_endpoint).unwrap();
    Web3::new(transport)
}

/// Loads any .env file and then check the environment variables for `key`.
/// Will panic if the `key` is not found.
pub fn get_env_var(key: &str) -> String {
    dotenv::dotenv().ok();
    env::var(key).unwrap()
}

/// Builds an instance of [Vault]
pub fn build_vault_instance() -> Vault {
    let rpc_url: String = get_env_var("RPC_URL");
    let web3 = build_web3(&rpc_url);

    Vault::new(web3)
}
