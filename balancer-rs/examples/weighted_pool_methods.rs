/**
 * This is a collection of examples for interacting with each of the Weighted Pool API methods.
 * Each method is run in main() below -- you can comment out whichever example you don't want
 * to run.
 *
 * The examples use the RPC_URL constant, but you can/should replace that with your own.
 */
extern crate balancer_rs;
mod helpers;

use balancer_rs::constants::addresses::*;
use balancer_rs::generated_contracts::weighted_pool::WeightedPool;
use balancer_rs::helpers::macros::*;
use balancer_rs::types::*;
use ethcontract::U256;

// HELPERS
// Helper to get the active instance that will interact with the ethereum node.
// You can replace the RPC_URL with whatever is your prefered rpc endpoint.
const RPC_URL: &'static str = "https://rpc.flashbots.net/";
const POOL_ADDRESS: &'static str = "0x01abc00e86c7e258823b9a055fd62ca6cf61a163";
fn get_pool_instance() -> WeightedPool {
  let transport = ethcontract::web3::transports::Http::new(RPC_URL).unwrap();
  let web3 = ethcontract::Web3::new(transport);
  let pool_address = addr!(POOL_ADDRESS);

  return balancer_rs::WeightedPool::new(web3, pool_address);
}

async fn on_swap() {
  let request = SwapRequest {
    kind: SwapKind::GivenIn("GIVEN_IN"),
    token_in: addr!(UNI_ADDRESS),
    token_out: addr!(AAVE_ADDRESS),
  };
  let balanceTokenIn = U256::from_str("1234").unwrap();
  let balanceTokenOut = U256::from_str("234").unwrap();
  let pool_instance = get_pool_instance();
  let vault_address = pool_instance
    .on_swap(request, balanceTokenIn, balanceTokenOut)
    .call()
    .await
    .unwrap();
}

/**
 * All methods for the Vault API are supported and type secure.
 */
#[tokio::main]
async fn main() {
  on_swap().await;
}
