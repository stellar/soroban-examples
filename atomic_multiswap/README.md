# Atomic Multiswap
This atomic swap batching example swaps a pair of tokens between the two groups of users that authorized the swap operation from the [Atomic Swap](../atomic_swap) example. This contract basically batches the multiple swaps while following some simple rules to match the swap participants.

## Test
For a quick test of the smart contract, run a test using the provided test file, `atomic_multiswap/src/test.rs`. The test will just return a pass/fail result, but it’s a convenient way to check if the code works, without deploying and invoking the contract manually. The test file also demonstates how to invoke the smart contract. 

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
- [Authorization documentation](https://developers.stellar.org/docs/learn/encyclopedia/security/authorization)
- [Detailed description of this example](https://developers.stellar.org/docs/build/smart-contracts/example-contracts/atomic-multi-swap)
- [Getting Started documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started)
