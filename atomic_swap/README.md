# Atomic Swap
This example contract swaps two tokens between two authorized parties atomically while following the limits they set. This example demonstrates advanced usage of Soroban auth framework and assumes the reader is familiar with the [auth](../auth) example and with Soroban token usage.

## Test
For a quick test of the smart contract, run a test using the provided test file, `atomic_swap/src/test.rs`. The test will just return a pass/fail result, but it’s a convenient way to check if the code works, without deploying and invoking the contract manually. The test file also demonstates how to invoke the smart contract. 

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
- [Authorization documentation](https://developers.stellar.org/docs/learn/encyclopedia/security/authorization)
- [Detailed description of this example](https://developers.stellar.org/docs/build/smart-contracts/example-contracts/atomic-swap)
- [Getting Started documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started)
