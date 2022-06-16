#![doc = include_str!("../README.md")]

mod enums;
mod generated_contracts;
mod macros;
mod structs;
mod types;

pub use enums::*;
/// A generic contract for an ERC20 token
pub use generated_contracts::erc20::ERC20;
pub use structs::*;
pub use types::*;

pub mod constants;
pub mod helpers;
pub mod pools;
pub mod vault;

pub use ethcontract::{Account, Address, PrivateKey, I256, U256};

pub use std::str::FromStr;
