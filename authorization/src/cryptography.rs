use soroban_sdk::{serde::Serialize, Account, BigInt, Env, EnvVal, Symbol};

use crate::{
    public_types::{
        Identifier, KeyedAccountAuthorization, KeyedAuthorization, KeyedEd25519Signature, Message,
        MessageV0, U256,
    },
    DataKey,
};

// Nonce management
pub fn read_nonce(e: &Env, id: Identifier) -> BigInt {
    let key = DataKey::Nonce(id);
    if let Some(nonce) = e.contract_data().get(key) {
        nonce.unwrap()
    } else {
        BigInt::zero(e)
    }
}

pub fn read_and_increment_nonce(e: &Env, id: Identifier) -> BigInt {
    let key = DataKey::Nonce(id.clone());
    let nonce = read_nonce(e, id);
    e.contract_data()
        .set(key, nonce.clone() + BigInt::from_u32(e, 1));
    nonce
}

fn check_ed25519_auth(
    e: &Env,
    auth: &KeyedEd25519Signature,
    nonce: BigInt,
    function: Symbol,
    parameters: EnvVal,
) {
    let stored_nonce = read_and_increment_nonce(&e, Identifier::Ed25519(auth.public_key.clone()));
    if nonce != stored_nonce {
        panic!("incorrect nonce")
    }

    let msg = MessageV0 {
        function,
        parameters: parameters.try_into().unwrap(),
    };
    let msg_bin = Message::V0(msg).serialize(e);

    e.verify_sig_ed25519(
        auth.public_key.clone().into(),
        msg_bin,
        auth.signature.clone().into(),
    );
}

fn check_account_auth(
    e: &Env,
    auth: &KeyedAccountAuthorization,
    nonce: BigInt,
    function: Symbol,
    parameters: EnvVal,
) {
    let stored_nonce = read_and_increment_nonce(&e, Identifier::Account(auth.clone().public_key));
    if nonce != stored_nonce {
        panic!("incorrect nonce")
    }

    let acc = Account::from_public_key(&auth.public_key).unwrap();

    let msg = MessageV0 {
        function,
        parameters: parameters.try_into().unwrap(),
    };
    let msg_bin = Message::V0(msg).serialize(e);

    let threshold = acc.medium_threshold();
    let mut weight = 0u32;

    let sigs = &auth.signatures;
    let mut prev_pk: Option<U256> = None;
    for sig in sigs.iter().map(Result::unwrap) {
        // Cannot take multiple signatures from the same key
        if let Some(prev) = prev_pk {
            if prev >= sig.public_key {
                panic!("signature out of order")
            }
        }

        e.verify_sig_ed25519(
            sig.public_key.clone().into(),
            msg_bin.clone(),
            sig.signature.into(),
        );
        // TODO: Check for overflow
        weight += acc.signer_weight(&sig.public_key);

        prev_pk = Some(sig.public_key);
    }

    if weight < threshold {
        panic!("insufficient signing weight")
    }
}

// Note that nonce is not used by KeyedAuthorization::Contract
pub fn check_auth(
    e: &Env,
    auth: &KeyedAuthorization,
    nonce: BigInt,
    function: Symbol,
    parameters: EnvVal,
) {
    match auth {
        KeyedAuthorization::Contract => {
            e.get_invoking_contract();
        }
        KeyedAuthorization::Ed25519(kea) => {
            check_ed25519_auth(e, kea, nonce.clone(), function, parameters)
        }
        KeyedAuthorization::Account(kaa) => {
            check_account_auth(e, kaa, nonce.clone(), function, parameters)
        }
    }
}
