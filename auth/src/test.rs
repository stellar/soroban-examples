#![cfg(test)]
extern crate std;
use super::*;

use soroban_sdk::{symbol, testutils::Address as _, Address, Env, IntoVal};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    let user_1 = Address::random(&env);
    let user_2 = Address::random(&env);

    assert_eq!(client.increment(&user_1, &5), 5);
    // Verify that the user indeed had to authorize a call of `increment` with
    // the expected arguments:
    assert_eq!(
        env.recorded_top_authorizations(),
        std::vec![(
            // Address for which auth is performed
            user_1.clone(),
            // Identifier of the called contract
            contract_id.clone(),
            // Name of the called function
            symbol!("increment"),
            // Arguments used to call `increment` (converted to the env-managed vector via `into_val`)
            (user_1.clone(), 5_u32).into_val(&env)
        )]
    );

    // Do more `increment` calls. It's not necessary to verify authorizations
    // for every one of them as we don't expect the auth logic to change from
    // call to call.
    assert_eq!(client.increment(&user_1, &2), 7);
    assert_eq!(client.increment(&user_2, &1), 1);
    assert_eq!(client.increment(&user_1, &3), 10);
    assert_eq!(client.increment(&user_2, &4), 5);
}
