# Long-term simulation with a changing token price

This example implements a scenario where:

- Transaction fees are paid in native tokens, e.g. ETH, CLX.
- AutoPrice governs the gas price, which is set in terms of native tokens.
- Buyers' purchasing power, expressed in fiat terms, remains constant throughout the years.
- Fee market demand cycles within the same bounds every day.
- The native token's price in terms of fiat is subject to high volatility. In this example, hourly ETHUSD exchange rate is used.

To run the example, execute the following:

```
bash ../../generate_demand_profiles.sh
cargo run --release -- -c config.toml
```

## Results

Since the gas price is set in native tokens, whose price in fiat is in turn volatile, AutoPrice is not able to catch up with the market using just daily adjustments. Even though AutoPrice targets a conservative aggregate block fullness at 65%---that is, AutoPrice is fine with a large percentage (35%) of aggregate block space being empty---it's not able to prevent surges, if the token price suddenly drops during the day. That is because token prices in crypto can drop up to ~10-20% and occasionally ~50% during the day. This reflects to the simulation as increased gas purchasing power of the buyers, who are now able to afford more transactions. We assume an efficient market, so buyers do end up sending more transactions, causing the surge.

Moreover, plotting the graph of gas price versus time, we see that it is roughly a daily averaged version of the multiplicative inverse of the token price.
