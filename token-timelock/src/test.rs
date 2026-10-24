#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Ledger};
use soroban_sdk::{symbol_short, token, Address, Env, IntoVal};
use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let sac = e.register_stellar_asset_contract_v2(admin.clone());
    (
        token::Client::new(e, &sac.address()),
        token::StellarAssetClient::new(e, &sac.address()),
    )
}

struct TokenTimelockTest<'a> {
    env: Env,
    deposit_address: Address,
    claim_address: Address,
    other_address: Address,
    token: TokenClient<'a>,
    unlock_time: u64,
}

impl<'a> TokenTimelockTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        env.ledger().with_mut(|li| {
            li.timestamp = 12345;
        });

        let deposit_address = Address::generate(&env);
        let claim_address = Address::generate(&env);
        let other_address = Address::generate(&env);

        let token_admin = Address::generate(&env);
        let (token, token_admin_client) = create_token_contract(&env, &token_admin);
        token_admin_client.mint(&deposit_address, &1000);

        TokenTimelockTest {
            env,
            deposit_address,
            claim_address,
            other_address,
            token,
            unlock_time: 12350,
        }
    }

    fn deploy_contract(&self, amount: i128) -> TokenTimelockContractClient<'a> {
        let contract_id = self.env.register(
            TokenTimelockContract,
            (
                &self.deposit_address,
                &self.token.address,
                amount,
                &self.claim_address,
                self.unlock_time,
            ),
        );
        TokenTimelockContractClient::new(&self.env, &contract_id)
    }
}

#[test]
fn test_constructor_and_claim() {
    let test = TokenTimelockTest::setup();
    let contract = test.deploy_contract(800);

    // Verify token was transferred from deposit_address to contract upon construction
    assert_eq!(test.token.balance(&test.deposit_address), 200);
    assert_eq!(test.token.balance(&contract.address), 800);
    assert_eq!(test.token.balance(&test.claim_address), 0);

    // Fast-forward ledger time past unlock_time
    test.env.ledger().with_mut(|li| {
        li.timestamp = 12351;
    });

    contract.claim(&test.claim_address);

    assert_eq!(
        test.env.auths(),
        [(
            test.claim_address.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    contract.address.clone(),
                    symbol_short!("claim"),
                    (test.claim_address.clone(),).into_val(&test.env),
                )),
                sub_invocations: std::vec![]
            }
        ),]
    );

    // Verify balance transferred to claimant and contract balance is now 0
    assert_eq!(test.token.balance(&test.deposit_address), 200);
    assert_eq!(test.token.balance(&contract.address), 0);
    assert_eq!(test.token.balance(&test.claim_address), 800);
}

#[test]
#[should_panic(expected = "time predicate is not fulfilled")]
fn test_out_of_time_bound_claim_not_possible() {
    let test = TokenTimelockTest::setup();
    let contract = test.deploy_contract(800);

    // Current timestamp is 12345, while unlock_time is 12350
    contract.claim(&test.claim_address);
}

#[test]
#[should_panic(expected = "claimant is not allowed to claim this balance")]
fn test_unauthorized_claim_not_possible() {
    let test = TokenTimelockTest::setup();
    let contract = test.deploy_contract(800);

    // Fast-forward ledger time past unlock_time
    test.env.ledger().with_mut(|li| {
        li.timestamp = 12351;
    });

    // Try claiming with another unauthorized address
    contract.claim(&test.other_address);
}

#[test]
#[should_panic(expected = "contract has no balance")]
fn test_double_claim_not_possible() {
    let test = TokenTimelockTest::setup();
    let contract = test.deploy_contract(800);

    test.env.ledger().with_mut(|li| {
        li.timestamp = 12351;
    });

    contract.claim(&test.claim_address);
    assert_eq!(test.token.balance(&test.claim_address), 800);

    // Second claim must fail
    contract.claim(&test.claim_address);
}

#[test]
#[should_panic(expected = "amount must be positive")]
fn test_zero_amount_constructor_panics() {
    let test = TokenTimelockTest::setup();
    test.deploy_contract(0);
}

#[test]
#[should_panic(expected = "amount must be positive")]
fn test_negative_amount_constructor_panics() {
    let test = TokenTimelockTest::setup();
    test.deploy_contract(-10);
}
