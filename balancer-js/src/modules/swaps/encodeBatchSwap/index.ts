import { BatchSwap } from '../types';

/**
 * This method provides a wrapper around the the Balancer Vault [method for a batchSwap](https://dev.balancer.fi/references/contracts/apis/the-vault#batch-swaps).
 * 
 * _NB: This method doesn't execute a batchSwap -- it returns an [ABI byte string](https://docs.soliditylang.org/en/latest/abi-spec.html) 
 * containing the data of the function call on a contract, which can then be sent to the network to be executed.
 * (ex. [sendTransaction](https://web3js.readthedocs.io/en/v1.2.11/web3-eth.html#sendtransaction)).
 * 
  @param {BatchSwap}          batchSwap - BatchSwap information used for query.
  @param {SwapType}           batchSwap.kind - either exactIn or exactOut.
  @param {BatchSwapSteps[]}   batchSwap.swaps - sequence of swaps.
  @param {string[]}           batchSwap.assets - array contains the addresses of all assets involved in the swaps.
  @returns {string}           encodedBatchSwapData - Returns an ABI byte string containing the data of the function call on a contract.
*/
export function encodeBatchSwap({
    kind,
    swaps,
    assets,
    funds,
    limits,
    deadline,
}: BatchSwap): string {}
