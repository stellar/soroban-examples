mod common;

use common::{deploy, load_fixture, replace_first_signal};
use soroban_sdk::{
    Address, Env, IntoVal,
    testutils::{Address as _, MockAuth, MockAuthInvoke},
};

#[test]
fn verifies_gnark_fixture_with_constructor_supplied_vk() {
    let env = Env::default();
    env.mock_all_auths();

    let fixture = load_fixture(&env);
    let admin = Address::generate(&env);
    let client = deploy(&env, &admin, &fixture.verification_key);

    assert!(client.verify_proof(&fixture.proof, &fixture.public_signals));
}

#[test]
fn rejects_gnark_fixture_with_wrong_public_signal() {
    let env = Env::default();
    env.mock_all_auths();

    let fixture = load_fixture(&env);
    let admin = Address::generate(&env);
    let client = deploy(&env, &admin, &fixture.verification_key);
    let wrong_signals = replace_first_signal(&env, &fixture.public_signals, "22");

    assert!(!client.verify_proof(&fixture.proof, &wrong_signals));
}

#[test]
fn set_verification_key_records_admin_auth() {
    let env = Env::default();
    env.mock_all_auths();

    let fixture = load_fixture(&env);
    let admin = Address::generate(&env);
    let client = deploy(&env, &admin, &fixture.verification_key);

    client
        .mock_auths(&[MockAuth {
            address: &admin,
            invoke: &MockAuthInvoke {
                contract: &client.address,
                fn_name: "set_verification_key",
                args: (&fixture.verification_key,).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .set_verification_key(&fixture.verification_key);

    assert!(client.verify_proof(&fixture.proof, &fixture.public_signals));
}
