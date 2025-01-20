#![cfg(test)]
extern crate std;

use crate::LiquidityPoolClient;

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    token, Address, Env, IntoVal,
};

fn create_token_contract<'a>(
    e: &Env,
    admin: &Address,
) -> (token::Client<'a>, token::StellarAssetClient<'a>) {
    let sac = e.register_stellar_asset_contract_v2(admin.clone());
    (
        token::Client::new(e, &sac.address()),
        token::StellarAssetClient::new(e, &sac.address()),
    )
}

fn create_liqpool_contract<'a>(
    e: &Env,
    token_a: &Address,
    token_b: &Address,
) -> LiquidityPoolClient<'a> {
    LiquidityPoolClient::new(e, &e.register(crate::LiquidityPool {}, (token_a, token_b)))
}

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();

    let admin1 = Address::generate(&e);
    let admin2 = Address::generate(&e);

    let (token1, token1_admin) = create_token_contract(&e, &admin1);
    let (token2, token2_admin) = create_token_contract(&e, &admin2);
    let user1 = Address::generate(&e);

    let liqpool = create_liqpool_contract(&e, &token1.address, &token2.address);

    token1_admin.mint(&user1, &1000);
    assert_eq!(token1.balance(&user1), 1000);

    token2_admin.mint(&user1, &1000);
    assert_eq!(token2.balance(&user1), 1000);

    liqpool.deposit(&user1, &100, &100, &100, &100);
    assert_eq!(
        e.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    liqpool.address.clone(),
                    symbol_short!("deposit"),
                    (&user1, 100_i128, 100_i128, 100_i128, 100_i128).into_val(&e)
                )),
                sub_invocations: std::vec![
                    AuthorizedInvocation {
                        function: AuthorizedFunction::Contract((
                            token1.address.clone(),
                            symbol_short!("transfer"),
                            (&user1, &liqpool.address, 100_i128).into_val(&e)
                        )),
                        sub_invocations: std::vec![]
                    },
                    AuthorizedInvocation {
                        function: AuthorizedFunction::Contract((
                            token2.address.clone(),
                            symbol_short!("transfer"),
                            (&user1, &liqpool.address, 100_i128).into_val(&e)
                        )),
                        sub_invocations: std::vec![]
                    }
                ]
            }
        )]
    );

    assert_eq!(liqpool.balance_shares(&user1), 100);
    assert_eq!(token1.balance(&user1), 900);
    assert_eq!(token1.balance(&liqpool.address), 100);
    assert_eq!(token2.balance(&user1), 900);
    assert_eq!(token2.balance(&liqpool.address), 100);

    liqpool.swap(&user1, &false, &49, &100);
    assert_eq!(
        e.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    liqpool.address.clone(),
                    symbol_short!("swap"),
                    (&user1, false, 49_i128, 100_i128).into_val(&e)
                )),
                sub_invocations: std::vec![AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        token1.address.clone(),
                        symbol_short!("transfer"),
                        (&user1, &liqpool.address, 97_i128).into_val(&e)
                    )),
                    sub_invocations: std::vec![]
                }]
            }
        )]
    );

    assert_eq!(token1.balance(&user1), 803);
    assert_eq!(token1.balance(&liqpool.address), 197);
    assert_eq!(token2.balance(&user1), 949);
    assert_eq!(token2.balance(&liqpool.address), 51);

    e.cost_estimate().budget().reset_unlimited();
    liqpool.withdraw(&user1, &100, &197, &51);

    assert_eq!(
        e.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    liqpool.address.clone(),
                    symbol_short!("withdraw"),
                    (&user1, 100_i128, 197_i128, 51_i128).into_val(&e)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(token1.balance(&user1), 1000);
    assert_eq!(token2.balance(&user1), 1000);
    assert_eq!(liqpool.balance_shares(&user1), 0);
    assert_eq!(token1.balance(&liqpool.address), 0);
    assert_eq!(token2.balance(&liqpool.address), 0);
}

#[test]
#[should_panic]
fn deposit_amount_zero_should_panic() {
    let e = Env::default();
    e.mock_all_auths();

    // Create contracts
    let admin1 = Address::generate(&e);
    let admin2 = Address::generate(&e);

    let (token1, token1_admin) = create_token_contract(&e, &admin1);
    let (token2, token2_admin) = create_token_contract(&e, &admin2);
    let liqpool = create_liqpool_contract(&e, &token1.address, &token2.address);

    // Create a user
    let user1 = Address::generate(&e);

    token1_admin.mint(&user1, &1000);
    assert_eq!(token1.balance(&user1), 1000);

    token2_admin.mint(&user1, &1000);
    assert_eq!(token2.balance(&user1), 1000);

    liqpool.deposit(&user1, &1, &0, &0, &0);
}

#[test]
#[should_panic]
fn swap_reserve_one_nonzero_other_zero() {
    let e = Env::default();
    e.mock_all_auths();

    // Create contracts
    let admin1 = Address::generate(&e);
    let admin2 = Address::generate(&e);

    let (token1, token1_admin) = create_token_contract(&e, &admin1);
    let (token2, token2_admin) = create_token_contract(&e, &admin2);

    let liqpool = create_liqpool_contract(&e, &token1.address, &token2.address);

    // Create a user
    let user1 = Address::generate(&e);

    token1_admin.mint(&user1, &1000);
    assert_eq!(token1.balance(&user1), 1000);

    token2_admin.mint(&user1, &1000);
    assert_eq!(token2.balance(&user1), 1000);

    // Try to get to a situation where the reserves are 1 and 0.
    // It shouldn't be possible.
    token2.transfer(&user1, &liqpool.address, &1);
    liqpool.swap(&user1, &false, &1, &1);
}
