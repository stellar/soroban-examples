//! Tests CLI commands from https://soroban.stellar.org/docs/how-to-guides/events

use soroban_test::{TestEnv, Wasm};

const WASM: &Wasm = &Wasm::Release("soroban_events_contract");

const EXPECTED_EVENT: &str = r#"#0: event: {"ext":"v0","contract_id":"0000000000000000000000000000000000000000000000000000000000000001","type_":"contract","body":{"v0":{"topics":[{"symbol":"COUNTER"},{"symbol":"increment"}],"data":{"u32":1}}}}
"#;

#[test]
fn invoke() {
    TestEnv::with_default(|workspace| {
        // events get dumped right to STDERR using eprintln; need to test with assert_cmd
        workspace
            .new_assert_cmd("contract")
            .arg("invoke")
            .arg("--wasm")
            .arg(&WASM.path())
            .args(["--id", "1"])
            .args(["--", "increment"])
            .assert()
            .stdout("1\n")
            .stderr(EXPECTED_EVENT);
    });
}
