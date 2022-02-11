import { BatchSwap, SwapType } from '../types';
import { Contract, PopulatedTransaction } from '@ethersproject/contracts';
import { queryBatchSwap } from '../queryBatchSwap';

interface SimpleFlashSwapParameters {
    vaultContract: Contract;
    poolIds: [string, string];
    assets: BatchSwap['assets'];
    flashLoanAmount: string;
}

const createSwaps = (
    poolIds: SimpleFlashSwapParameters['poolIds'],
    amount: string
) => {
    return [
        {
            poolId: poolIds[0],
            assetInIndex: 0,
            assetOutIndex: 1,
            amount,
            userData: '0x',
        },
        {
            poolId: poolIds[1],
            assetInIndex: 1,
            assetOutIndex: 0,
            amount: '0',
            userData: '0x',
        },
    ];
};

const createArguments = async ({
    vaultContract,
    poolIds,
    assets,
    flashLoanAmount,
}: SimpleFlashSwapParameters) => {
    const swaps = createSwaps(poolIds, flashLoanAmount);

    const funds = {
        sender: await vaultContract.signer.getAddress(),
        fromInternalBalance: false,
        recipient: await vaultContract.signer.getAddress(),
        toInternalBalance: false,
    };

    const limits = ['0', '0'];

    const deadline = '999999999999999999';

    return {
        kind: SwapType.SwapExactIn,
        swaps,
        assets,
        funds,
        limits,
        deadline,
    };
};

/**
 * Simple interface to execute a simple flashSwap with the Balancer Vault.
 *
 * A "simple" flash swap is an arbitrage executed with only two tokens and two pools,
 * swapping in the first pool and then back in the second pool for a profit. For more
 * complex flash swaps, you will have to use the batch swap method.
 *
 * Learn more: A [Flash Swap](https://dev.balancer.fi/resources/swaps/flash-swaps).
 *
 * @param {SimpleFlashSwapParameters}   params - BatchSwap information used for query.
 * @param {Contract}                    params.vaultContract - the ethersjs contract for the Balancer Vault. Must have a Signer.
 * @param {string}                      params.flashLoanAmount - initial input amount for the flash loan (first asset)
 * @param {string[]}                    params.poolIds - array of Balancer pool ids
 * @param {string[]}                    params.assets - array of token addresses
 * @returns {TransactionResponse}       Returns an ethersjs transaction response
 */
export async function simpleFlashSwap(
    params: SimpleFlashSwapParameters
): Promise<PopulatedTransaction> {
    const args = await createArguments(params);

    return params.vaultContract.batchSwap(
        args.kind,
        args.swaps,
        args.assets,
        args.funds,
        args.limits,
        args.deadline
    );
}

/**
 * Simple interface to test if a simple flash swap is valid and see potential profits.
 *
 * A "simple" flash swap is an arbitrage executed with only two tokens and two pools,
 * swapping in the first pool and then back in the second pool for a profit. For more
 * complex flash swaps, you will have to use the batch swap method.
 *
 * Learn more: A [Flash Swap](https://dev.balancer.fi/resources/swaps/flash-swaps).
 *
 * _NB: This method doesn't execute a flashSwap
 *
 * @param {SimpleFlashSwapParameters}   params - BatchSwap information used for query.
 * @param {Contract}                    params.vaultContract - the ethersjs contract for the Balancer Vault.
 * @param {string}                      params.flashLoanAmount - initial input amount for the flash loan (first asset)
 * @param {string[]}                    params.poolIds - array of Balancer pool ids
 * @param {string[]}                    params.assets - array of token addresses
 * @returns {Promise<{profits: Record<string, string>, isProfitable: boolean}>}       Returns an ethersjs transaction response
 */
export async function querySimpleFlashSwap(
    params: SimpleFlashSwapParameters
): Promise<{ profits: Record<string, string>; isValid: boolean }> {
    const args = await createArguments(params);
    const [tokenAddress1, tokenAddress2] = params.assets;

    try {
        const response = await queryBatchSwap(
            params.vaultContract,
            args.kind,
            args.swaps,
            args.assets
        );

        const profits = {
            [tokenAddress1]: String(Number(response[0]) * -1),
            [tokenAddress2]: String(Number(response[1]) * -1),
        };

        return {
            profits,
            isValid:
                Number(profits[tokenAddress1]) > 0 &&
                Number(profits[tokenAddress2]) > 0,
        };
    } catch (err) {
        throw `Failed to querySimpleFlashSwap: ${err}`;
    }
}
