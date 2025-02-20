# Fuzz Testing
The fuzzing example demonstrates how to fuzz test Soroban contracts with `cargo-fuzz` and customize the input to fuzz tests with the `arbitrary` crate. It also demonstrates how to adapt fuzz tests into reusable property tests with the `proptest` and `proptest-arbitrary-interop` crates. It builds on the timelock example.

## Test
You will need the cargo-fuzz tool, and to run cargo-fuzz you will need a nightly Rust toolchain:

```
cargo install cargo-fuzz
rustup install nightly
```

To run one of the fuzz tests, navigate to the fuzzing directory and run the cargo fuzz subcommand with the nightly toolchain:

```
cd fuzzing
cargo +nightly fuzz run fuzz_target_1
```

See the main [README](../README.md) file for information about how to build and invoke the code using the CLI.

## Relevant Links
- [Open example in GitPod](https://gitpod.io/#https://github.com/stellar/soroban-examples)
- [Unit test documentation](https://developers.stellar.org/docs/build/guides/testing/unit-tests)
- [Detailed description of this example](https://developers.stellar.org/docs/build/smart-contracts/example-contracts/fuzzing)
- [Getting Started documentation](https://developers.stellar.org/docs/build/smart-contracts/getting-started)
