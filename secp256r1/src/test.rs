#![cfg(test)]
extern crate std;

use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use rand_core::OsRng;
use soroban_sdk::{testutils::BytesN as _, vec, Bytes, BytesN, Env, Error as SdkError};

use crate::{Secp256r1Contract, Secp256r1ContractArgs, Secp256r1ContractClient};

// NIST CAVP P-256 (secp256r1) test vector
// Public Key: 65 bytes uncompressed (0x04 || X || Y)
const PUB_KEY_HEX: [u8; 65] = [
    0x04, 0xe4, 0x24, 0xdc, 0x61, 0xd4, 0xbb, 0x3c, 0xb7, 0xef, 0x43, 0x44, 0xa7, 0xf8, 0x95, 0x7a,
    0x0c, 0x51, 0x34, 0xe1, 0x6f, 0x7a, 0x67, 0xc0, 0x74, 0xf8, 0x2e, 0x6e, 0x12, 0xf4, 0x9a, 0xbf,
    0x3c, 0x97, 0x0e, 0xed, 0x7a, 0xa2, 0xbc, 0x48, 0x65, 0x15, 0x45, 0x94, 0x9d, 0xe1, 0xdd, 0xda,
    0xf0, 0x12, 0x7e, 0x59, 0x65, 0xac, 0x85, 0xd1, 0x24, 0x3d, 0x6f, 0x60, 0xe7, 0xdf, 0xae, 0xe9,
    0x27,
];

// SHA-256 Message Digest: 32 bytes
const DIGEST_HEX: [u8; 32] = [
    0xd1, 0xb8, 0xef, 0x21, 0xeb, 0x41, 0x82, 0xee, 0x27, 0x06, 0x38, 0x06, 0x10, 0x63, 0xa3, 0xf3,
    0xc1, 0x6c, 0x11, 0x4e, 0x33, 0x93, 0x7f, 0x69, 0xfb, 0x23, 0x2c, 0xc8, 0x33, 0x96, 0x5a, 0x94,
];

// Signature: 64 bytes (r || s)
const SIG_HEX: [u8; 64] = [
    0xbf, 0x96, 0xb9, 0x9a, 0xa4, 0x9c, 0x70, 0x5c, 0x91, 0x0b, 0xe3, 0x31, 0x42, 0x01, 0x7c, 0x64,
    0x2f, 0xf5, 0x40, 0xc7, 0x63, 0x49, 0xb9, 0xda, 0xb7, 0x2f, 0x98, 0x1f, 0xd9, 0x34, 0x7f, 0x4f,
    0x17, 0xc5, 0x50, 0x95, 0x81, 0x90, 0x89, 0xc2, 0xe0, 0x3b, 0x9c, 0xd4, 0x15, 0xab, 0xdf, 0x12,
    0x44, 0x4e, 0x32, 0x30, 0x75, 0xd9, 0x8f, 0x31, 0x92, 0x0b, 0x9e, 0x0f, 0x57, 0xec, 0x87, 0x1c,
];

#[test]
fn test_nist_cavp_custom_account_auth() {
    let env = Env::default();
    let public_key = BytesN::from_array(&env, &PUB_KEY_HEX);
    let signature = BytesN::from_array(&env, &SIG_HEX);
    let payload = BytesN::from_array(&env, &DIGEST_HEX);

    let contract_id = env.register(
        Secp256r1Contract,
        Secp256r1ContractArgs::__constructor(&public_key),
    );
    let client = Secp256r1ContractClient::new(&env, &contract_id);

    // Verify owner getter
    assert_eq!(client.get_owner(), public_key);

    // Verify valid signature passes custom account __check_auth
    env.try_invoke_contract_check_auth::<SdkError>(
        &client.address,
        &payload,
        signature.into(),
        &vec![&env],
    )
    .unwrap();

    // Verify invalid signature is rejected by custom account __check_auth
    let random_signature = BytesN::<64>::random(&env);
    assert!(env
        .try_invoke_contract_check_auth::<SdkError>(
            &client.address,
            &payload,
            random_signature.into(),
            &vec![&env],
        )
        .is_err());
}

#[test]
fn test_dynamic_key_verification_and_account_auth() {
    let env = Env::default();

    // Generate random P-256 keypair
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    let encoded_point = verifying_key.to_encoded_point(false);
    let pub_bytes: [u8; 65] = encoded_point.as_bytes().try_into().unwrap();
    let public_key = BytesN::from_array(&env, &pub_bytes);

    // Register contract
    let contract_id = env.register(
        Secp256r1Contract,
        Secp256r1ContractArgs::__constructor(&public_key),
    );
    let client = Secp256r1ContractClient::new(&env, &contract_id);

    assert_eq!(client.get_owner(), public_key);

    // 1. Test message verification with verify and verify_with_owner
    let message_bytes = b"Hello, Stellar Soroban secp256r1!";
    let message = Bytes::from_slice(&env, message_bytes);

    let signature_obj: Signature = signing_key.sign(message_bytes);
    let signature_obj = signature_obj.normalize_s().unwrap_or(signature_obj);
    let sig_bytes: [u8; 64] = signature_obj.to_bytes().into();
    let signature = BytesN::from_array(&env, &sig_bytes);

    // Verification succeeds
    client.verify(&public_key, &message, &signature);
    client.verify_with_owner(&message, &signature);

    // Tampered message fails
    let tampered_message = Bytes::from_slice(&env, b"Tampered message content!");
    let res = client.try_verify(&public_key, &tampered_message, &signature);
    assert!(res.is_err());

    let res_owner = client.try_verify_with_owner(&tampered_message, &signature);
    assert!(res_owner.is_err());

    // Tampered signature fails
    let mut bad_sig_bytes = sig_bytes;
    bad_sig_bytes[0] ^= 0xff;
    let bad_signature = BytesN::from_array(&env, &bad_sig_bytes);
    assert!(client
        .try_verify(&public_key, &message, &bad_signature)
        .is_err());
    assert!(client
        .try_verify_with_owner(&message, &bad_signature)
        .is_err());

    // 2. Test custom account authentication (__check_auth)
    // In Soroban, signature_payload is a 32-byte hash passed by the host during require_auth
    let auth_payload = BytesN::<32>::random(&env);
    // Sign the 32-byte payload directly with ECDSA (prehash)
    use p256::ecdsa::signature::hazmat::PrehashSigner;
    let auth_sig: Signature = signing_key.sign_prehash(&auth_payload.to_array()).unwrap();
    let auth_sig = auth_sig.normalize_s().unwrap_or(auth_sig);
    let auth_sig_bytes: [u8; 64] = auth_sig.to_bytes().into();
    let auth_signature = BytesN::from_array(&env, &auth_sig_bytes);

    // Passes __check_auth
    env.try_invoke_contract_check_auth::<SdkError>(
        &client.address,
        &auth_payload,
        auth_signature.clone().into(),
        &vec![&env],
    )
    .unwrap();

    // Mismatched payload fails
    let wrong_payload = BytesN::<32>::random(&env);
    assert!(env
        .try_invoke_contract_check_auth::<SdkError>(
            &client.address,
            &wrong_payload,
            auth_signature.into(),
            &vec![&env],
        )
        .is_err());
}
