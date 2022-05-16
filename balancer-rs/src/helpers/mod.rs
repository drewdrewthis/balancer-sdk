// use crate::hex;
// use ethabi::Token;
use ethcontract_common::{abi::Token, TransactionHash};
use std::convert::TryInto;
pub use std::str::FromStr;
use web3::types::{Address, H160, H256, U256};

// pub use crate::generated_test_contract_fixed::SimpleTestContract;
// use crate::infura;
pub use hex;

pub use ethcontract::tokens::{Bytes, Tokenize};

#[cfg(test)]
mod tests {
  #[test]
  fn test_use_simple_contract() {
    use super::*;

    const VAULT_CONTRACT_ADDRESS: &'static str = "0xBA12222222228d8Ba445958a75a0704d566BF2C8";

    // let web3 = crate::infura::build_web3();
    let rpc_endpoint = crate::infura::build_url();
    let transport = ethcontract::web3::transports::Http::new(&rpc_endpoint).unwrap();
    let web3 = ethcontract::Web3::new(transport);

    let vault_address =
      ethcontract::Address::from_str(&VAULT_CONTRACT_ADDRESS.to_string()).unwrap();

    let instance =
      crate::generated_test_contract_fixed::SimpleTestContract::at(&web3, vault_address);
    // let pool_id = ethcontract::Bytes::from(
    //   "0x01abc00e86c7e258823b9a055fd62ca6cf61a16300010000000000000000003b".as_u128(),
    // );

    let pool_id = "0x01abc00e86c7e258823b9a055fd62ca6cf61a16300010000000000000000003b";

    let bytes = Token::FixedBytes(hex::encode(pool_id).into());

    let param = ethcontract::tokens::Bytes::from_token(bytes).unwrap();

    instance.get_pool(param).call();
  }
}
