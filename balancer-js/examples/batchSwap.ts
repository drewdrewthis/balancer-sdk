import Web3 from 'web3';
import dotenv from 'dotenv';
import { BalancerSDK, Network, SwapType } from '../src';
import { BalancerSdkConfig } from '../src/types';
import { BALANCER_NETWORK_CONFIG } from '../src/lib/constants/config';

dotenv.config();

const { WALLET_PRIVATE_KEY, INFURA } = process.env;

const network = Network.KOVAN;
const rpcUrl = `https://kovan.infura.io/v3/${INFURA}`;

const web3 = new Web3(new Web3.providers.HttpProvider(rpcUrl));
const account = web3.eth.accounts.privateKeyToAccount(WALLET_PRIVATE_KEY);
const { address } = account;

/**
 * Example showing how to use encodeBatchSwap. User must sign and send transaction.
 */
async function runEncodeBatchSwap() {
    /**
     * Step 1: Create encoded data string for transaction below.
     */
    const config: BalancerSdkConfig = { network, rpcUrl };

    const balancer = new BalancerSDK(config);

    const data = balancer.swaps.encodeBatchSwap({
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

    /**
     * Step 2: Build transaction object to be passed into web3 interface.
     */
    const tx_object = {
        chainId: Network.KOVAN,
        gas: web3.utils.toHex('100000'),
        gasPrice: web3.utils.toHex('100'),
        nonce: await web3.eth.getTransactionCount(address),
        data,
        to: BALANCER_NETWORK_CONFIG[network].addresses.contracts.vault,
    };

    /**
     * Step 3: Execute (sign + send)
     */
    const signed_tx = await web3.eth.accounts
        .signTransaction(tx_object, WALLET_PRIVATE_KEY)
        .then((stx: { rawTransaction: string }) =>
            web3.eth
                .sendSignedTransaction(stx.rawTransaction)
                .on('receipt', console.log)
        );

    console.log(signed_tx);
}

// yarn examples:run ./examples/flashSwap.ts
runEncodeBatchSwap();
