# Increment With Pause
This contract is built on top of the increment contract, and let's the contract be paused by a dependency. 

## Test
For a quick test of the smart contract, run a test using the provided test file, `increment_with_pause/src/test_real.rs`. The test will just return a pass/fail result, but it’s a convenient way to check if the code works, without deploying and invoking the contract manually. The test file also demonstates how to invoke the smart contract. 

Besides the test file mentioned above, there's also a mock test file, `increment_with_pause/src/test_mock.rs`. The first test file requires a pausing dependency ([pause](../pause)) to exist, and the mock test can run without a pausing dependency. In the example below, the test with a dependency is used. It is necessary to build the dependency before running the test.

From the root of the contract run this command:

```
cd ../pause
stellar contract build

cd ../increment_with_pause
cargo test test_real
```

You should see the output:

```
running 1 test
test test::test ... ok
```

See the main [README](../README.md) file for information about how to build and invoke the code using the CLI.

## Relevant Links
- [Open example in GitPod](https://gitpod.io/#https://github.com/stellar/soroban-examples/tree/v21.6.0)
- [Getting Started documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started)

