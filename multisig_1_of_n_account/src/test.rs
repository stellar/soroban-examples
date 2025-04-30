#![cfg(test)]
extern crate std;

use ed25519_dalek::{Keypair, Signer};
use rand::thread_rng;
use soroban_sdk::{testutils::BytesN as _, vec, BytesN, Env, IntoVal};

use crate::{Contract, Error, Signature};

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    let signer1 = Keypair::generate(&mut thread_rng());
    let signer2 = Keypair::generate(&mut thread_rng());

    let contract_id = env.register(
        Contract,
        (vec![
            &env,
            BytesN::from_array(&env, &signer1.public.to_bytes()),
            BytesN::from_array(&env, &signer2.public.to_bytes()),
        ],),
    );

    let payload = BytesN::random(&env);

    // `__check_auth` can't be called directly, hence we need to use
    // `try_invoke_contract_check_auth` testing utility that emulates being
    // called by the Soroban host during a `require_auth` call.
    assert_eq!(
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
        Ok(())
    )
}
