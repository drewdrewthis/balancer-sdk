import { Contract } from '@ethersproject/contracts';
import { assert, expect } from 'chai';
import {
    convertSimpleFlashSwapToBatchSwapParameters,
    querySimpleFlashSwap,
} from '..';
import { SwapType } from '../../types';
import vaultAbi from '@/lib/abi/Vault.json';
import { balancerVault } from '../../../../lib/constants/config';
import { InfuraProvider } from '@ethersproject/providers';
import { Network } from '../../../../lib/constants/network';

describe('convertSimpleFlashSwapToBatchSwapParameters', () => {
    it('should convert flash swap parameters to batch swap parameters', () => {
        const flashSwapParams = {
            flashLoanAmount: '10000',
            poolIds: [
                '0x0cdab06b07197d96369fea6f3bea6efc7ecdf7090002000000000000000003de',
                '0x17018c2f7c345add873474879ff0ed98ebd6346a000200000000000000000642',
            ],
            assets: [
                '0xc2569dd7d0fd715b054fbf16e75b001e5c0c1115',
                '0x04df6e4121c27713ed22341e7c7df330f56f289b',
            ],
            walletAddress: '0x35f5a330FD2F8e521ebd259FA272bA8069590741',
        };

        const batchSwapParameters = {
            kind: SwapType.SwapExactIn,
            swaps: [
                {
                    poolId: '0x0cdab06b07197d96369fea6f3bea6efc7ecdf7090002000000000000000003de',
                    assetInIndex: 0,
                    assetOutIndex: 1,
                    amount: '10000',
                    userData: '0x',
                },
                {
                    poolId: '0x17018c2f7c345add873474879ff0ed98ebd6346a000200000000000000000642',
                    assetInIndex: 1,
                    assetOutIndex: 0,
                    amount: '0',
                    userData: '0x',
                },
            ],
            assets: [
                '0xc2569dd7d0fd715b054fbf16e75b001e5c0c1115',
                '0x04df6e4121c27713ed22341e7c7df330f56f289b',
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
                new InfuraProvider(Network.KOVAN, process.env.INFURA)
            ),
            flashLoanAmount: '10000',
            poolIds: [
                '0x0cdab06b07197d96369fea6f3bea6efc7ecdf7090002000000000000000003de',
                '0x17018c2f7c345add873474879ff0ed98ebd6346a000200000000000000000642',
            ],
            assets: [
                '0xc2569dd7d0fd715b054fbf16e75b001e5c0c1115',
                '0x04df6e4121c27713ed22341e7c7df330f56f289b',
            ],
        });

        it('should return the estimated profits', () => {
            assert.isNumber(
                Number(
                    response.profits[
                        '0xc2569dd7d0fd715b054fbf16e75b001e5c0c1115'
                    ]
                )
            );

            assert.isNumber(
                Number(
                    response.profits[
                        '0x04df6e4121c27713ed22341e7c7df330f56f289b'
                    ]
                )
            );
        });

        it('should indicated if flash swap will be profitable', async () => {
            assert.isBoolean(response.isProfitable);
        });
    });
});
