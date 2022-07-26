use stellar_contract_sdk::{serde::Serialize, BigInt, Env, EnvVal};

use stellar_token_contract::public_types::{
    KeyedAccountAuthorization, KeyedAuthorization, KeyedEd25519Authorization, Message, MessageV0,
    U256,
};

use crate::DataKey;

// Nonce management
pub fn read_nonce(e: &Env) -> BigInt {
    let key = DataKey::Nonce;
    if e.has_contract_data(key.clone()) {
        e.get_contract_data(key.clone())
    } else {
        BigInt::from_u32(e, 0)
    }
}

pub fn read_and_increment_nonce(e: &Env) -> BigInt {
    let key = DataKey::Nonce;
    let nonce = read_nonce(e);
    e.put_contract_data(key, nonce.clone() + BigInt::from_u32(e, 1));
    nonce
}

#[repr(u32)]
pub enum Domain {
    Withdraw = 0,
    UpdatePrice = 1,
}

fn check_ed25519_auth(
    e: &Env,
    auth: KeyedEd25519Authorization,
    domain: Domain,
    parameters: EnvVal,
) {
    let msg = MessageV0 {
        nonce: read_and_increment_nonce(&e),
        domain: domain as u32,
        parameters: parameters.try_into().unwrap(),
    };
    let msg_bin = Message::V0(msg).serialize(e);

    e.verify_sig_ed25519(auth.signature.into(), auth.public_key.into(), msg_bin);
}

fn check_account_auth(
    e: &Env,
    auth: KeyedAccountAuthorization,
    domain: Domain,
    parameters: EnvVal,
) {
    use stellar_contract_sdk::Binary;
    let acc_id: Binary = auth.public_key.clone().into();

    let msg = MessageV0 {
        nonce: read_and_increment_nonce(&e),
        domain: domain as u32,
        parameters: parameters.try_into().unwrap(),
    };
    let msg_bin = Message::V0(msg).serialize(e);

    let threshold = e.account_get_medium_threshold(acc_id.clone());
    let mut weight = 0u32;

    let sigs = &auth.auth.signatures;
    let mut prev_pk: Option<U256> = None;
    for sig in sigs.iter().map(Result::unwrap) {
        // Cannot take multiple signatures from the same key
        if let Some(prev) = prev_pk {
            if prev >= sig.public_key {
                panic!()
            }
        }

        e.verify_sig_ed25519(
            sig.signature.into(),
            sig.public_key.clone().into(),
            msg_bin.clone(),
        );
        // TODO: Check for overflow
        weight += e.account_get_signer_weight(acc_id.clone(), sig.public_key.clone().into());

        prev_pk = Some(sig.public_key);
    }

    if weight < threshold {
        panic!()
    }
}

pub fn check_auth(e: &Env, auth: KeyedAuthorization, domain: Domain, parameters: EnvVal) {
    match auth {
        KeyedAuthorization::Contract => {
            e.get_invoking_contract();
        }
        KeyedAuthorization::Ed25519(kea) => check_ed25519_auth(e, kea, domain, parameters),
        KeyedAuthorization::Account(kaa) => check_account_auth(e, kaa, domain, parameters),
    }
}
