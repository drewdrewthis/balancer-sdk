/**
 * This is a collection of examples for interacting with each of the Vault API methods.
 * Each method is run in main() below -- you can comment out whichever example you don't want
 * to run.
 *
 * The examples use the RPC_URL constant, but you can/should replace that with your own.
 */
extern crate balancer_rs;
mod helpers;
mod sample_data;

use balancer_rs::helpers::errors::handle_bal_error;
use balancer_rs::helpers::get_env_var;
use balancer_rs::vault::Vault;
use balancer_rs::*;
use ethcontract::Account;
use ethcontract::Address;
use ethcontract::PrivateKey;
use ethcontract::I256;
use ethcontract::U256;
use helpers::*;
use std::str::FromStr;

const RECIPIENT_WALLET_ADDRESS: &str = "0x35f5a330FD2F8e521ebd259FA272bA8069590741";
const SENDER_WALLET_ADDRESS: &str = "0x35f5a330FD2F8e521ebd259FA272bA8069590741";

// HELPERS

// Helper to get the active instance that will interact with the ethereum node.
// You can replace the RPC_URL with whatever is your prefered rpc endpoint.
fn get_vault_instance() -> Vault {
    let rpc_url: String = get_env_var("RPC_URL");
    let transport = ethcontract::web3::transports::Http::new(&rpc_url).unwrap();
    let web3 = ethcontract::Web3::new(transport);

    Vault::new(web3)
}

#[allow(dead_code)]
async fn query_batch_swap() {
    print_start_new_example("Vault#queryBatchSwap");

    let instance = get_vault_instance();

    let assets = vec![
        addr!(sample_data::kovan::USDC_ADDRESS),
        addr!(sample_data::kovan::DAI_ADDRESS),
    ];

    let swap_step = BatchSwapStep::new(
        PoolId(sample_data::kovan::POOLS[1].id),
        0,
        1,
        "10",
        UserData("0x"),
    );

    let funds = FundManagement {
        sender: addr!(SENDER_WALLET_ADDRESS),
        from_internal_balance: false,
        recipient: addr!(RECIPIENT_WALLET_ADDRESS),
        to_internal_balance: false,
    };

    let deltas = match instance
        .query_batch_swap(
            SwapKind::GivenIn.into(),
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

/// Executes a batch swap with a single swap step.
///
/// ## Gotchas
/// - If you don't provide gas, gas_price, or nonce, the interal [`TransactionBuilder`] will have to make a call to the node to get those values.
/// - If the transaction will fail, then getting these values will fail as well, even on the `build` step.
///
pub async fn batch_swap() {
    print_start_new_example("Vault#batchSwap");

    let instance = get_vault_instance();

    let assets = vec![
        addr!(sample_data::kovan::USDC_ADDRESS),
        addr!(sample_data::kovan::DAI_ADDRESS),
    ];

    let limits = vec![i256!("100"), i256!("-9125892514880")];

    println!("Limits: {:#?}", limits);
    println!("Assets: {:#?}", assets);

    let swap_step = BatchSwapStep::new(
        PoolId(sample_data::kovan::POOLS[1].id),
        0,
        1,
        "10",
        UserData("0x"),
    );

    let funds = FundManagement {
        sender: addr!(SENDER_WALLET_ADDRESS),
        from_internal_balance: false,
        recipient: addr!(RECIPIENT_WALLET_ADDRESS),
        to_internal_balance: false,
    };

    let private_key = get_env_var("PRIVATE_KEY");
    let private_key_secure = PrivateKey::from_str(&private_key).unwrap();

    println!(
        "private key {:?} for {}",
        private_key_secure.clone(),
        private_key
    );

    let tx = instance
        .batch_swap(
            SwapKind::GivenIn.into(),
            vec![swap_step.clone().into()],
            assets,
            funds.into(),
            limits,
            // Infinity
            u256!("999999999999999999"),
        )
        .from(Account::Offline(private_key_secure, Some(42)))
        .gas(4_712_388.into())
        .gas_price(u256!("100000000000").into())
        .into_inner();

    let data = match tx
        // .from(Account::Offline(private_key_secure, Some(42)))
        // .gas(4_712_388.into())
        // .gas_price(u256!("100000000000").into())
        // .nonce(u256!("4"))
        .clone()
        .build()
        .await
    {
        Ok(any) => any,
        Err(e) => {
            println!("Failed to build transaction: {}", e);
            // handle_bal_error(&e);
            return;
        }
    };

    let rpc_url: String = get_env_var("RPC_URL");
    let transport = ethcontract::web3::transports::Http::new(&rpc_url).unwrap();
    let web3 = ethcontract::Web3::new(transport);

    let result = web3
        .eth()
        .send_raw_transaction(data.raw().unwrap())
        .await
        .unwrap();

    println!("Batch swap result {:#?} for swap {:#?}", result, swap_step);
}

#[allow(dead_code)]
async fn weth() {
    print_start_new_example("Vault#WETH");

    let instance = get_vault_instance();
    let weth_address = instance.weth().call().await.expect("Failed to get WETH");

    println!("Balancer Vault WETH address {:#?}", weth_address);
}

/**
 * All methods for the Vault API are supported and type secure.
 */
#[tokio::main]
async fn main() {
    query_batch_swap().await;
    batch_swap().await;
}
