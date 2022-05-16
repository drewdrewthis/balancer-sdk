mod helpers;
use std::str;

#[tokio::main]
async fn main() {
  helpers::print_start_new_example("Vault#getPoolId");

  let pool_id =
    hexutil::read_hex("0x32296969ef14eb0c6d29669c550d4a0449130230000200000000000000000080")
      .unwrap();

  let web3 = balancer_rs::infura::build_web3();
  let vault = balancer_rs::vault::VaultService::new(web3);
  let pool_addr = vault.get_pool(hex::encode(pool_id)).await;
  let address_str = web3::helpers::to_string(&pool_addr);

  println!(
    "Balancer Pool address {} for pool id 0x01abc00e86c7e258823b9a055fd62ca6cf61a16300010000000000000000003b",
    address_str
  )
}
