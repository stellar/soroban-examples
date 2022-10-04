#![cfg(test)]

use super::*;

use soroban_sdk::{testutils::Accounts, BytesN, Env, Invoker};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, ExampleContract);
    let client = ExampleContractClient::new(&env, contract_id);

    // Initialize contract by setting the admin.
    let admin = env.accounts().generate();
    let admin_invoker = &Invoker::Account(admin.clone());
    client.set_admin(admin_invoker);

    // Check if user 1 has a num, it doesn't yet.
    let user1 = env.accounts().generate();
    let user1_invoker = &Invoker::Account(user1.clone());
    assert_eq!(client.num(user1_invoker), None);

    // Have user 1 set a num for themselves.
    let five = BigInt::from_u32(&env, 5);
    client.with_source_account(&user1).set_num(&five);
    assert_eq!(client.num(user1_invoker), Some(five));

    // Have admin overwrite user 1's num.
    let ten = BigInt::from_u32(&env, 10);
    client
        .with_source_account(&admin)
        .overwrite(user1_invoker, &ten);
    assert_eq!(client.num(user1_invoker), Some(ten));
}
