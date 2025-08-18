#![cfg(test)]

extern crate std;

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, BytesN, Env, IntoVal,
};

use crate::{UpgradeableContract, UpgradeableContractClient};

mod new_contract {
    soroban_sdk::contractimport!(
        file = "../new_contract/target/wasm32v1-none/release/soroban_upgradeable_contract_new_contract.wasm"
    );
}

fn install_new_wasm(e: &Env) -> BytesN<32> {
    e.deployer().upload_contract_wasm(new_contract::WASM)
}

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(UpgradeableContract, (&admin,));

    let client = UpgradeableContractClient::new(&env, &contract_id);

    assert_eq!(1, client.version());

    let new_wasm_hash = install_new_wasm(&env);

    client.upgrade(&new_wasm_hash);
    assert_eq!(2, client.version());

    // new_v2_fn was added in the new contract, so the existing
    // client is out of date. Generate a new one.
    let client = new_contract::Client::new(&env, &contract_id);
    assert_eq!(1010101, client.new_v2_fn());

    // New contract version requires the `NewAdmin` key to be initialized, but since the constructor
    // hasn't been called, it is not initialized, thus calling try_upgrade won't work.
    let new_update_result = client.try_upgrade(&new_wasm_hash);
    assert!(new_update_result.is_err());

    // `handle_upgrade` sets the `NewAdmin` key properly.
    client.handle_upgrade();

    // Now upgrade should succeed (though we are not actually changing the Wasm).
    client.upgrade(&new_wasm_hash);
    // The new admin is the same as the old admin, so the authorization is still performed for
    // the `admin` address.
    assert_eq!(
        env.auths(),
        std::vec![(
            admin,
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    contract_id.clone(),
                    symbol_short!("upgrade"),
                    (new_wasm_hash,).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    )
}
