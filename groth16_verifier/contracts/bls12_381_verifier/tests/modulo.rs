#[allow(dead_code)]
mod common;

use bls12_381_verifier::Groth16Error;
use common::{deploy, load_fixture};
use soroban_sdk::{
    Address, Bytes, BytesN, Env, IntoVal, Symbol, U256, Vec, testutils::Address as _,
};

const FR_MODULUS_BE: [u8; 32] = [
    0x73, 0xed, 0xa7, 0x53, 0x29, 0x9d, 0x7d, 0x48, 0x33, 0x39, 0xd8, 0x08, 0x09, 0xa1, 0xd8, 0x05,
    0x53, 0xbd, 0xa4, 0x02, 0xff, 0xfe, 0x5b, 0xfe, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x01,
];

fn bytes32(env: &Env, value: &U256) -> BytesN<32> {
    let mut bytes = [0; 32];
    value.to_be_bytes().copy_into_slice(&mut bytes);
    BytesN::from_array(env, &bytes)
}

#[test]
fn rejects_noncanonical_public_input() {
    let env = Env::default();
    env.mock_all_auths();
    let fixture = load_fixture(&env, "gnark");
    let client = deploy(&env, &Address::generate(&env), &fixture.verification_key);
    let canonical_signal = fixture.public_signals.get(0).unwrap();
    let x = U256::from_be_bytes(&env, &Bytes::from_array(&env, &canonical_signal.to_array()));
    let canonical = Vec::from_array(&env, [canonical_signal]);
    let modulus = U256::from_be_bytes(&env, &Bytes::from_array(&env, &FR_MODULUS_BE));
    let alias = Vec::from_array(&env, [bytes32(&env, &modulus.add(&x))]);

    assert_eq!(
        env.try_invoke_contract::<bool, Groth16Error>(
            &client.address,
            &Symbol::new(&env, "verify_proof"),
            (&fixture.proof, canonical).into_val(&env),
        ),
        Ok(Ok(true))
    );
    assert_eq!(
        env.try_invoke_contract::<bool, Groth16Error>(
            &client.address,
            &Symbol::new(&env, "verify_proof"),
            (&fixture.proof, alias).into_val(&env),
        ),
        Err(Ok(Groth16Error::NonCanonicalPublicInput))
    );
}

#[test]
fn enforces_scalar_field_boundary() {
    let env = Env::default();
    env.mock_all_auths();
    let fixture = load_fixture(&env, "gnark");
    let client = deploy(&env, &Address::generate(&env), &fixture.verification_key);
    let modulus = U256::from_be_bytes(&env, &Bytes::from_array(&env, &FR_MODULUS_BE));
    let largest = Vec::from_array(
        &env,
        [bytes32(&env, &modulus.sub(&U256::from_u32(&env, 1)))],
    );
    let modulus = Vec::from_array(&env, [bytes32(&env, &modulus)]);

    assert_eq!(
        env.try_invoke_contract::<bool, Groth16Error>(
            &client.address,
            &Symbol::new(&env, "verify_proof"),
            (&fixture.proof, largest).into_val(&env),
        ),
        Ok(Ok(false))
    );
    assert_eq!(
        env.try_invoke_contract::<bool, Groth16Error>(
            &client.address,
            &Symbol::new(&env, "verify_proof"),
            (&fixture.proof, modulus).into_val(&env),
        ),
        Err(Ok(Groth16Error::NonCanonicalPublicInput))
    );
}
