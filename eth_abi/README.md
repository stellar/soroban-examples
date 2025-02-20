# ETH ABI
The custom types example demonstrates how to decode contract specs in the Application Binary Interface format. The example uses a provided JSON file `test_snapshot/test/test_exec.1.json`.

## Test
For a quick test of the smart contract, run a test using the provided test file, `eth_abi/src/test.rs`. The test will just return a pass/fail result, but it’s a convenient way to check if the code works, without deploying and invoking the contract manually. The test file also demonstates how to invoke the smart contract. 

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
- [Understanding the contract specification](https://developers.stellar.org/docs/build/guides/dapps/working-with-contract-specs#understanding-the-contract-specification)
- [Getting Started documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started)
