#![cfg(test)]

use super::*;
use soroban_sdk::token::TokenClient;
use soroban_sdk::Address;
use soroban_sdk::{bytesn, testutils::Address as _, vec, Env};
use token::StellarAssetClient as TokenAdminClient;

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let sac = e.register_stellar_asset_contract_v2(admin.clone());
    (
        token::Client::new(e, &sac.address()),
        token::StellarAssetClient::new(e, &sac.address()),
    )
}

#[test]
fn test_valid_claim() {
    let env = Env::default();
    env.mock_all_auths_allowing_non_root_auth();

    let token_admin = Address::generate(&env);
    let (token, token_admin_client) = create_token_contract(&env, &token_admin);
    token_admin_client.mint(&token_admin_client.address, &1000);

    let contract_id = env.register(
        MerkleDistributionContract,
        MerkleDistributionContractArgs::__constructor(
            &bytesn!(
                &env,
                0x11932105f1a4d0092e87cead3a543da5afd8adcff63f9a8ceb6c5db3c8135722
            ),
            &token.address,
            &1000,
            &token_admin_client.address,
        ),
    );
    let client = MerkleDistributionContractClient::new(&env, &contract_id);

    let receiver = Address::from_str(
        &env,
        "CAASCQKVVBSLREPEUGPOTQZ4BC2NDBY2MW7B2LGIGFUPIY4Z3XUZRVTX",
    );
    let amount = 100;
    let proofs = vec![
        &env,
        bytesn!(
            &env,
            0xfc0d9c2f46c1e910bd3af8665318714c7c97486d2a206f96236c6e7e50c080d7
        ),
        bytesn!(
            &env,
            0xc83f7b26055572e5e84c78ec4d4f45b85b71698951077baafe195279c1f30be4
        ),
    ];

    client.claim(&3_u32, &receiver, &amount, &proofs);
    assert_eq!(token.balance(&receiver), 100);
    assert_eq!(token.balance(&contract_id), 900);
    assert!(env.auths().is_empty());
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_double_claim() {
    let env: Env = Env::default();
    env.mock_all_auths_allowing_non_root_auth();

    let token_admin = Address::generate(&env);
    let (token, token_admin_client) = create_token_contract(&env, &token_admin);
    token_admin_client.mint(&token_admin_client.address, &1000);

    let contract_id = env.register(
        MerkleDistributionContract,
        MerkleDistributionContractArgs::__constructor(
            &bytesn!(
                &env,
                0x11932105f1a4d0092e87cead3a543da5afd8adcff63f9a8ceb6c5db3c8135722
            ),
            &token.address,
            &1000,
            &token_admin_client.address,
        ),
    );
    let client = MerkleDistributionContractClient::new(&env, &contract_id);

    let receiver = Address::from_str(
        &env,
        "CAASCQKVVBSLREPEUGPOTQZ4BC2NDBY2MW7B2LGIGFUPIY4Z3XUZRVTX",
    );
    let amount: i128 = 100;
    let proofs = vec![
        &env,
        bytesn!(
            &env,
            0xfc0d9c2f46c1e910bd3af8665318714c7c97486d2a206f96236c6e7e50c080d7
        ),
        bytesn!(
            &env,
            0xc83f7b26055572e5e84c78ec4d4f45b85b71698951077baafe195279c1f30be4
        ),
    ];

    client.claim(&3_u32, &receiver, &amount, &proofs);
    client.claim(&3_u32, &receiver, &amount, &proofs);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_bad_claim() {
    let env = Env::default();
    env.mock_all_auths_allowing_non_root_auth();

    let token_admin = Address::generate(&env);
    let (token, token_admin_client) = create_token_contract(&env, &token_admin);
    token_admin_client.mint(&token_admin_client.address, &1000);

    let contract_id = env.register(
        MerkleDistributionContract,
        MerkleDistributionContractArgs::__constructor(
            &bytesn!(
                &env,
                0x11932105f1a4d0092e87cead3a543da5afd8adcff63f9a8ceb6c5db3c8135722
            ),
            &token.address,
            &1000,
            &token_admin_client.address,
        ),
    );
    let client = MerkleDistributionContractClient::new(&env, &contract_id);

    let receiver = Address::from_str(
        &env,
        "CAASCQKVVBSLREPEUGPOTQZ4BC2NDBY2MW7B2LGIGFUPIY4Z3XUZRVTX",
    );
    let amount = 100000; // This is a different amount
    let proofs = vec![
        &env,
        bytesn!(
            &env,
            0xfc0d9c2f46c1e910bd3af8665318714c7c97486d2a206f96236c6e7e50c080d7
        ),
        bytesn!(
            &env,
            0xc83f7b26055572e5e84c78ec4d4f45b85b71698951077baafe195279c1f30be4
        ),
    ];

    client.claim(&3_u32, &receiver, &amount, &proofs);
}
