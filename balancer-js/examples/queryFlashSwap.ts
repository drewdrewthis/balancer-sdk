import dotenv from 'dotenv';
import { BalancerSDK, Network } from '../src';
import { BalancerSdkConfig } from '../src/types';

dotenv.config();

const { INFURA } = process.env;

const network = Network.KOVAN;
const rpcUrl = `https://kovan.infura.io/v3/${INFURA}`;

/**
 * Example showing how to use flashSwap.
 */
async function runQueryFlashSwap() {
    /**
     * Step 1: Create encoded data string for transaction below.
     */
    const config: BalancerSdkConfig = { network, rpcUrl };

    const balancer = new BalancerSDK(config);

    const tx = balancer.swaps.queryFlashSwap({
        poolIds: [
            '0x7320d680ca9bce8048a286f00a79a2c9f8dcd7b3000100000000000000000044',
            '0x36128d5436d2d70cab39c9af9cce146c38554ff0000100000000000000000008',
        ],
        assets: [
            '0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174',
            '0x9a71012B13CA4d3D0Cdc72A177DF3ef03b0E76A3',
        ],
    });

    console.log(tx);
}

// yarn examples:run ./examples/flashSwap.ts
runQueryFlashSwap();
