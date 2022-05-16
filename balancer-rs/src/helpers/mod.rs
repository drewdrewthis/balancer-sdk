pub use std::str::FromStr;

extern crate hexutil;

pub use hex;

pub use ethcontract::tokens::{Bytes, Tokenize};

#[cfg(test)]
mod tests {
  // #[test]
  #[actix_rt::test]
  async fn test_use_simple_contract() {
    use super::*;

    const VAULT_CONTRACT_ADDRESS: &'static str = "0xBA12222222228d8Ba445958a75a0704d566BF2C8";

    let rpc_endpoint = crate::infura::build_url();
    let transport = ethcontract::web3::transports::Http::new(&rpc_endpoint).unwrap();
    let web3 = ethcontract::Web3::new(transport);

    let vault_address =
      ethcontract::Address::from_str(&VAULT_CONTRACT_ADDRESS.to_string()).unwrap();

    let instance =
      crate::generated_test_contract_fixed::SimpleTestContract::at(&web3, vault_address);

    let pool_id =
      hexutil::read_hex("0x32296969ef14eb0c6d29669c550d4a0449130230000200000000000000000080")
        .unwrap();

    let bytes = ethcontract_common::abi::Token::FixedBytes(pool_id);

    let param = ethcontract::tokens::Bytes::from_token(bytes).unwrap();

    let result = instance.get_pool(param);

    // Running this will show that it will actually call and get the data from the blockchain
    // println!("Address found: {:#?}", result.clone().call().await.unwrap());

    assert_eq!(
      result.m.tx.data.unwrap(),
      ethcontract::web3::types::Bytes(
        hexutil::read_hex("0x32296969ef14eb0c6d29669c550d4a0449130230000200000000000000000080")
          .unwrap()
      )
    );
  }
}
