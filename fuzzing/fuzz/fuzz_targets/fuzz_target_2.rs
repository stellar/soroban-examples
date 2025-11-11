//! An advanced fuzz test.
//!
//! This demonstrates use of the `SorabanArbitrary` trait,
//! and the advancement of time.

#![no_main]

use crate::arbitrary::Unstructured;
use libfuzzer_sys::fuzz_target;
use soroban_fuzzing_contract::*;
use soroban_ledger_snapshot::LedgerSnapshot;
use soroban_sdk::testutils::{
    arbitrary::{arbitrary, Arbitrary, SorobanArbitrary},
    Address as _, LedgerInfo,
};
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::token::StellarAssetClient as TokenAdminClient;
use soroban_sdk::xdr::ScAddress;
use soroban_sdk::{Address, Env, FromVal, Vec};
use std::vec::Vec as RustVec;

const NUM_ADDRESSES: usize = 2;

#[derive(Arbitrary, Debug)]
struct Input {
    addresses: [<Address as SorobanArbitrary>::Prototype; NUM_ADDRESSES],
    #[arbitrary(with = |u: &mut Unstructured| u.int_in_range(0..=i128::MAX))]
    token_mint: i128,
    steps: RustVec<Step>,
}

#[derive(Arbitrary, Debug)]
struct Step {
    #[arbitrary(with = |u: &mut Unstructured| u.int_in_range(1..=u64::MAX))]
    advance_time: u64,
    command: Command,
}

#[derive(Arbitrary, Debug)]
enum Command {
    Deposit(DepositCommand),
    Claim(ClaimCommand),
}

#[derive(Arbitrary, Debug)]
struct DepositCommand {
    #[arbitrary(with = |u: &mut Unstructured| u.int_in_range(0..=NUM_ADDRESSES - 1))]
    depositor_index: usize,
    amount: i128,
    // This is an ugly way to get a vector of integers in range
    #[arbitrary(with = |u: &mut Unstructured| {
        u.arbitrary_len::<usize>().map(|len| {
            (0..len).map(|_| {
                u.int_in_range(0..=NUM_ADDRESSES - 1)
            }).collect::<Result<RustVec<usize>, _>>()
        }).and_then(|inner_result| inner_result)
    })]
    claimant_indexes: RustVec<usize>,
    time_bound: <TimeBound as SorobanArbitrary>::Prototype,
}

#[derive(Arbitrary, Debug)]
struct ClaimCommand {
    #[arbitrary(with = |u: &mut Unstructured| u.int_in_range(0 ..= NUM_ADDRESSES - 1))]
    claimant_index: usize,
    amount: i128,
}

fuzz_target!(|input: Input| {
    let (config, mut prev_env) = Config::setup(input);

    for step in &config.input.steps {
        // Advance time and create a new env from snapshot.
        let curr_env = {
            let mut snapshot = prev_env.to_snapshot();
            snapshot.ledger.sequence_number += 1;
            snapshot.ledger.timestamp = snapshot.ledger.timestamp.saturating_add(step.advance_time);
            let env = Env::from_snapshot(snapshot);
            env.cost_estimate().budget().reset_unlimited();
            env
        };

        step.command.exec(&config, &curr_env);
        assert_invariants(&config, &prev_env, &curr_env);

        prev_env = curr_env;
    }
});

#[derive(Debug)]
struct Config {
    input: Input,
    token_contract_id: ScAddress,
    timelock_contract_id: ScAddress,
    deposit_info: Option<(Address, i128, <TimeBound as SorobanArbitrary>::Prototype)>,
}

impl Config {
    fn setup(input: Input) -> (Config, Env) {
        let snapshot = {
            let init_ledger = LedgerInfo {
                timestamp: 12345,
                protocol_version: 1,
                sequence_number: 10,
                network_id: Default::default(),
                base_reserve: 10,
                min_temp_entry_ttl: u32::MAX,
                min_persistent_entry_ttl: u32::MAX,
                max_entry_ttl: u32::MAX,
            };

            LedgerSnapshot::from(init_ledger, None)
        };

        let env = Env::from_ledger_snapshot(snapshot);

        env.mock_all_auths();

        let token_admin = Address::generate(&env);

        // This is a bit ugly - anticipate which deposit step will succeed
        // and store that information for making assertians later.
        let deposit_info = input.steps.iter().find_map(|step| match step.command {
            Command::Deposit(ref cmd)
                if cmd.amount > 0
                    && cmd.claimant_indexes.len() > 0
                    && cmd.claimant_indexes.len() <= 10 =>
            {
                let depositor_address =
                    Address::from_val(&env, &input.addresses[cmd.depositor_index]);
                Some((depositor_address, cmd.amount, cmd.time_bound.clone()))
            }
            _ => None,
        });

        let sac = env.register_stellar_asset_contract_v2(token_admin.clone());
        let token_contract_id = sac.address();
        let timelock_contract_id = env.register(ClaimableBalanceContract, ());

        if let Some((depositor_address, _, _)) = &deposit_info {
            let token_admin_client = TokenAdminClient::new(&env, &token_contract_id);
            token_admin_client.mint(&depositor_address, &input.token_mint);
        }

        let config = Config {
            input,
            token_contract_id: token_contract_id.try_into().unwrap(),
            timelock_contract_id: timelock_contract_id.try_into().unwrap(),
            deposit_info,
        };

        (config, env)
    }
}

impl Command {
    fn exec(&self, config: &Config, env: &Env) {
        match self {
            Command::Deposit(cmd) => cmd.exec(config, env),
            Command::Claim(cmd) => cmd.exec(config, env),
        }
    }
}

impl DepositCommand {
    fn exec(&self, config: &Config, env: &Env) {
        let token_contract_id = Address::from_val(env, &config.token_contract_id);
        let timelock_contract_id = Address::from_val(env, &config.timelock_contract_id);

        // The contract needs to be re-registered each time the Env is created.
        let _timelock_contract_id =
            env.register_at(&timelock_contract_id, ClaimableBalanceContract, ());

        let timelock_client = ClaimableBalanceContractClient::new(&env, &timelock_contract_id);
        let depositor_address =
            Address::from_val(env, &config.input.addresses[self.depositor_index]);
        let claimant_addresses: RustVec<Address> = self
            .claimant_indexes
            .iter()
            .map(|idx| Address::from_val(env, &config.input.addresses[*idx]))
            .collect();
        let time_bound = TimeBound::from_val(env, &self.time_bound);

        let _ = timelock_client.try_deposit(
            &depositor_address,
            &token_contract_id,
            &self.amount,
            &Vec::from_slice(&env, &claimant_addresses),
            &time_bound,
        );
    }
}

impl ClaimCommand {
    fn exec(&self, config: &Config, env: &Env) {
        let timelock_contract_id = Address::from_val(env, &config.timelock_contract_id);

        let _timelock_contract_id =
            env.register_at(&timelock_contract_id, ClaimableBalanceContract, ());

        let timelock_client = ClaimableBalanceContractClient::new(&env, &timelock_contract_id);
        let claimant_address = Address::from_val(env, &config.input.addresses[self.claimant_index]);

        let _ = timelock_client.try_claim(&claimant_address, &self.amount);
    }
}

fn assert_invariants(config: &Config, prev_env: &Env, curr_env: &Env) {
    // Make assertions that depend only on the current state.
    assert_current(config, curr_env);

    // Make assertions that compare to the previous state.
    assert_delta(config, prev_env, curr_env);
}

fn assert_current(config: &Config, env: &Env) {
    let token_contract_id = Address::from_val(env, &config.token_contract_id);
    let timelock_contract_id = Address::from_val(env, &config.timelock_contract_id);

    let token_client = TokenClient::new(env, &token_contract_id);

    // Configure the environment to access the timelock contract's storage.
    env.as_contract(&timelock_contract_id, || {
        let storage = env.storage().persistent();

        // Get the two datums owned by the timelock contract.
        let is_initialized = storage.has(&DataKey::Init);
        let claimable_balance = storage.get::<_, ClaimableBalance>(&DataKey::Balance);

        // Call the token client to get the balance held in the timelock contract.
        // This consumes contract execution budget.
        let actual_token_balance: i128 = token_client
            .try_balance(&timelock_contract_id)
            .unwrap_or(Ok(0))
            .unwrap();

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
            assert_eq!(claimable_balance.amount, actual_token_balance);

            assert!(claimable_balance.claimants.len() > 0);

            assert!(config.deposit_info.is_some());
            if let Some((_, deposit_amount, _)) = &config.deposit_info {
                assert!(claimable_balance.amount <= *deposit_amount);
            }
        }
    });
}

// Here we can make assertions by comparing the previous contract
// state to the current contract state.
fn assert_delta(config: &Config, prev_env: &Env, curr_env: &Env) {
    let time_bound = config.deposit_info.as_ref().map(|i| i.2.clone());
    let prev_balance = {
        let timelock_contract_id = Address::from_val(prev_env, &config.timelock_contract_id);
        prev_env.as_contract(&timelock_contract_id, || {
            let storage = prev_env.storage().persistent();
            let claimable_balance = storage.get::<_, ClaimableBalance>(&DataKey::Balance);
            if let Some(claimable_balance) = claimable_balance {
                Some(claimable_balance.amount)
            } else {
                None
            }
        })
    };
    let curr_balance = {
        let timelock_contract_id = Address::from_val(curr_env, &config.timelock_contract_id);
        curr_env.as_contract(&timelock_contract_id, || {
            let storage = curr_env.storage().persistent();
            let claimable_balance = storage.get::<_, ClaimableBalance>(&DataKey::Balance);
            if let Some(claimable_balance) = claimable_balance {
                Some(claimable_balance.amount)
            } else {
                None
            }
        })
    };
    let curr_timestamp = curr_env.ledger().timestamp();

    match (time_bound, prev_balance, curr_balance) {
        (Some(time_bound), Some(prev_balance), Some(curr_balance)) => {
            let time_bound = TimeBound::from_val(curr_env, &time_bound);
            let balance_changed = prev_balance != curr_balance;
            if balance_changed {
                match time_bound.kind {
                    TimeBoundKind::Before => {
                        assert!(curr_timestamp <= time_bound.timestamp);
                    }
                    TimeBoundKind::After => {
                        assert!(curr_timestamp >= time_bound.timestamp);
                    }
                }
            }
        }
        _ => {}
    }
}
