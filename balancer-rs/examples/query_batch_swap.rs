//! # Examples: query_batch_swap
//!
//! This is a collection of examples for the different types of swaps available through Balancer's protocol.
//! It uses the following env variables from a .env file:
//!
//! > **Note**
//! > You must approve the tokens for spending before you can execute a swap.
//!
//! ## ENV VARIABLES
//! The example uses the following environment variables from a .env file:
//! - `RPC_URL`

extern crate balancer_sdk;
mod helpers;
mod sample_data;

use balancer_sdk::helpers::errors::handle_bal_error;
use balancer_sdk::helpers::*;
use balancer_sdk::*;
use ethcontract::{Address, U256};
use helpers::*;
use std::str::FromStr;

const RECIPIENT_WALLET_ADDRESS: &str = "0x35f5a330FD2F8e521ebd259FA272bA8069590741";
const SENDER_WALLET_ADDRESS: &str = "0x35f5a330FD2F8e521ebd259FA272bA8069590741";

#[tokio::main]
async fn main() {
    print_start_new_example("Vault#queryBatchSwap");

    let instance = build_vault_instance();

    let assets = vec![
        addr!(sample_data::kovan::USDC_ADDRESS),
        addr!(sample_data::kovan::DAI_ADDRESS),
    ];

    let swap_step = BatchSwapStep {
        pool_id: sample_data::kovan::POOLS[1].id.parse().unwrap(),
        asset_in_index: 0,
        asset_out_index: 1,
        amount: u256!(10),
        user_data: UserData("0x"),
    };

    let funds = FundManagement {
        sender: addr!(SENDER_WALLET_ADDRESS),
        from_internal_balance: false,
        recipient: addr!(RECIPIENT_WALLET_ADDRESS),
        to_internal_balance: false,
    };

    let deltas = match instance
        .query_batch_swap(
            SwapKind::GivenIn as u8,
            vec![swap_step.clone().into()],
            assets,
            funds.into(),
        )
        .call()
        .await
    {
        Ok(any) => any,
        Err(e) => {
            println!("Failed to query: {}", e);
            handle_bal_error(&e);
            return;
        }
    };

    println!("Asset deltas for {:#?} are {:#?}", swap_step, deltas);
}
