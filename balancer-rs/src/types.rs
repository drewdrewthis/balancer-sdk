use ethcontract::tokens::{Bytes, Tokenize};
use ethcontract_common::abi::Token::FixedBytes;

pub struct HexString(pub &'static str);
impl HexString {
  pub fn to_bytes32(&self) -> ethcontract::Bytes<[u8; 32]> {
    let hex_string = hexutil::read_hex(&self.0);
    let bytes = FixedBytes(hex_string.unwrap());
    return Bytes::from_token(bytes).unwrap();
  }
}
