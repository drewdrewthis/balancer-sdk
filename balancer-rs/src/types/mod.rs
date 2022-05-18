use ethcontract::tokens::{Bytes, Tokenize};
use ethcontract_common::abi::Token::FixedBytes;

pub struct IERC20(ethcontract::Address);
pub struct Bytes32(ethcontract::Bytes<[u8; 32]>);
pub struct Address(ethcontract::Address);

pub mod swap_request;
pub use swap_request::SwapRequest;

pub struct HexString(pub &'static str);
impl HexString {
  pub fn to_bytes32(&self) -> Bytes32 {
    let hex_string = hexutil::read_hex(&self.0);
    let bytes = FixedBytes(hex_string.unwrap());
    let result = Bytes::from_token(bytes).unwrap();

    return Bytes32(result);
  }
}

pub enum SwapKind {
  GivenIn(u8),
  GivenOut(u8),
}

#[cfg(test)]
pub mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test_hex_string_to_bytes32() {
    let pool_id = "0x32296969ef14eb0c6d29669c550d4a0449130230000200000000000000000080";
    let data = HexString(pool_id).to_bytes32();

    assert_eq!(
      Bytes([
        50, 41, 105, 105, 239, 20, 235, 12, 109, 41, 102, 156, 85, 13, 74, 4, 73, 19, 2, 48, 0, 2,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 128,
      ]),
      data
    );
  }
}
