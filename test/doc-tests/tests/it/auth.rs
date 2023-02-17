//! Tests CLI commands from https://soroban.stellar.org/docs/how-to-guides/auth

use soroban_cli::commands::config::identity::address;
use soroban_test::{TestEnv, Wasm};

const WASM: &Wasm = &Wasm::Release("soroban_auth_contract");

const INCREMENT_1: &str = "2";
const INCREMENT_2: &str = "5";

fn account_pk(workspace: &TestEnv, hd_path: u8) -> String {
    workspace
        .cmd_arr::<address::Cmd>(&["--hd-path", &hd_path.to_string()])
        .public_key()
        .unwrap()
        .to_string()
}

#[test]
fn invoke_and_read() {
    TestEnv::with_default(|workspace| {
        let account_0_pk = account_pk(workspace, 0);
        let account_1_pk = account_pk(workspace, 1);

        assert_eq!(
            format!("{INCREMENT_1}"),
            workspace
                .invoke(&[
                    "--wasm",
                    &WASM.path().to_string_lossy(),
                    "--id",
                    "1",
                    "--",
                    "increment",
                    "--user",
                    &account_0_pk,
                    "--value",
                    &INCREMENT_1,
                ])
                .unwrap(),
        );
        assert_eq!(
            format!("{INCREMENT_2}"),
            workspace
                .invoke(&[
                    "--hd-path",
                    "1",
                    "--wasm",
                    &WASM.path().to_string_lossy(),
                    "--id",
                    "1",
                    "--",
                    "increment",
                    "--user",
                    &account_1_pk,
                    "--value",
                    &INCREMENT_2,
                ])
                .unwrap(),
        );

        // `read::Cmd`'s `run` returns `()` & dumps right to STDOUT; need to use assert_cmd
        workspace
            .new_assert_cmd("contract")
            .arg("read")
            .args(["--id", "1"])
            .assert()
            .stderr("")
            .stdout(format!(
                r#""[""Counter"",""{account_0_pk}""]",{INCREMENT_1}
"[""Counter"",""{account_1_pk}""]",{INCREMENT_2}
"#
            ));
    });
}

#[test]
fn invoke_auth_preview() {
    TestEnv::with_default(|workspace| {
        let account_0_pk = account_pk(workspace, 0);

        // `invoke::Cmd`'s `auth` option prints directly to STDERR; need to test with assert_cmd
        workspace.new_assert_cmd("contract")
            .arg("invoke")
            .arg("--auth")
            .arg("--wasm")
            .arg(&WASM.path())
            .args(["--id", "1"])
            .args(["--"])
            .arg("increment")
            .args(["--user", &account_0_pk])
            .args(["--value", &INCREMENT_1])
            .assert()
            .stdout(format!("{INCREMENT_1}\n"))
            .stderr(
                r#"Contract auth: [{"address_with_nonce":null,"root_invocation":{"contract_id":"0000000000000000000000000000000000000000000000000000000000000001","function_name":"increment","args":[{"address":{"account":{"public_key_type_ed25519":"d18f0210ff6cc1f2dcf1301fbbd4c30ee11a075820684d471df89d0f1011ea28"}}},{"u32":"#.to_string() + &format!("{INCREMENT_1}") + r#"}],"sub_invocations":[]},"signature_args":[]}]
"#
            );
    });
}
