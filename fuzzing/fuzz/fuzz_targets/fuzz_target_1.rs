//! A very simple fuzz test of the timelock contract.
//!
//! This makes one deposit of arbitrary amount,
//! then makes one claim of arbitrary amount.
//! Between each step it makes assertions about the contract state.

#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_fuzzing_contract::*;
use soroban_sdk::testutils::{
    arbitrary::{arbitrary, Arbitrary},
    Address as _, Ledger, LedgerInfo,
};
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::token::StellarAssetClient as TokenAdminClient;
use soroban_sdk::{vec, Address, Env};

#[derive(Arbitrary, Debug)]
struct Input {
    deposit_amount: i128,
    claim_amount: i128,
}

fuzz_target!(|input: Input| {
    let env = Env::default();

    env.mock_all_auths();

    env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 10,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: u32::MAX,
        min_persistent_entry_ttl: u32::MAX,
        max_entry_ttl: u32::MAX,
    });

    // Turn off the CPU/memory budget for testing.
    env.cost_estimate().budget().reset_unlimited();

    let depositor_address = Address::generate(&env);
    let claimant_address = Address::generate(&env);
    let token_admin = Address::generate(&env);

    let sac = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_contract_id = sac.address();
    let token_client = TokenClient::new(&env, &token_contract_id);
    let token_admin_client = TokenAdminClient::new(&env, &token_contract_id);

    let timelock_contract_id = env.register(ClaimableBalanceContract, ());
    let timelock_client = ClaimableBalanceContractClient::new(&env, &timelock_contract_id);

    token_admin_client.mint(&depositor_address, &i128::max_value());

    // Deposit, then assert invariants.
    {
        let _ = timelock_client.try_deposit(
            &depositor_address,
            &token_contract_id,
            &input.deposit_amount,
            &vec![&env, claimant_address.clone()],
            &TimeBound {
                kind: TimeBoundKind::Before,
                timestamp: 123456,
            },
        );

        assert_invariants(&env, &timelock_contract_id, &token_client, &input);
    }

    // Claim, then assert invariants.
    {
        let _ = timelock_client.try_claim(&claimant_address, &input.claim_amount);

        assert_invariants(&env, &timelock_contract_id, &token_client, &input);
    }
});

/// Directly inspect the contract state and make assertions about it.
fn assert_invariants(
    env: &Env,
    timelock_contract_id: &Address,
    token_client: &TokenClient,
    input: &Input,
) {
    // Configure the environment to access the timelock contract's storage.
    env.as_contract(timelock_contract_id, || {
        let storage = env.storage().persistent();

        // Get the two datums owned by the timelock contract.
        let is_initialized = storage.has(&DataKey::Init);
        let claimable_balance = storage.get::<_, ClaimableBalance>(&DataKey::Balance);

        // Call the token client to get the balance held in the timelock contract.
        // This consumes contract execution budget.
        let actual_token_balance = token_client.balance(timelock_contract_id);

        // There can only be a claimaible balance after the contract is initialized,
        // but once the balance is claimed there is no balance,
        // but the contract remains initialized.
        // This is a truth table of valid states.
        assert!(match (is_initialized, claimable_balance.is_some()) {
            (false, false) => true,
            (false, true) => false,
            (true, true) => true,
            (true, false) => true,
        });

        assert!(actual_token_balance >= 0);

        if let Some(claimable_balance) = claimable_balance {
            assert!(claimable_balance.amount > 0);
            assert!(claimable_balance.amount <= input.deposit_amount);
            assert_eq!(claimable_balance.amount, actual_token_balance);

            assert!(claimable_balance.claimants.len() > 0);
        }
    });
}
