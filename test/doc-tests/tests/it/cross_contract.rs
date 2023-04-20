//! Tests CLI commands from https://soroban.stellar.org/docs/how-to-guides/cross-contract-call

use soroban_cli::commands::contract::{deploy, install};
use soroban_test::{TestEnv, Wasm};

const WASM_A: &Wasm = &Wasm::Release("soroban_cross_contract_a_contract");
const WASM_B: &Wasm = &Wasm::Release("soroban_cross_contract_b_contract");

#[test]
fn invoke() {
    TestEnv::with_default(|workspace| {
        let hash_a = WASM_A.hash().unwrap();
        workspace
            .cmd_arr::<install::Cmd>(&["--wasm", &WASM_A.path().to_string_lossy()])
            .run_in_sandbox(WASM_A.bytes())
            .unwrap();
        workspace
            .cmd_arr::<deploy::Cmd>(&["--id", "a", "--wasm-hash", &format!("{hash_a}")])
            .run_in_sandbox(hash_a)
            .unwrap();

        let hash_b = WASM_B.hash().unwrap();
        workspace
            .cmd_arr::<install::Cmd>(&["--wasm", &WASM_B.path().to_string_lossy()])
            .run_in_sandbox(WASM_B.bytes())
            .unwrap();
        workspace
            .cmd_arr::<deploy::Cmd>(&["--id", "b", "--wasm-hash", &format!("{hash_b}")])
            .run_in_sandbox(hash_b)
            .unwrap();

        let res = workspace
            .invoke(&[
                "--id",
                "b",
                "--",
                "add_with",
                "--contract_id",
                "a",
                "--x",
                "5",
                "--y",
                "7",
            ])
            .unwrap();

        assert_eq!(res, "12");
    });
}
