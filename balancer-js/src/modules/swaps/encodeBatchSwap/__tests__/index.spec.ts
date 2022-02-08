import { expect } from 'chai';
import { encodeBatchSwap } from '..';
import { SwapType } from '../../types';

describe('#encodeBatchSwap', () => {
    describe('#flashSwap', () => {
        context('success', () => {
            it('should return an ABI byte string', () => {
                const bytestring = encodeBatchSwap({
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
                    limits: [0, 0], // No limits
                    deadline: '999999999999999999', // Infinity
                });

                expect(bytestring).to.be('TO BE GENERATED ONCE IN PLACE');
            });
        });
    });
});
