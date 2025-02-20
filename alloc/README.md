# Alloc
The allocator example demonstrates how to utilize the allocator feature when writing a contract. The `soroban-sdk` crate provides a lightweight bump-pointer allocator which can be used to emulate heap memory allocation in a Wasm smart contract.

## Test
For a quick test of the smart contract, run a test using the provided test file, `alloc/src/test.rs`. The test will just return a pass/fail result, but it’s a convenient way to check if the code works, without deploying and invoking the contract manually. The test file also demonstates how to invoke the smart contract. 

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
- [Allocation documentation](https://developers.stellar.org/docs/learn/encyclopedia/contract-development/rust-dialect#limited-ideally-zero-dynamic-memory-allocation)
- [Detailed description of this example](https://developers.stellar.org/docs/build/smart-contracts/example-contracts/alloc)
- [Getting Started documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started)

