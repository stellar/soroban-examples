# Cross Contract Calls
The cross contract call example demonstrates how to call a contract from another contract. In this example there are two contracts that are compiled separately, deployed separately, and then tested together. There are a variety of ways to develop and test contracts with dependencies on other contracts, and the Soroban SDK and tooling is still building out the tools to support these workflows.


## Test
For a quick test of the smart contract, run a test using the provided test file. Most examples only have one contract, but since this example is demonstrating how to make cross contract calls, two contracts are needed. They are named `contract_a` and `contract_b`.

To run the tests for the example, navigate to the `cross_contract/contract_b` directory, which will contain the test file `auth/src/test.rs`, and run cargo test. Before running the test, `contract_a` must be built.

The test will just return a pass/fail result, but it’s a convenient way to check if the code works, without deploying and invoking the contract manually. The test file also demonstates how to invoke the smart contract. 


```
cd contract_a
stellar contract build
```



From the root of the contract run this command:

```
cd ../contract_b
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
- [Cross contract documentation](https://developers.stellar.org/docs/learn/encyclopedia/contract-development/contract-interactions/cross-contract)
- [Detailed description of this example](https://developers.stellar.org/docs/build/smart-contracts/example-contracts/cross-contract-call)
- [Getting Started documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started)
