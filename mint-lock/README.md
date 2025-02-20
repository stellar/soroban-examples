# Mint Lock
The mint lock example demonstrates how to write a contract that can delegate minting tokens from another address with limits on how much those addresses can mint across a specified time period.

The admin of the token contracts used must be the mint lock contract.

## Test
For a quick test of the smart contract, run a test using the provided test file, `mint-lock/src/test.rs`. The test will just return a pass/fail result, but it’s a convenient way to check if the code works, without deploying and invoking the contract manually. The test file also demonstates how to invoke the smart contract. 

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
- [Open example in GitPod](https://gitpod.io/#https://github.com/stellar/soroban-examples)
- [Getting Started documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started)

