#![cfg(test)]

extern crate std;

use super::*;
use soroban_sdk::{testutils::Events, Env, Event, IntoVal};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);

    // Assert on the events emitted by the last contract invocation using
    // the contract's event struct defined with #[contractevent] macro to
    // construct the expected event in xdr form for comparison.
    assert_eq!(client.increment(), 1);
    assert_eq!(
        env.events().all(),
        std::vec![IncrementEvent { count: 1 }.to_xdr(&env, &contract_id)]
    );

    // Assert on the events emitted by the last contract invocation that
    // were emitted by a contract with a specific contract id. This is
    // useful when your contract might call other contracts that also emit events.
    assert_eq!(client.increment(), 2);
    assert_eq!(
        env.events().all().filter_by_contract(&contract_id),
        std::vec![IncrementEvent { count: 2 }.to_xdr(&env, &contract_id)]
    );

    // Assert on the events emitted by the last contract invocation by
    // building a tuple form of the event manually. This is useful
    // when the contract does not define its events using the #[contractevent] macro.
    //
    // Tuple Format: (contract_id: Address, topics: Val, data: Val)
    assert_eq!(client.increment(), 3);
    assert_eq!(
        env.events().all(),
        soroban_sdk::vec![
            &env,
            (
                contract_id,
                (symbol_short!("COUNTER"), symbol_short!("increment")).into_val(&env),
                3u32.into_val(&env)
            ),
        ]
    );
}
