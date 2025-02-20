# Liquidity Pool
The liquidity pool example demonstrates how to write a constant product liquidity pool contract. A liquidity pool is an automated way to add liquidity for a set of tokens that will facilitate asset conversion between them. Users can deposit some amount of each token into the pool, receiving a proportional number of "token shares." The user will then receive a portion of the accrued conversion fees when they ultimately "trade in" their token shares to receive their original tokens back.

## Test
For a quick test of the smart contract, run a test using the provided test file, `liquidity_pool/src/test.rs`. The test will just return a pass/fail result, but it’s a convenient way to check if the code works, without deploying and invoking the contract manually. The test file also demonstates how to invoke the smart contract. 

From the root of the contract run this command:

```
cargo test
```

You should see the output:

```
running 1 test
test test::test ... ok
```

See the main [README](../README.md) file for information about how to build and invoke the code using the CLI.

## Relevant Links
- [Open example in GitPod](https://gitpod.io/#https://github.com/stellar/soroban-examples/tree/v21.6.0)
- [Detailed description of this example](https://developers.stellar.org/docs/build/smart-contracts/example-contracts/liquidity-pool)
- [Getting Started documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started)

