use soroban_auth::{AccountSignatures, Ed25519Signature, Signature};
use soroban_sdk::serde::Serialize;
use soroban_sdk::{contracttype, Account, Bytes, BytesN, Env, RawVal, Symbol, Vec};

pub trait PayloadTrait {
    fn payload(
        e: Env,
        function: Symbol,
        args: Vec<RawVal>,
        callstack: Vec<(BytesN<32>, Symbol)>,
    ) -> SignaturePayload;
}

#[derive(Clone)]
#[contracttype]
pub struct SignaturePayloadV0 {
    pub function: Symbol,
    pub contract: BytesN<32>,
    pub network: Bytes,
    pub args: Vec<RawVal>,
    pub salt: RawVal,
}

#[derive(Clone)]
#[contracttype]
pub enum SignaturePayload {
    V0(SignaturePayloadV0),
}

fn check_ed25519_auth(env: &Env, auth: &Ed25519Signature, payload: SignaturePayload) {
    let msg_bytes = payload.serialize(env);
    env.verify_sig_ed25519(auth.public_key.clone(), msg_bytes, auth.signature.clone());
}

fn check_account_auth(env: &Env, auth: &AccountSignatures, payload: SignaturePayload) {
    let msg_bytes = payload.serialize(env);

    let acc = Account::from_public_key(&auth.account_id).unwrap();
    let threshold = acc.medium_threshold();
    let mut weight = 0u32;

    let sigs = &auth.signatures;
    let mut prev_pk: Option<BytesN<32>> = None;
    for sig in sigs.iter().map(Result::unwrap) {
        // Cannot take multiple signatures from the same key
        if let Some(prev) = prev_pk {
            if prev == sig.public_key {
                panic!("signature duplicate")
            }
            if prev > sig.public_key {
                panic!("signature out of order")
            }
        }

        env.verify_sig_ed25519(sig.public_key.clone(), msg_bytes.clone(), sig.signature);

        weight = weight
            .checked_add(acc.signer_weight(&sig.public_key))
            .expect("weight overflow");

        prev_pk = Some(sig.public_key);
    }

    if weight < threshold {
        panic!("insufficient signing weight")
    }
}

/// Verifies a Signature. It's important to note that this module does
/// not provide replay protection. That will need to be implemented by
/// the user.
pub fn check_auth<T: PayloadTrait>(
    env: &Env,
    sig: &Signature,
    function: Symbol,
    args: Vec<RawVal>,
) {
    match sig {
        Signature::Contract => {
            env.get_invoking_contract();
        }
        Signature::Ed25519(kea) => {
            let payload = T::payload(env.clone(), function, args, env.get_current_call_stack());
            check_ed25519_auth(env, &kea, payload);
        }
        Signature::Account(kaa) => {
            let payload = T::payload(env.clone(), function, args, env.get_current_call_stack());
            check_account_auth(env, &kaa, payload);
        }
    }
}
