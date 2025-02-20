# Deep Contract Auth
This example demonstrates how a contract can authorize deep subcontract calls on its behalf.

By default, only direct calls that contracts make are authorized. However, in some scenarios one may want to authorize a deeper call (a common example would be token transfer).

Here we provide the abstract example: contract A calls contract B, then contract B calls contract C. Both contract B and contract C `require_auth` for contract A address and contract A provides proper authorization to make the calls succeed.

## Test
For a quick test of the smart contract, run a test using the provided test file, `deep_contract_auth/src/test.rs`. The test will just return a pass/fail result, but it’s a convenient way to check if the code works, without deploying and invoking the contract manually. The test file also demonstates how to invoke the smart contract. 

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
- [Getting Started documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started)
