pub use ethcontract::tokens::{Bytes, Tokenize};
pub use ethcontract::U256;
pub use ethcontract_common::abi::Token::FixedBytes;
pub use std::str::FromStr;

#[macro_export]
macro_rules! addr {
  ($address: expr) => {
    ethcontract::Address::from_str($address).unwrap()
  };
}

#[macro_export]
macro_rules! u256 {
  ($string: expr) => {{
    {
      U256::from_dec_str($string).unwrap()
    }
  }};
}
