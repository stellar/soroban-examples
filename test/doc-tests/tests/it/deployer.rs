//! Tests CLI commands from https://soroban.stellar.org/docs/how-to-guides/deployer

use soroban_cli::commands::contract::install;
use soroban_test::{TestEnv, Wasm};

const WASM_DEPLOYER_TEST: &Wasm = &Wasm::Release("soroban_deployer_test_contract");
const WASM_DEPLOYER: &Wasm = &Wasm::Release("soroban_deployer_contract");

const INIT_VALUE: &str = "5";

#[test]
fn invoke() {
    TestEnv::with_default(|workspace| {
        let hash = WASM_DEPLOYER_TEST.hash().unwrap();

        // install (aka upload) the bytes
        let install_ret = workspace
            .cmd_arr::<install::Cmd>(&["--wasm", &WASM_DEPLOYER_TEST.path().to_string_lossy()])
            .run_in_sandbox(WASM_DEPLOYER_TEST.bytes())
            .unwrap();
        assert_eq!(hash, install_ret);

        // now invoke a 2nd contract, which deploys an instance of the previously-uploaded bytes
        let new_contract_info_array = workspace
            .invoke(&[
                "--wasm",
                &WASM_DEPLOYER.path().to_string_lossy(),
                "--id",
                "0",
                "--",
                "deploy",
                "--salt",
                "0000000000000000000000000000000000000000000000000000000000000000",
                "--wasm_hash",
                &hash.to_string(),
                "--init_fn",
                "init",
                "--init_args",
                &format!("[{{\"u32\":{INIT_VALUE}}}]"),
            ])
            .unwrap();

        let contract_id = new_contract_info_array.split('"').nth(1).unwrap();

        assert_eq!(
            format!("{INIT_VALUE}"),
            workspace
                .invoke(&["--id", contract_id, "--", "value",])
                .unwrap()
        );
    });
}
