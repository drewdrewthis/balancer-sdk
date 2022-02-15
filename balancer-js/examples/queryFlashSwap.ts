import dotenv from 'dotenv';
import { BalancerSDK, Network } from '../src';
import { BalancerSdkConfig } from '../src/types';
import { DAI, USDC } from './constants';

dotenv.config();

const { INFURA } = process.env;

const network = Network.KOVAN;
const rpcUrl = `https://kovan.infura.io/v3/${INFURA}`;

/**
 * Example showing how to use flashSwap.
 *
 * To find pool ids and token adddresses on Kova:
 * https://thegraph.com/hosted-service/subgraph/balancer-labs/balancer-kovan-v2
 *
 * Gotchas:
 * - Both pools must have both assets for swaps to work
 * - No token balances can be zero
 * - flashLoanAmount must not add or subtract > 30% of pool liquidity
 */
async function runQueryFlashSwap() {
    /**
     * Step 1: Create encoded data string for transaction below.
     */
    const config: BalancerSdkConfig = { network, rpcUrl };

    const balancer = new BalancerSDK(config);

    const response = await balancer.swaps.querySimpleFlashSwap({
        flashLoanAmount: '100',
        poolIds: [
            '0x0cdab06b07197d96369fea6f3bea6efc7ecdf7090002000000000000000003de',
            '0x17018c2f7c345add873474879ff0ed98ebd6346a000200000000000000000642',
        ],
        assets: [USDC.address, DAI.address],
    });

    console.table(response);
}

// yarn examples:run ./examples/queryFlashSwap.ts
runQueryFlashSwap();
