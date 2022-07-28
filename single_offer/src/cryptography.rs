use soroban_sdk::{serde::Serialize, Account, BigInt, Env, EnvVal};

use soroban_token_contract::public_types::{
    KeyedAccountAuthorization, KeyedAuthorization, KeyedEd25519Authorization, Message, MessageV0,
    U256,
};

use crate::DataKey;

// Nonce management
pub fn read_nonce(e: &Env) -> BigInt {
    let key = DataKey::Nonce;
    if let Some(nonce) = e.contract_data().get(key) {
        nonce.unwrap()
    } else {
        BigInt::zero(&e)
    }
}

pub fn read_and_increment_nonce(e: &Env) -> BigInt {
    let key = DataKey::Nonce;
    let nonce = read_nonce(e);
    e.contract_data()
        .set(key, nonce.clone() + BigInt::from_u32(e, 1));
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

    e.verify_sig_ed25519(auth.public_key.into(), msg_bin, auth.signature.into());
}

fn check_account_auth(
    e: &Env,
    auth: KeyedAccountAuthorization,
    domain: Domain,
    parameters: EnvVal,
) {
    let account = Account::from_public_key(&auth.public_key).unwrap();

    let msg = MessageV0 {
        nonce: read_and_increment_nonce(&e),
        domain: domain as u32,
        parameters: parameters.try_into().unwrap(),
    };
    let msg_bin = Message::V0(msg).serialize(e);

    let threshold = account.medium_threshold();
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
            sig.public_key.clone().into(),
            msg_bin.clone(),
            sig.signature.into(),
        );
        // TODO: Check for overflow
        weight += account.signer_weight(&sig.public_key);

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
