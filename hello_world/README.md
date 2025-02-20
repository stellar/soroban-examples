# Hello World
The hello world example demonstrates how to create, build and deploy a simple smart contract on Soroban. The example is used for the [Getting Started](https://developers.stellar.org/docs/build/smart-contracts/getting-started) guide in the documentation.

## Test
For a quick test of the smart contract, run a test using the provided test file, `hello_world/src/test.rs`. The test will just return a pass/fail result, but it’s a convenient way to check if the code works, without deploying and invoking the contract manually. The test file also demonstates how to invoke the smart contract. 

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
- [Detailed description of this example](https://developers.stellar.org/docs/build/smart-contracts/getting-started/hello-world)
- [Getting Started documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started)

