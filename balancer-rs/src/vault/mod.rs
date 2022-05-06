use dotenv::dotenv;
use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::types::{Address, U256};

pub fn init() {
  println!("Vault module successfully initialized");
}

#[allow(dead_code)]
pub fn get_number() -> i32 {
  return 1;
}

#[allow(dead_code)]
pub fn weth() -> String {
  dotenv().ok();

  let mut rpc_endpoint = "".to_owned();
  const INFURA_HOST: &str = "https://mainnet.infura.io/v3/";
  rpc_endpoint.push_str(INFURA_HOST);
  rpc_endpoint.push_str(&env::var("INFURA_PROJECT_ID").expect("Env key not present"));

  return rpc_endpoint.to_string();

  // let transport = web3::transports::Http::new(&rpc_endpoint)?;
  // let web3 = web3::Web3::new(transport);

  // let vault_address = Address::from_str("0x42447d5f59d5bf78a82c34663474922bdf278162").unwrap();

  // let token_contract =
  //   Contract::from_json(web3s.eth(), aave_addr, include_bytes!("erc20_abi.json")).unwrap();
  // let token_name: String = token_contract
  //   .query("name", (), None, Options::default(), None)
  //   .await
  //   .unwrap();
  // let total_supply: U256 = token_contract
  //   .query("totalSupply", (), None, Options::default(), None)
  //   .await
  //   .unwrap();
  // println!("Token name: {}, total supply: {}", token_name, total_supply);
  // return ("test").to_string();
}
