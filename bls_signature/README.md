# BLS Signature Custom Account

This is a basic custom account contract that implements the `FastAggregateVerify` function in [BLS Signatures](https://www.ietf.org/archive/id/draft-irtf-cfrg-bls-signature-05.html#name-fastaggregateverify). 

## Test
For a quick test of the smart contract, run a test using the provided test file, `bls_signature/src/test.rs`. The test will just return a pass/fail result, but it’s a convenient way to check if the code works, without deploying and invoking the contract manually. The test file also demonstates how to invoke the smart contract. 

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
- [BLS Signatures](https://www.ietf.org/archive/id/draft-irtf-cfrg-bls-signature-05.html#name-fastaggregateverify)
- [Hashing to Elliptic Curves](https://datatracker.ietf.org/doc/html/rfc9380)
- [Getting Started documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started)

