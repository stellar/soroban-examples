#![cfg(test)]

use crate::external as liqpool;
use ed25519_dalek::Keypair;
use rand::{thread_rng, RngCore};
use stellar_contract_sdk::Env;
use stellar_token_contract::external as token;
use token::{Identifier, MessageWithoutNonce as TokenContractFn, U256};

fn generate_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}

fn make_auth(kp: &Keypair, msg: &token::Message) -> token::Authorization {
    let signature = msg.sign(kp).unwrap();
    token::Authorization::Ed25519(signature)
}

fn make_keyed_auth(kp: &Keypair, msg: &token::Message) -> token::KeyedAuthorization {
    use token::{KeyedAuthorization, KeyedEd25519Authorization};
    let signature = msg.sign(kp).unwrap();
    KeyedAuthorization::Ed25519(KeyedEd25519Authorization {
        public_key: kp.public.to_bytes(),
        signature,
    })
}

fn sign_ed25519_then_do(e: &mut Env, contract_id: &[u8; 32], kp: &Keypair, cf: TokenContractFn) {
    let nonce = token::nonce(e, contract_id, &(Identifier::Ed25519(kp.public.to_bytes())));
    let msg = token::Message(nonce, cf);
    match &msg.1 {
        TokenContractFn::Approve(id, amount) => {
            token::approve(e, contract_id, &make_keyed_auth(kp, &msg), id, amount);
        }
        TokenContractFn::Transfer(to, amount) => {
            token::xfer(e, contract_id, &make_keyed_auth(kp, &msg), to, amount);
        }
        TokenContractFn::TransferFrom(from, to, amount) => {
            token::xfer_from(e, contract_id, &make_keyed_auth(kp, &msg), from, to, amount);
        }
        TokenContractFn::Burn(from, amount) => {
            token::burn(e, contract_id, &make_auth(kp, &msg), from, amount);
        }
        TokenContractFn::Freeze(id) => {
            token::freeze(e, contract_id, &make_auth(kp, &msg), id);
        }
        TokenContractFn::Mint(to, amount) => {
            token::mint(e, contract_id, &make_auth(kp, &msg), to, amount);
        }
        TokenContractFn::SetAdministrator(id) => {
            token::set_admin(e, contract_id, &make_auth(kp, &msg), id);
        }
        TokenContractFn::Unfreeze(id) => {
            token::unfreeze(e, contract_id, &make_auth(kp, &msg), id);
        }
    }
}

fn create_token_contract(e: &mut Env, admin: &Keypair) -> U256 {
    let mut id: U256 = Default::default();
    thread_rng().fill_bytes(&mut id);
    token::register_test_contract(&e, &id);
    token::initialize(e, &id, &Identifier::Ed25519(admin.public.to_bytes()));
    id
}

fn create_liqpool_contract(e: &mut Env, token_a: &U256, token_b: &U256) -> U256 {
    let mut id: U256 = Default::default();
    thread_rng().fill_bytes(&mut id);
    liqpool::register_test_contract(&e, &id);
    liqpool::initialize(e, &id, token_a, token_b);
    id
}

#[test]
fn test() {
    let mut e = Env::with_empty_recording_storage();

    let mut admin_a = generate_keypair();
    let mut admin_b = generate_keypair();
    let id1 = generate_keypair();

    let mut token_a = create_token_contract(&mut e, &admin_a);
    let mut token_b = create_token_contract(&mut e, &admin_b);
    if token_a > token_b {
        (token_a, token_b) = (token_b, token_a);
        (admin_a, admin_b) = (admin_b, admin_a);
    }
    let liqpool = create_liqpool_contract(&mut e, &token_a, &token_b);
    let token_share = liqpool::share_id(&mut e, &liqpool);

    sign_ed25519_then_do(
        &mut e,
        &token_a,
        &admin_a,
        TokenContractFn::Mint(Identifier::Ed25519(id1.public.to_bytes()), 1000u64.into()),
    );
    assert_eq!(
        token::balance(
            &mut e,
            &token_a,
            &Identifier::Ed25519(id1.public.to_bytes())
        ),
        1000u64.into()
    );
    sign_ed25519_then_do(
        &mut e,
        &token_b,
        &admin_b,
        TokenContractFn::Mint(Identifier::Ed25519(id1.public.to_bytes()), 1000u64.into()),
    );
    assert_eq!(
        token::balance(
            &mut e,
            &token_b,
            &Identifier::Ed25519(id1.public.to_bytes())
        ),
        1000u64.into()
    );

    sign_ed25519_then_do(
        &mut e,
        &token_a,
        &id1,
        TokenContractFn::Transfer(Identifier::Contract(liqpool), 100u64.into()),
    );
    assert_eq!(
        token::balance(
            &mut e,
            &token_a,
            &Identifier::Ed25519(id1.public.to_bytes())
        ),
        900u64.into()
    );
    assert_eq!(
        token::balance(&mut e, &token_a, &Identifier::Contract(liqpool.clone())),
        100u64.into()
    );
    sign_ed25519_then_do(
        &mut e,
        &token_b,
        &id1,
        TokenContractFn::Transfer(Identifier::Contract(liqpool), 100u64.into()),
    );
    assert_eq!(
        token::balance(
            &mut e,
            &token_b,
            &Identifier::Ed25519(id1.public.to_bytes())
        ),
        900u64.into()
    );
    assert_eq!(
        token::balance(&mut e, &token_b, &Identifier::Contract(liqpool.clone())),
        100u64.into()
    );
    liqpool::deposit(
        &mut e,
        &liqpool,
        &Identifier::Ed25519(id1.public.to_bytes()),
    );
    assert_eq!(
        token::balance(
            &mut e,
            &token_share,
            &Identifier::Ed25519(id1.public.to_bytes()),
        ),
        100u64.into()
    );

    sign_ed25519_then_do(
        &mut e,
        &token_a,
        &id1,
        TokenContractFn::Transfer(Identifier::Contract(liqpool), 100u64.into()),
    );
    assert_eq!(
        token::balance(
            &mut e,
            &token_a,
            &Identifier::Ed25519(id1.public.to_bytes())
        ),
        800u64.into()
    );
    assert_eq!(
        token::balance(&mut e, &token_a, &Identifier::Contract(liqpool.clone())),
        200u64.into()
    );
    liqpool::swap(
        &mut e,
        &liqpool,
        &Identifier::Ed25519(id1.public.to_bytes()),
        &0u64.into(),
        &49.into(),
    );
    assert_eq!(
        token::balance(
            &mut e,
            &token_a,
            &Identifier::Ed25519(id1.public.to_bytes())
        ),
        800u64.into()
    );
    assert_eq!(
        token::balance(&mut e, &token_a, &Identifier::Contract(liqpool.clone())),
        200u64.into()
    );
    assert_eq!(
        token::balance(
            &mut e,
            &token_b,
            &Identifier::Ed25519(id1.public.to_bytes())
        ),
        949u64.into()
    );
    assert_eq!(
        token::balance(&mut e, &token_b, &Identifier::Contract(liqpool.clone())),
        51u64.into()
    );

    sign_ed25519_then_do(
        &mut e,
        &token_share,
        &id1,
        TokenContractFn::Transfer(Identifier::Contract(liqpool), 100u64.into()),
    );
    assert_eq!(
        token::balance(
            &mut e,
            &token_share,
            &Identifier::Ed25519(id1.public.to_bytes()),
        ),
        0u64.into()
    );
    assert_eq!(
        token::balance(&mut e, &token_share, &Identifier::Contract(liqpool.clone()),),
        100u64.into()
    );
    liqpool::withdraw(
        &mut e,
        &liqpool,
        &Identifier::Ed25519(id1.public.to_bytes()),
    );
    assert_eq!(
        token::balance(
            &mut e,
            &token_a,
            &Identifier::Ed25519(id1.public.to_bytes())
        ),
        1000u64.into()
    );
    assert_eq!(
        token::balance(
            &mut e,
            &token_b,
            &Identifier::Ed25519(id1.public.to_bytes())
        ),
        1000u64.into()
    );
}
