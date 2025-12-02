# Deployer
The deployer example demonstrates how to deploy contracts using a contract.

Here we deploy a contract on behalf of any address and initialize it atomically.

## Test
For a quick test of the smart contract, run a test using the provided test file. Most examples only have one contract, but since this example is demonstrating deploying one contract from another, two contracts are needed. They are named `contract` and `deployer`.

To run the tests for the example, navigate to the `deployer/deployer` directory, which will contain the test file `deployer/src/test.rs`, and run cargo test. Before running the test, `contract` must be built.

The test will just return a pass/fail result, but it’s a convenient way to check if the code works, without deploying and invoking the contract manually. The test file also demonstates how to invoke the smart contract. 


```
cd contract
stellar contract build
```

From the root of the contract run this command:

```
cd ../deployer
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
- [Contract deployment documentation](https://developers.stellar.org/docs/build/guides/conventions/deploy-contract)
- [Detailed description of this example](https://developers.stellar.org/docs/build/smart-contracts/example-contracts/deployer)
- [Getting Started documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started)

