#![cfg(test)]

use super::*;
use soroban_sdk::testutils::{Ledger, LedgerInfo};
use soroban_sdk::{vec, Env, IntoVal};

mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
    pub type TokenClient = Client;
}

use token::TokenClient;

fn create_token_contract(e: &Env, admin: &Address) -> TokenClient {
    TokenClient::new(
        e,
        &e.register_stellar_asset_contract_with_admin(admin.clone()),
    )
}

fn create_claimable_balance_contract(e: &Env) -> ClaimableBalanceContractClient {
    ClaimableBalanceContractClient::new(e, e.register_contract(None, ClaimableBalanceContract {}))
}

struct ClaimableBalanceTest {
    env: Env,
    deposit_account: Account,
    claim_accounts: [Account; 3],
    token: TokenClient,
    contract: ClaimableBalanceContractClient,
    contract_address: Address,
}

impl ClaimableBalanceTest {
    fn setup() -> Self {
        let env: Env = Default::default();
        env.ledger().set(LedgerInfo {
            timestamp: 12345,
            protocol_version: 1,
            sequence_number: 10,
            network_passphrase: Default::default(),
            base_reserve: 10,
        });

        let deposit_account = Account::random(&env);

        let claim_accounts = [
            Account::random(&env),
            Account::random(&env),
            Account::random(&env),
        ];

        let token_admin = Account::random(&env);

        let token = create_token_contract(&env, &token_admin.address());
        token.mint(&token_admin, &deposit_account.address(), &1000);

        let contract = create_claimable_balance_contract(&env);
        let contract_address = Address::from_contract_id(&env, &contract.contract_id);
        ClaimableBalanceTest {
            env,
            deposit_account,
            claim_accounts,
            token,
            contract,
            contract_address,
        }
    }
}

#[test]
fn test_deposit_and_claim() {
    let test = ClaimableBalanceTest::setup();
    test.contract.deposit(
        &test.deposit_account,
        &test.token.contract_id,
        &800,
        &vec![
            &test.env,
            test.claim_accounts[0].address(),
            test.claim_accounts[1].address(),
        ],
        &TimeBound {
            kind: TimeBoundKind::Before,
            timestamp: 12346,
        },
    );
    assert!(test.env.verify_account_authorization(
        &test.deposit_account,
        &[(&test.contract.contract_id, "deposit")],
        (
            &test.token.contract_id,
            800_i128,
            vec![
                &test.env,
                test.claim_accounts[0].address(),
                test.claim_accounts[1].address(),
            ],
            TimeBound {
                kind: TimeBoundKind::Before,
                timestamp: 12346,
            },
        )
            .into_val(&test.env),
    ));

    // That shouldn't be necessary in tests most of the time. Top-level contract
    // shouldn't worry about sub-contract authorizations normally.
    assert!(test.env.verify_account_authorization(
        &test.deposit_account,
        &[
            (&test.contract.contract_id, "deposit"),
            (&test.token.contract_id, "xfer")
        ],
        (&test.contract_address, 800_i128).into_val(&test.env),
    ));

    assert_eq!(test.token.balance(&test.deposit_account.address()), 200);
    assert_eq!(test.token.balance(&test.contract_address), 800);
    assert_eq!(test.token.balance(&test.claim_accounts[1].address()), 0);

    test.contract.claim(&test.claim_accounts[1]);
    assert!(test.env.verify_account_authorization(
        &test.claim_accounts[1],
        &[(&test.contract.contract_id, "claim")],
        vec![&test.env],
    ));

    assert_eq!(test.token.balance(&test.deposit_account.address()), 200);
    assert_eq!(test.token.balance(&test.contract_address), 0);
    assert_eq!(test.token.balance(&test.claim_accounts[1].address()), 800);
}

#[test]
#[should_panic(expected = "contract has been already initialized")]
fn test_double_deposit_not_possible() {
    let test = ClaimableBalanceTest::setup();
    test.contract.deposit(
        &test.deposit_account,
        &test.token.contract_id,
        &1,
        &vec![&test.env, test.claim_accounts[0].address()],
        &TimeBound {
            kind: TimeBoundKind::Before,
            timestamp: 12346,
        },
    );
    test.contract.deposit(
        &test.deposit_account,
        &test.token.contract_id,
        &1,
        &vec![&test.env, test.claim_accounts[0].address()],
        &TimeBound {
            kind: TimeBoundKind::Before,
            timestamp: 12346,
        },
    );
}

#[test]
#[should_panic(expected = "claimant is not allowed to claim this balance")]
fn test_unauthorized_claim_not_possible() {
    let test = ClaimableBalanceTest::setup();
    test.contract.deposit(
        &test.deposit_account,
        &test.token.contract_id,
        &800,
        &vec![
            &test.env,
            test.claim_accounts[0].address(),
            test.claim_accounts[1].address(),
        ],
        &TimeBound {
            kind: TimeBoundKind::Before,
            timestamp: 12346,
        },
    );

    test.contract.claim(&test.claim_accounts[2]);
}

#[test]
#[should_panic(expected = "time predicate is not fulfilled")]
fn test_out_of_time_bound_claim_not_possible() {
    let test = ClaimableBalanceTest::setup();
    test.contract.deposit(
        &test.deposit_account,
        &test.token.contract_id,
        &800,
        &vec![&test.env, test.claim_accounts[0].address()],
        &TimeBound {
            kind: TimeBoundKind::After,
            timestamp: 12346,
        },
    );

    test.contract.claim(&test.claim_accounts[0]);
}

#[test]
#[should_panic(expected = "HostStorageError")]
fn test_double_claim_not_possible() {
    let test = ClaimableBalanceTest::setup();
    test.contract.deposit(
        &test.deposit_account,
        &test.token.contract_id,
        &800,
        &vec![&test.env, test.claim_accounts[0].address()],
        &TimeBound {
            kind: TimeBoundKind::Before,
            timestamp: 12346,
        },
    );

    test.contract.claim(&test.claim_accounts[0]);
    assert_eq!(test.token.balance(&test.claim_accounts[0].address()), 800);
    test.contract.claim(&test.claim_accounts[0]);
}

#[test]
#[should_panic(expected = "contract has been already initialized")]
fn test_deposit_after_claim_not_possible() {
    let test = ClaimableBalanceTest::setup();
    test.contract.deposit(
        &test.deposit_account,
        &test.token.contract_id,
        &800,
        &vec![&test.env, test.claim_accounts[0].address()],
        &TimeBound {
            kind: TimeBoundKind::After,
            timestamp: 12344,
        },
    );

    test.contract.claim(&test.claim_accounts[0]);
    assert_eq!(test.token.balance(&test.claim_accounts[0].address()), 800);
    test.contract.deposit(
        &test.deposit_account,
        &test.token.contract_id,
        &200,
        &vec![&test.env, test.claim_accounts[0].address()],
        &TimeBound {
            kind: TimeBoundKind::After,
            timestamp: 12344,
        },
    );
}
