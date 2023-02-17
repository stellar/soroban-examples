//! Tests CLI commands from https://soroban.stellar.org/docs/how-to-guides/hello-world

use soroban_test::{TestEnv, Wasm};

const WASM: &Wasm = &Wasm::Release("soroban_hello_world_contract");
const FRIEND: &str = "friend";

#[test]
fn invoke() {
    TestEnv::with_default(|workspace| {
        assert_eq!(
            format!("[\"Hello\",\"{FRIEND}\"]"),
            workspace
                .invoke(&[
                    "--id",
                    "1",
                    "--wasm",
                    &WASM.path().to_string_lossy(),
                    "--",
                    "hello",
                    "--to",
                    FRIEND,
                ])
                .unwrap()
        );
    });
}
