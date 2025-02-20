# Custom Types
The custom types example demonstrates how to define your own data structures that can be stored on the ledger, or used as inputs and outputs to contract invocations. This example is an extension of the [storing data example](https://developers.stellar.org/docs/build/smart-contracts/getting-started/storing-data).


## Test
For a quick test of the smart contract, run a test using the provided test file, `custom_types/src/test.rs`. The test will just return a pass/fail result, but it’s a convenient way to check if the code works, without deploying and invoking the contract manually. The test file also demonstates how to invoke the smart contract. 

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
- [Custom types documentation](https://developers.stellar.org/docs/learn/encyclopedia/contract-development/types/custom-types)
- [Detailed description of this example](https://developers.stellar.org/docs/build/smart-contracts/example-contracts/custom-types)
- [Getting Started documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started)
