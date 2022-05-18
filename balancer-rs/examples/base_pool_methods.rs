/**
 * This is a collection of examples for interacting with each of the Base Vault API methods.
 * Each method is run in main() below -- you can comment out whichever example you don't want
 * to run.
 *
 * The examples use the RPC_URL constant, but you can/should replace that with your own.
 *
 * Since the Base Pool contract is inherited by all of the other pools, we can use any pool interface.
 * In these examples, we use the WeightedPool
 */
extern crate balancer_rs;
mod helpers;

use balancer_rs::generated_contracts::weighted_pool::WeightedPool;
use balancer_rs::helpers::*;
use balancer_rs::weighted_pool;
use helpers::*;
use std::str::FromStr;

// HELPERS

// Helper to get the active instance that will interact with the ethereum node.
// You can replace the RPC_URL with whatever is your prefered rpc endpoint.
const RPC_URL: &'static str = "https://rpc.flashbots.net/";
const POOL_ADDRESS: &'static str = "0x01abc00e86c7e258823b9a055fd62ca6cf61a163";
fn get_pool_instance() -> WeightedPool {
  let pool_address = addr!("0x01abc00e86c7e258823b9a055fd62ca6cf61a163");
  return weighted_pool::get_contract_instance(RPC_URL, pool_address);
}

// BASE POOL API EXAMPLES
async fn get_vault() {
  print_start_new_example("BasePool#getPoolId");

  let instance = get_pool_instance();
  let vault_address = instance.get_vault().call().await.unwrap();

  println!(
    "Balancer Pool Vault Address {:#?}",
    bytes32_to_string(vault_address),
    POOL_ADDRESS
  );
}

async fn get_pool_id() {
  print_start_new_example("BasePool#getPoolId");

  let instance = get_pool_instance();
  let authorizer = instance.get_pool_id().call().await.unwrap();

  println!(
    "Balancer Pool Id {:#?} for pool with address {:#?}",
    bytes32_to_string(authorizer),
    POOL_ADDRESS
  );
}

/**
 * All methods for the Vault API are supported and type secure.
 */
#[tokio::main]
async fn main() {
  get_vault().await;
  get_pool_id().await;
}
