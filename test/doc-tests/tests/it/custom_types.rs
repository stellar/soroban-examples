//! Tests CLI commands from https://soroban.stellar.org/docs/how-to-guides/custom-types

use soroban_test::{TestEnv, Wasm};

const WASM: &Wasm = &Wasm::Release("soroban_custom_types_contract");

fn increment_by_5(workspace: &TestEnv) -> String {
    workspace
        .invoke(&[
            "--wasm",
            &WASM.path().to_string_lossy(),
            "--id",
            "1",
            "--",
            "increment",
            "--incr",
            "5",
        ])
        .unwrap()
}

#[test]
fn invoke() {
    TestEnv::with_default(|workspace| {
        assert_eq!("5", increment_by_5(workspace));
    });
}

const EXPECTED_READ: &str = r#"STATE,"{""count"":5,""last_incr"":5}"
"#;

#[test]
fn read() {
    TestEnv::with_default(|workspace| {
        increment_by_5(workspace);

        // `read::Cmd`'s `run` returns `()` & dumps right to STDOUT; need to use assert_cmd
        workspace
            .new_assert_cmd("contract")
            .arg("read")
            .args(["--id", "1"])
            .args(["--key", "STATE"])
            .assert()
            .stderr("")
            .stdout(EXPECTED_READ);
    });
}
