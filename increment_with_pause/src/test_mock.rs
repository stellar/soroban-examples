#![cfg(test)]
use crate::{Error, IncrementContract, IncrementContractArgs, IncrementContractClient, Pause};
use soroban_sdk::{contract, contractimpl, Env};

mod notpaused {
    use super::*;
    #[contract]
    pub struct Mock;
    #[contractimpl]
    impl Pause for Mock {
        fn paused(_env: Env) -> bool {
            false
        }
    }
}

#[test]
fn test_notpaused() {
    let env = Env::default();
    let pause_id = env.register(notpaused::Mock, ());
    let contract_id = env.register(
        IncrementContract,
        IncrementContractArgs::__constructor(&pause_id),
    );
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
}

mod paused {
    use super::*;
    #[contract]
    pub struct Mock;
    #[contractimpl]
    impl Pause for Mock {
        fn paused(_env: Env) -> bool {
            true
        }
    }
}

#[test]
fn test_paused() {
    let env = Env::default();
    let pause_id = env.register(paused::Mock, ());
    let contract_id = env.register(
        IncrementContract,
        IncrementContractArgs::__constructor(&pause_id),
    );
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.try_increment(), Err(Ok(Error::Paused)));
}
