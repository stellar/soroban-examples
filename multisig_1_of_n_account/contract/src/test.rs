#![cfg(test)]
extern crate std;

use ed25519_dalek::{Keypair, Signer};
use rand::thread_rng;
use soroban_sdk::{testutils::BytesN as _, vec, BytesN, Env, IntoVal, InvokeError};

use crate::{Contract, Error, Signature};

#[test]
fn test_1of2_success() {
    let env = Env::default();

    // Generate signing keypairs.
    let signer1 = Keypair::generate(&mut thread_rng());
    let signer2 = Keypair::generate(&mut thread_rng());

    // Register the account contract, passing in the two signers (public keys) to the constructor.
    let contract_id = env.register(
        Contract,
        (vec![
            &env,
            BytesN::from_array(&env, &signer1.public.to_bytes()),
            BytesN::from_array(&env, &signer2.public.to_bytes()),
        ],),
    );

    // Generate a random payload to use for the test. When the account contract is being
    // called as part of a require_auth, the payload will be a hash of the network passphrase,
    // contract ID, function name, and all the parameters that the contract calling require_auth
    // has specified as should be part of the signature.
    let payload = BytesN::random(&env);

    assert_eq!(
        // `__check_auth` can't be called directly, hence we need to use
        // `try_invoke_contract_check_auth` testing utility that emulates being
        // called by the Soroban host during a `require_auth` call.
        env.try_invoke_contract_check_auth::<Error>(
            &contract_id,
            &payload,
            Signature {
                public_key: BytesN::from_array(&env, &signer1.public.to_bytes()),
                signature: BytesN::from_array(
                    &env,
                    &signer1.sign(payload.to_array().as_slice()).to_bytes()
                ),
            }
            .into_val(&env),
            &vec![&env],
        ),
        // If __check_auth returns no error, the signature verification succeeded.
        Ok(())
    );

    assert_eq!(
        env.try_invoke_contract_check_auth::<Error>(
            &contract_id,
            &payload,
            Signature {
                public_key: BytesN::from_array(&env, &signer2.public.to_bytes()),
                signature: BytesN::from_array(
                    &env,
                    &signer2.sign(payload.to_array().as_slice()).to_bytes()
                ),
            }
            .into_val(&env),
            &vec![&env],
        ),
        Ok(())
    );
}

#[test]
fn test_1of2_unknown_signer() {
    let env = Env::default();

    // Generate signing keypairs.
    let signer1 = Keypair::generate(&mut thread_rng());
    let signer2 = Keypair::generate(&mut thread_rng());
    let signer3 = Keypair::generate(&mut thread_rng()); // 👈 Unknown signer.

    // Register the account contract, passing in the two signers (public keys) to the constructor.
    let contract_id = env.register(
        Contract,
        (vec![
            &env,
            BytesN::from_array(&env, &signer1.public.to_bytes()),
            BytesN::from_array(&env, &signer2.public.to_bytes()),
        ],),
    );

    let payload = BytesN::random(&env);

    assert_eq!(
        env.try_invoke_contract_check_auth::<Error>(
            &contract_id,
            &payload,
            Signature {
                public_key: BytesN::from_array(&env, &signer3.public.to_bytes()),
                signature: BytesN::from_array(
                    &env,
                    &signer3.sign(payload.to_array().as_slice()).to_bytes()
                ),
            }
            .into_val(&env),
            &vec![&env],
        ),
        Err(Ok(Error::UnknownSigner))
    );
}

#[test]
fn test_1of2_failed_verification() {
    let env = Env::default();

    // Generate signing keypairs.
    let signer1 = Keypair::generate(&mut thread_rng());
    let signer2 = Keypair::generate(&mut thread_rng());

    // Register the account contract, passing in the two signers (public keys) to the constructor.
    let contract_id = env.register(
        Contract,
        (vec![
            &env,
            BytesN::from_array(&env, &signer1.public.to_bytes()),
            BytesN::from_array(&env, &signer2.public.to_bytes()),
        ],),
    );

    let payload = BytesN::random(&env);

    assert_eq!(
        env.try_invoke_contract_check_auth::<Error>(
            &contract_id,
            &payload,
            Signature {
                // ❗️Claims to be signer1.
                public_key: BytesN::from_array(&env, &signer1.public.to_bytes()),
                signature: BytesN::from_array(
                    &env,
                    // ❗️Signature is not a valid signer1 signature, it's a signer2.
                    &signer2.sign(payload.to_array().as_slice()).to_bytes()
                ),
            }
            .into_val(&env),
            &vec![&env],
        ),
        Err(Err(InvokeError::Abort))
    );
}

#[test]
fn test_1of2_invalid_signature_structure() {
    let env = Env::default();

    // Generate signing keypairs.
    let signer1 = Keypair::generate(&mut thread_rng());
    let signer2 = Keypair::generate(&mut thread_rng());

    // Register the account contract, passing in the two signers (public keys) to the constructor.
    let contract_id = env.register(
        Contract,
        (vec![
            &env,
            BytesN::from_array(&env, &signer1.public.to_bytes()),
            BytesN::from_array(&env, &signer2.public.to_bytes()),
        ],),
    );

    let payload = BytesN::random(&env);

    assert_eq!(
        env.try_invoke_contract_check_auth::<Error>(
            &contract_id,
            &payload,
            // ❗️Signature is not a valid signature structure, instead of being the two
            // components a public key and a signature, it is just the signature.
            BytesN::from_array(
                &env,
                &signer1.sign(payload.to_array().as_slice()).to_bytes()
            )
            .into_val(&env),
            &vec![&env],
        ),
        Err(Err(InvokeError::Abort))
    );
}
