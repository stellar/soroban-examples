# Tokens
The token example demonstrates how to write a token contract that implements the [Token Interface](https://developers.stellar.org/docs/tokens/token-interface).

## Test
For a quick test of the smart contract, run a test using the provided test file, `token/src/test.rs`. The test will just return a pass/fail result, but it’s a convenient way to check if the code works, without deploying and invoking the contract manually. The test file also demonstates how to invoke the smart contract. 

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
- [Detailed description of this example](https://developers.stellar.org/docs/build/smart-contracts/example-contracts/tokens)
- [Getting Started documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started)

