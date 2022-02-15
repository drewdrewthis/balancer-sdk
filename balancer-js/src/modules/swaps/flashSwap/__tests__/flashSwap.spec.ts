import { Contract } from '@ethersproject/contracts';
import { assert, expect } from 'chai';
import {
    convertSimpleFlashSwapToBatchSwapParameters,
    querySimpleFlashSwap,
} from '..';
import { SwapType } from '../../types';
import vaultAbi from '@/lib/abi/Vault.json';
import { balancerVault } from '../../../../lib/constants/config';
import { InfuraProvider, Provider } from '@ethersproject/providers';
import { Network } from '../../../../lib/constants/network';

describe('convertSimpleFlashSwapToBatchSwapParameters', () => {
    it('should convert flash swap parameters to batch swap parameters', () => {
        const flashSwapParams = {
            flashLoanAmount: '10000',
            poolIds: [
                '0x7320d680ca9bce8048a286f00a79a2c9f8dcd7b3000100000000000000000044',
                '0x36128d5436d2d70cab39c9af9cce146c38554ff0000100000000000000000008',
            ],
            assets: [
                '0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174',
                '0x9a71012B13CA4d3D0Cdc72A177DF3ef03b0E76A3',
            ],
            walletAddress: '0x35f5a330FD2F8e521ebd259FA272bA8069590741',
        };

        const batchSwapParameters = {
            kind: SwapType.SwapExactIn,
            swaps: [
                {
                    poolId: '0x7320d680ca9bce8048a286f00a79a2c9f8dcd7b3000100000000000000000044',
                    assetInIndex: 0,
                    assetOutIndex: 1,
                    amount: '10000',
                    userData: '0x',
                },
                {
                    poolId: '0x36128d5436d2d70cab39c9af9cce146c38554ff0000100000000000000000008',
                    assetInIndex: 1,
                    assetOutIndex: 0,
                    amount: '0',
                    userData: '0x',
                },
            ],
            assets: [
                '0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174',
                '0x9a71012B13CA4d3D0Cdc72A177DF3ef03b0E76A3',
            ],
            funds: {
                fromInternalBalance: false,
                recipient: '0x35f5a330FD2F8e521ebd259FA272bA8069590741',
                sender: '0x35f5a330FD2F8e521ebd259FA272bA8069590741',
                toInternalBalance: false,
            },
            limits: ['0', '0'], // No limits
            deadline: '999999999999999999', // Infinity
        };

        expect(
            convertSimpleFlashSwapToBatchSwapParameters(flashSwapParams)
        ).to.eql(batchSwapParameters);
    });
});
describe('querySimpleFlashSwap', () => {
    describe('response', async function () {
        // This request is sometimes slow
        this.slow(3000);

        const response = await querySimpleFlashSwap({
            vaultContract: new Contract(
                balancerVault,
                vaultAbi,
                new InfuraProvider(Network.POLYGON, process.env.INFURA)
            ),
            flashLoanAmount: '10000',
            poolIds: [
                '0x7320d680ca9bce8048a286f00a79a2c9f8dcd7b3000100000000000000000044',
                '0x36128d5436d2d70cab39c9af9cce146c38554ff0000100000000000000000008',
            ],
            assets: [
                '0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174',
                '0x9a71012B13CA4d3D0Cdc72A177DF3ef03b0E76A3',
            ],
        });

        it('should return the estimated profits', () => {
            assert.isNumber(
                Number(
                    response.profits[
                        '0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174'
                    ]
                )
            );

            assert.isNumber(
                Number(
                    response.profits[
                        '0x9a71012B13CA4d3D0Cdc72A177DF3ef03b0E76A3'
                    ]
                )
            );
        });

        it('should indicated if flash swap will be profitable', async () => {
            assert.isBoolean(response.isProfitable);
        });
    });
});
