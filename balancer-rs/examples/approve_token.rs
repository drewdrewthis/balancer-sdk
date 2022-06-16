//! # Example: Approve a token for spending
//!
//! ## ENV VARIABLES
//! The example uses the following environment variables from a .env file:
//! - `RPC_URL`
//! - `PRIVATE_KEY`

extern crate balancer_sdk;

mod helpers;
mod sample_data;

use std::str::FromStr;

use balancer_sdk::{
    helpers::{build_web3, get_env_var, TokenApprover},
    *,
};
use ethcontract::PrivateKey;
use helpers::*;

pub async fn approve_token() {
    print_start_new_example("Set approval on token for spender (set allowance for Balancer Vault)");

    let rpc_url: String = get_env_var("RPC_URL");
    let web3 = build_web3(&rpc_url);

    let token = addr!(sample_data::kovan::USDC_ADDRESS);
    let private_key = PrivateKey::from_str(&get_env_var("PRIVATE_KEY")).unwrap();
    let token_approver =
        TokenApprover::new(web3, addr!(vault::VAULT_CONTRACT_ADDRESS), private_key);

    let result = token_approver
        .approve(token, u256!("1000000000000000000"))
        .await
        .unwrap();

    println!("Approval result: {:#?}", result);
}

#[tokio::main]
async fn main() {
    approve_token().await
}
