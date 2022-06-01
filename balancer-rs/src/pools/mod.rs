use super::generated_contracts::liquidity_bootstrapping_pool::LiquidityBootStrappingPool as GeneratedLiquidityBootstrappingPool;
use super::generated_contracts::managed_pool::ManagedPool as GeneratedManagedPool;
use super::generated_contracts::meta_stable_pool::MetaStablePool as GeneratedMetaStablePool;
use super::generated_contracts::stable_pool::StablePool as GeneratedStablePool;
use super::generated_contracts::weighted_pool::WeightedPool as GeneratedWeightedPool;
use ethcontract::Address;

macro_rules! define_contract {
  ($name:ident, $generated_name:ident) => {
    pub struct $name {}
    impl $name {
      pub fn new(
        web3: ethcontract::Web3<ethcontract::web3::transports::Http>,
        pool_address: Address,
      ) -> $generated_name {
        return $generated_name::at(&web3, pool_address);
      }
    }
  };
}

define_contract!(WeightedPool, GeneratedWeightedPool);
define_contract!(
  LiquidityBootStrappingPool,
  GeneratedLiquidityBootstrappingPool
);

define_contract!(MetaStablePool, GeneratedMetaStablePool);
define_contract!(StablePool, GeneratedStablePool);
define_contract!(ManagedPool, GeneratedManagedPool);

// The API for the MetaStablePool will work for this contract. We don't have a JSON ABI for it for now.
define_contract!(WeightedPool2Tokens, GeneratedMetaStablePool);
