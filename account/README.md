# Account
Accounts are the central data structure in Stellar- they hold balances, sign transactions, and issue assets. Accounts can only exist with a valid keypair and the required minimum balance of XLM. 

This example is a basic multi-sig account contract with a customizable per-token authorization policy. This example contract demonstrates how to build the account contract, and how to implement custom authorization policies that can govern all the account contract interactions.

Custom accounts are exclusive to Soroban and can't be used to perform other Stellar operations.

## Test
For a quick test of the smart contract, run a test using the provided test file, `account/src/test.rs`. The test will just return a pass/fail result, but it’s a convenient way to check if the code works, without deploying and invoking the contract manually. The test file also demonstates how to invoke the smart contract. 

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
- [Accounts documentation](https://developers.stellar.org/docs/learn/fundamentals/stellar-data-structures/accounts)
- [Detailed description of this example](https://developers.stellar.org/docs/build/smart-contracts/example-contracts/custom-account)
- [Getting Started documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started)


