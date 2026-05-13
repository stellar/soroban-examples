mod common;

use common::{deploy, load_fixture, replace_first_signal};
use soroban_sdk::{Address, Env, testutils::Address as _};

#[test]
fn verifies_circom_fixture_with_constructor_supplied_vk() {
    let env = Env::default();
    env.mock_all_auths();

    let fixture = load_fixture(&env, "circom");
    let admin = Address::generate(&env);
    let client = deploy(&env, &admin, &fixture.verification_key);

    assert!(client.verify_proof(&fixture.proof, &fixture.public_signals));
}

#[test]
fn rejects_circom_fixture_with_wrong_public_signal() {
    let env = Env::default();
    env.mock_all_auths();

    let fixture = load_fixture(&env, "circom");
    let admin = Address::generate(&env);
    let client = deploy(&env, &admin, &fixture.verification_key);
    let wrong_signals = replace_first_signal(&env, &fixture.public_signals, "22");

    assert!(!client.verify_proof(&fixture.proof, &wrong_signals));
}
