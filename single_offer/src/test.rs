#![cfg(test)]

use crate::testutils::{register_test_contract as register_single_offer, SingleOffer};
use crate::token::{self, Identifier, Signature, TokenMetadata};
use soroban_sdk::{testutils::Accounts, AccountId, BytesN, Env, IntoVal};

fn create_token_contract(e: &Env, admin: &AccountId) -> token::Client {
    let token = token::Client::new(e, &e.register_contract_token());
    // decimals, name, symbol don't matter in tests
    token.init(
        &Identifier::Account(admin.clone()),
        &TokenMetadata {
            name: "name".into_val(e),
            symbol: "symbol".into_val(e),
            decimals: 7,
        },
    );
    token
}

fn create_single_offer_contract(
    e: &Env,
    admin: &AccountId,
    token_a: &BytesN<32>,
    token_b: &BytesN<32>,
    n: u32,
    d: u32,
) -> SingleOffer {
    let single_offer = SingleOffer::new(e, &register_single_offer(e));
    single_offer.initialize(&Identifier::Account(admin.clone()), token_a, token_b, n, d);
    single_offer
}

#[test]
fn test() {
    let e: Env = Default::default();
    let admin1 = e.accounts().generate();
    let admin2 = e.accounts().generate();
    let user1 = e.accounts().generate();
    let user2 = e.accounts().generate();
    let user1_id = Identifier::Account(user1.clone());
    let user2_id = Identifier::Account(user2.clone());

    let token1 = create_token_contract(&e, &admin1);
    let token2 = create_token_contract(&e, &admin2);

    // The price here is 1 A == .5 B
    let offer =
        create_single_offer_contract(&e, &user1, &token1.contract_id, &token2.contract_id, 1, 2);
    let offer_id = Identifier::Contract(offer.contract_id.clone());

    token1
        .with_source_account(&admin1)
        .mint(&Signature::Invoker, &0, &user1_id, &1000);

    assert_eq!(token1.balance(&user1_id), 1000);
    token2
        .with_source_account(&admin2)
        .mint(&Signature::Invoker, &0, &user2_id, &1000);
    assert_eq!(token2.balance(&user2_id), 1000);

    // Deposit 100 token1 into offer
    token1
        .with_source_account(&user1)
        .xfer(&Signature::Invoker, &0, &offer_id, &100);
    assert_eq!(token1.balance(&user1_id), 900);
    assert_eq!(token1.balance(&offer_id), 100);

    // Trade 10 token2 for 20 token1
    token2
        .with_source_account(&user2)
        .xfer(&Signature::Invoker, &0, &offer_id, &10);
    assert_eq!(token2.balance(&user2_id), 990);
    assert_eq!(token2.balance(&offer_id), 10);
    offer.trade(&user2_id, &20);
    assert_eq!(token1.balance(&user1_id), 900);
    assert_eq!(token1.balance(&user2_id), 20);
    assert_eq!(token1.balance(&offer_id), 80);
    assert_eq!(token2.balance(&user1_id), 10);
    assert_eq!(token2.balance(&user2_id), 990);
    assert_eq!(token2.balance(&offer_id), 0);

    // Withdraw 70 token1 from offer
    offer.withdraw(&user1, &70);
    assert_eq!(token1.balance(&user1_id), 970);
    assert_eq!(token1.balance(&user2_id), 20);
    assert_eq!(token1.balance(&offer_id), 10);
    assert_eq!(token2.balance(&user1_id), 10);
    assert_eq!(token2.balance(&user2_id), 990);
    assert_eq!(token2.balance(&offer_id), 0);

    // The price here is 1 A = 1 B
    offer.updt_price(&user1, 1, 1);

    // Trade 10 token2 for 10 token1
    token2
        .with_source_account(&user2)
        .xfer(&Signature::Invoker, &0, &offer_id, &10);
    assert_eq!(token2.balance(&user2_id), 980);
    assert_eq!(token2.balance(&offer_id), 10);
    offer.trade(&user2_id, &10);
    assert_eq!(token1.balance(&user1_id), 970);
    assert_eq!(token1.balance(&user2_id), 30);
    assert_eq!(token1.balance(&offer_id), 00);
    assert_eq!(token2.balance(&user1_id), 20);
    assert_eq!(token2.balance(&user2_id), 980);
    assert_eq!(token2.balance(&offer_id), 0);
}
