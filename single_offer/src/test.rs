#![cfg(test)]

use crate::external as single_offer;
use ed25519_dalek::Keypair;
use rand::{thread_rng, RngCore};
use stellar_contract_sdk::Env;
use stellar_token_contract::external as token;
use token::{Identifier, MessageWithoutNonce as TokenContractFn, U256};

fn generate_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}

fn make_auth(kp: &Keypair, msg: &token::Message) -> token::Authorization {
    use token::{Authorization, Ed25519Authorization};
    let signature = msg.sign(kp).unwrap();
    Authorization::Ed25519(Ed25519Authorization {
        nonce: msg.0.clone(),
        signature,
    })
}

fn make_keyed_auth(kp: &Keypair, msg: &token::Message) -> token::KeyedAuthorization {
    use token::{Ed25519Authorization, KeyedAuthorization, KeyedEd25519Authorization};
    let signature = msg.sign(kp).unwrap();
    KeyedAuthorization::Ed25519(KeyedEd25519Authorization {
        public_key: kp.public.to_bytes(),
        auth: Ed25519Authorization {
            nonce: msg.0.clone(),
            signature,
        },
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

fn create_single_offer_contract(
    e: &mut Env,
    admin: &Keypair,
    token_a: &U256,
    token_b: &U256,
    n: u32,
    d: u32,
) -> U256 {
    let mut id: U256 = Default::default();
    thread_rng().fill_bytes(&mut id);
    single_offer::register_test_contract(&e, &id);
    single_offer::initialize(
        e,
        &id,
        Identifier::Ed25519(admin.public.to_bytes()),
        token_a,
        token_b,
        n,
        d,
    );
    id
}

#[test]
fn test() {
    let mut e = Env::with_empty_recording_storage();

    let admin_a = generate_keypair();
    let admin_b = generate_keypair();
    let id1 = generate_keypair();
    let id2 = generate_keypair();

    let token_a = create_token_contract(&mut e, &admin_a);
    let token_b = create_token_contract(&mut e, &admin_b);

    // The price here is 1 A == .5 B
    let single_offer = create_single_offer_contract(&mut e, &id1, &token_a, &token_b, 1, 2);

    // MINT A for id1
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

    // MINT B for id2
    sign_ed25519_then_do(
        &mut e,
        &token_b,
        &admin_b,
        TokenContractFn::Mint(Identifier::Ed25519(id2.public.to_bytes()), 1000u64.into()),
    );
    assert_eq!(
        token::balance(
            &mut e,
            &token_b,
            &Identifier::Ed25519(id2.public.to_bytes())
        ),
        1000u64.into()
    );

    // TRANSFER 100 A FROM ID1 TO CONTRACT
    sign_ed25519_then_do(
        &mut e,
        &token_a,
        &id1,
        TokenContractFn::Transfer(Identifier::Contract(single_offer), 100u64.into()),
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
        token::balance(&mut e, &token_a, &Identifier::Contract(single_offer)),
        100u64.into()
    );

    // TRANSFER 10 B FROM ID2 TO CONTRACT
    sign_ed25519_then_do(
        &mut e,
        &token_b,
        &id2,
        TokenContractFn::Transfer(Identifier::Contract(single_offer), 10u64.into()),
    );
    assert_eq!(
        token::balance(
            &mut e,
            &token_b,
            &Identifier::Ed25519(id2.public.to_bytes())
        ),
        990.into()
    );
    assert_eq!(
        token::balance(&mut e, &token_b, &Identifier::Contract(single_offer)),
        10u64.into()
    );

    // TRADE WILL SEND 10 B FROM CONTRACT TO ID1, AND 20 A FROM CONTRACT TO ID2
    single_offer::trade(
        &mut e,
        &single_offer,
        Identifier::Ed25519(id2.public.to_bytes()),
        20,
    );

    // VALIDATE A BALANCES
    assert_eq!(
        token::balance(
            &mut e,
            &token_a,
            &Identifier::Ed25519(id1.public.to_bytes())
        ),
        900.into()
    );
    assert_eq!(
        token::balance(
            &mut e,
            &token_a,
            &Identifier::Ed25519(id2.public.to_bytes())
        ),
        20.into()
    );
    assert_eq!(
        token::balance(&mut e, &token_a, &Identifier::Contract(single_offer)),
        80u64.into()
    );

    // VALIDATE B BALANCES
    assert_eq!(
        token::balance(
            &mut e,
            &token_b,
            &Identifier::Ed25519(id1.public.to_bytes())
        ),
        10.into()
    );
    assert_eq!(
        token::balance(
            &mut e,
            &token_b,
            &Identifier::Ed25519(id2.public.to_bytes())
        ),
        990.into()
    );
    assert_eq!(
        token::balance(&mut e, &token_b, &Identifier::Contract(single_offer)),
        0u64.into()
    );

    single_offer::withdraw(&mut e, &single_offer);

    assert_eq!(
        token::balance(
            &mut e,
            &token_a,
            &Identifier::Ed25519(id1.public.to_bytes())
        ),
        980.into()
    );
    assert_eq!(
        token::balance(
            &mut e,
            &token_a,
            &Identifier::Ed25519(id2.public.to_bytes())
        ),
        20.into()
    );
    assert_eq!(
        token::balance(&mut e, &token_a, &Identifier::Contract(single_offer)),
        0u64.into()
    );
}
