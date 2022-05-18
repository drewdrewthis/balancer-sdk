use ethcontract::U256;

use super::*;

pub struct SwapRequest {
  pub kind: SwapKind,
  pub token_in: IERC20,
  pub token_out: IERC20,
  pub amount: U256,

  // Misc data
  pub pool_id: Bytes32,
  pub last_change_block: U256,
  pub from: Address,
  pub to: Address,
  pub user_data: ethcontract::tokens::Bytes<Vec<u8>>,
}

