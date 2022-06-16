//! # Examples: Single Swap
//!
//! Executes a single swap of USDC for DAI via the Vault for a particular pool.
//!
//! > **Note**
//! > You must approve the tokens for spending before you can execute a swap.
//!
//! Successful swap transaction: `https://kovan.etherscan.io/tx/0xfcb6d38c73841f37bd4bf5d0e1245822a8c2457877cf071390d04fce336ce7d5`
//!
//! ## ENV VARIABLES
//! The example uses the following environment variables from a .env file:
//! - `RPC_URL`
//! - `PRIVATE_KEY`

extern crate balancer_sdk;
mod helpers;
mod sample_data;

use balancer_sdk::helpers::*;
use balancer_sdk::*;
use ethcontract::Account;
use ethcontract::Address;
use ethcontract::PrivateKey;
use ethcontract::U256;
use helpers::*;
use std::str::FromStr;

const RECIPIENT_WALLET_ADDRESS: &str = "0x35f5a330FD2F8e521ebd259FA272bA8069590741";
const SENDER_WALLET_ADDRESS: &str = "0x35f5a330FD2F8e521ebd259FA272bA8069590741";

#[tokio::main]
async fn main() {
    print_start_new_example("Vault#singleSwap");

    let instance = build_vault_instance();

    let swap_step = SingleSwap {
        pool_id: pool_id!(sample_data::kovan::POOLS[1].id),
        kind: SwapKind::GivenIn,
        asset_in: addr!(sample_data::kovan::USDC_ADDRESS),
        asset_out: addr!(sample_data::kovan::DAI_ADDRESS),
        amount: u256!("10"),
        user_data: UserData("0x").into(),
    };

    let funds = FundManagement {
        sender: addr!(SENDER_WALLET_ADDRESS),
        from_internal_balance: false,
        recipient: addr!(RECIPIENT_WALLET_ADDRESS),
        to_internal_balance: false,
    };

    let limit = u256!("10000000000000");

    let deadline = u256!("999999999999999999");

    let private_key = PrivateKey::from_str(&get_env_var("PRIVATE_KEY")).unwrap();

    let result = match instance
        .swap(swap_step.clone().into(), funds.into(), limit, deadline)
        .from(Account::Offline(private_key, Some(42)))
        .gas(u256!("4712388"))
        .gas_price(u256!("100000000000").into())
        .send()
        .await
    {
        Ok(any) => any,
        Err(e) => {
            println!("Failed to build transaction. Please make sure that you've approved the required tokens for spending. See the `check_allowance` example.");
            println!("{:#?}", e);
            return;
        }
    };

    println!("Batch swap result {:#?} for swap {:#?}", result, swap_step);
}
