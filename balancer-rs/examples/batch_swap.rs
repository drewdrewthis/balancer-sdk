//! # Example: Batch Swap
//! Executes a batch swap with a single swap step.
//!
//! ## Gotchas
//! - If you don't provide gas, gas_price, or nonce, the internal [`TransactionBuilder`] will have to make a call to the node to get those values.
//! - If the transaction will fail, then getting these values will fail as well, even on the `build` step.
//! - If you haven't approved the tokens for spending, it will fail.
//!
//! Successful batch swap transaction on Kovan: `https://kovan.etherscan.io/tx/0x2f7603dc9dbc0ae406bdfd95abe06d3d90152d329fad4faf1021954978468993`
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
use ethcontract::I256;
use ethcontract::U256;
use helpers::*;
use std::str::FromStr;

const RECIPIENT_WALLET_ADDRESS: &str = "0x35f5a330FD2F8e521ebd259FA272bA8069590741";
const SENDER_WALLET_ADDRESS: &str = "0x35f5a330FD2F8e521ebd259FA272bA8069590741";

#[tokio::main]
async fn main() {
    print_start_new_example("Vault#batchSwap");

    let instance = build_vault_instance();

    let assets = vec![
        addr!(sample_data::kovan::USDC_ADDRESS),
        addr!(sample_data::kovan::DAI_ADDRESS),
    ];

    let limits = vec![i256!("1000000000000000000"), i256!("1000000000000000000")];

    let swap_step = BatchSwapStep {
        pool_id: pool_id!(sample_data::kovan::POOLS[1].id),
        asset_out_index: 0,
        asset_in_index: 1,
        amount: u256!("10"),
        user_data: UserData("0x"),
    };

    let funds = FundManagement {
        sender: addr!(SENDER_WALLET_ADDRESS),
        from_internal_balance: false,
        recipient: addr!(RECIPIENT_WALLET_ADDRESS),
        to_internal_balance: false,
    };

    let private_key = get_env_var("PRIVATE_KEY");
    let private_key_secure = PrivateKey::from_str(&private_key).unwrap();

    let result = match instance
        .batch_swap(
            SwapKind::GivenIn as u8,
            vec![swap_step.clone().into()],
            assets,
            funds.into(),
            limits,
            // Infinity
            u256!("999999999999999999"),
        )
        .from(Account::Offline(private_key_secure, None))
        .gas(u256!(4_712_388))
        .gas_price(u256!("100000000000").into())
        // .confirmations(1)
        .send()
        .await
    {
        Ok(any) => any,
        Err(e) => {
            println!("Failed to execute batch swap");
            println!("{:#?}", e);
            return;
        }
    };

    println!("Batch swap result {:#?} for swap {:#?}", result, swap_step);
}
