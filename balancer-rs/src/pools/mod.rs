use super::generated_contracts::meta_stable_pool::MetaStablePool as GeneratedMetaStablePool;
use super::generated_contracts::weighted_pool::WeightedPool as GeneratedWeightedPool;
use ethcontract::Address;

pub struct WeightedPool {}
impl WeightedPool {
  pub fn new(
    web3: ethcontract::Web3<ethcontract::web3::transports::Http>,
    pool_address: Address,
  ) -> GeneratedWeightedPool {
    return GeneratedWeightedPool::at(&web3, pool_address);
  }
}

trait MetaPoolType {
  fn new(
    web3: ethcontract::Web3<ethcontract::web3::transports::Http>,
    pool_address: Address,
  ) -> GeneratedMetaStablePool {
    return GeneratedMetaStablePool::at(&web3, pool_address);
  }
}

pub struct MetaStablePool {}
impl MetaPoolType for MetaStablePool {}

pub struct WeightedPool2Tokens {}
impl MetaPoolType for WeightedPool2Tokens {}

pub struct StablePool {}
impl MetaPoolType for StablePool {}
