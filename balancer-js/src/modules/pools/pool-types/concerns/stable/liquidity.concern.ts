import { LiquidityConcern } from '../types';
import { PoolToken } from '@/types';
import { BigNumber, parseFixed, formatFixed } from '@ethersproject/bignumber';

const SCALING_FACTOR = 18;

export class StablePoolLiquidity implements LiquidityConcern {
  calcTotal(tokens: PoolToken[]): string {
    let sumBalance = BigNumber.from(0);
    let sumValue = BigNumber.from(0);

    for (let i = 0; i < tokens.length; i++) {
      const token = tokens[i];

      // if a token's price is unknown, ignore it
      // it will be computed at the next step
      if (!token.price?.usd) {
        continue;
      }

      const price = parseFixed(token.price.usd, SCALING_FACTOR);
      const balance = parseFixed(token.balance, SCALING_FACTOR);

      const value = balance.mul(price);
      sumValue = sumValue.add(value);
      sumBalance = sumBalance.add(balance);
    }

    // if at least the partial value of the pool is known
    // then compute the rest of the value of tokens with unknown prices
    if (sumBalance.gt(0)) {
      const avgPrice = sumValue.div(sumBalance);

      for (let i = 0; i < tokens.length; i++) {
        const token = tokens[i];

        if (token.price?.usd) {
          continue;
        }

        const balance = parseFixed(token.balance, SCALING_FACTOR);

        const value = balance.mul(avgPrice);
        sumValue = sumValue.add(value);
        sumBalance = sumBalance.add(balance);
      }
    }

    return formatFixed(sumValue, SCALING_FACTOR * 2).toString();
  }
}
