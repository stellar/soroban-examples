//! Tests CLI commands from https://soroban.stellar.org/docs/how-to-guides/errors

use soroban_cli::commands::contract::invoke;
use soroban_test::{TestEnv, Wasm};

const WASM: &Wasm = &Wasm::Release("soroban_errors_contract");

const EXPECTED_ERROR_START: &str = r#"HostError
Value: Status(ContractError(1))
"#;

#[test]
fn invoke() {
    TestEnv::with_default(|sandbox| {
        let increment = || {
            sandbox.invoke(&[
                "--wasm",
                &WASM.path().to_string_lossy(),
                "--id",
                "1",
                "--",
                "increment",
            ])
        };

        // works the first five times
        assert_eq!(increment().unwrap(), "1");
        assert_eq!(increment().unwrap(), "2");
        assert_eq!(increment().unwrap(), "3");
        assert_eq!(increment().unwrap(), "4");
        assert_eq!(increment().unwrap(), "5");

        // then errors
        let res = increment();

        assert!(matches!(res, Err(invoke::Error::Host(_))));
        assert!(format!("{res:?}").contains(EXPECTED_ERROR_START));
    });
}
