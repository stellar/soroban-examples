#![cfg(test)]

use super::{DataKey, TtlContract, TtlContractClient};
use soroban_sdk::testutils::storage::{Instance, Persistent, Temporary};
use soroban_sdk::testutils::Ledger;
use soroban_sdk::Env;

extern crate std;

/// Create an environment with specific values of network settings.
fn create_env() -> Env {
    let env = Env::default();
    env.ledger().with_mut(|li| {
        // Current ledger sequence - the TTL is the number of
        // ledgers from the `sequence_number` (exclusive) until
        // the last ledger sequence where entry is still considered
        // alive.
        li.sequence_number = 100_000;
        // Minimum TTL for persistent entries - new persistent (and instance)
        // entries will have this TTL when created.
        li.min_persistent_entry_ttl = 500;
        // Minimum TTL for temporary entries - new temporary
        // entries will have this TTL when created.
        li.min_temp_entry_ttl = 100;
        // Maximum TTL of any entry. Note, that entries can have their TTL
        // extended indefinitely, but each extension can be at most
        // `max_entry_ttl` ledger from the current `sequence_number`.
        li.max_entry_ttl = 15000;
    });
    env
}

// This test covers the general behavior of TTL extensions via `get_ttl`
// test utility.
// Using `get_ttl` is the recommended way to ensure that the TTL has
// been extended to the expected value.
#[test]
fn test_extend_ttl_behavior() {
    let env = create_env();
    let contract_id = env.register(TtlContract, ());
    let client = TtlContractClient::new(&env, &contract_id);

    // Create initial entries and make sure their TTLs correspond to
    // `min_persistent_entry_ttl` and `min_temp_entry_ttl` values set in
    // `create_env()`.
    client.setup();
    env.as_contract(&contract_id, || {
        // Note, that TTL doesn't include the current ledger, but when entry
        // is created the current ledger is counted towards the number of
        // ledgers specified by `min_persistent/temp_entry_ttl`, thus
        // the TTL is 1 ledger less than the respective setting.
        assert_eq!(env.storage().persistent().get_ttl(&DataKey::MyKey), 499);
        assert_eq!(env.storage().instance().get_ttl(), 499);
        assert_eq!(env.storage().temporary().get_ttl(&DataKey::MyKey), 99);
    });

    // Extend persistent entry TTL to 5000 ledgers - now it is 5000.
    client.extend_persistent();
    env.as_contract(&contract_id, || {
        assert_eq!(env.storage().persistent().get_ttl(&DataKey::MyKey), 5000);
    });

    // Extend instance TTL to 10000 ledgers - now it is 10000.
    client.extend_instance();
    env.as_contract(&contract_id, || {
        assert_eq!(env.storage().instance().get_ttl(), 10000);
    });

    // Extend temporary entry TTL to 7000 ledgers - now it is 7000.
    client.extend_temporary();
    env.as_contract(&contract_id, || {
        assert_eq!(env.storage().temporary().get_ttl(&DataKey::MyKey), 7000);
    });

    // Now bump the ledger sequence by 5000 in order to sanity-check
    // the threshold settings of `extend_ttl` operations.
    env.ledger().with_mut(|li| {
        li.sequence_number = 100_000 + 5_000;
    });
    // Now the TTL of every entry has been reduced by 5000 ledgers.
    env.as_contract(&contract_id, || {
        assert_eq!(env.storage().persistent().get_ttl(&DataKey::MyKey), 0);
        assert_eq!(env.storage().instance().get_ttl(), 5000);
        assert_eq!(env.storage().temporary().get_ttl(&DataKey::MyKey), 2000);
    });
    // Extend TTL of all the entries.
    client.extend_persistent();
    client.extend_instance();
    client.extend_temporary();
    env.as_contract(&contract_id, || {
        assert_eq!(env.storage().persistent().get_ttl(&DataKey::MyKey), 5000);
        // Instance TTL hasn't been increased because the remaining TTL
        // (5000 ledgers) is larger than the threshold used by
        // `extend_instance` (2000 ledgers)
        assert_eq!(env.storage().instance().get_ttl(), 5000);
        assert_eq!(env.storage().temporary().get_ttl(&DataKey::MyKey), 7000);
    });
}

// This test demonstrates that temporary entries are considered to be removed
// after their TTL expires.
// It is not the recommended way to test `extend_ttl` (use `get_ttl` instead).
// This behavior is mostly useful to catch bugs (such as missing/invalid TTL
// extensions).
// Note, that while test environment emulates the entry expiration logic, in
// the real environment anyone can extend the TTL of any entry, so you should
// never rely on the entries to be automatically removed. Temporary storage
// is just a cost optimization, time boundaries still have to be managed by
// the contract logic, as they are in e.g. token example
// (https://github.com/stellar/soroban-examples/blob/002edecda8da85d71f7fdc000eeed924c5a71cbd/token/src/allowance.rs#L7)
#[test]
fn test_temp_entry_removal() {
    let env = create_env();
    let contract_id = env.register(TtlContract, ());
    let client = TtlContractClient::new(&env, &contract_id);
    client.setup();
    // Extend the contract instance to live more than 7001 ledgers.
    client.extend_instance();
    // Extend the temporary entry TTL to 7000 ledgers.
    client.extend_temporary();
    // Bump the ledger sequence by 7001 ledgers (one ledger past TTL).
    env.ledger().with_mut(|li| {
        li.sequence_number += 7001;
    });
    // Now the entry is no longer present in the environment.
    env.as_contract(&contract_id, || {
        assert_eq!(env.storage().temporary().has(&DataKey::MyKey), false);
    });
}

// This test demonstrates that persistent entries are considered to be 'archived'
// after their TTL expires and then automatically restored when accessed. This
// emulates the default simulation behavior that will be encountered by most
// of the contract users.
#[test]
fn test_persistent_entry_auto_restored() {
    let env = create_env();
    let contract_id = env.register(TtlContract, ());
    let client = TtlContractClient::new(&env, &contract_id);
    client.setup();
    // Extend the contract instance to outlive the persistent entry.
    client.extend_instance();
    // Extend the persistent entry TTL to 5000 ledgers.
    client.extend_persistent();

    env.as_contract(&contract_id, || {
        assert_eq!(env.storage().persistent().get_ttl(&DataKey::MyKey), 5000);
    });
    // Bump the ledger sequence by 10001 ledgers (one ledger past TTL).
    env.ledger().with_mut(|li| {
        li.sequence_number += 5001;
    });
    // Now any call involving the expired persistent data will cause automatic
    // restoration.
    client.extend_persistent();
    // Automatic restoration is mostly transparent (in the same way as it is
    // going to be transparent to the contract users), the main side effect is
    // the increased fees. This can be observed via examining the resources
    // in `env.cost_estimate`:
    let resources = env.cost_estimate().resources();
    // Notice that disk read bytes and write bytes are increased even though the
    // function itself is itself read-only.
    assert!(resources.disk_read_bytes > 0);
    assert!(resources.write_bytes > 0);
    assert_eq!(resources.write_entries, 1);

    // TTL is now 5000 ledgers again.
    env.as_contract(&contract_id, || {
        assert_eq!(env.storage().persistent().get_ttl(&DataKey::MyKey), 5000);
    });

    // Bump the ledger sequence even more to cause the contract instance to
    // also be archived.
    env.ledger().with_mut(|li| {
        li.sequence_number += 100_000;
    });

    client.extend_persistent();
    let resources = env.cost_estimate().resources();
    // We have 2 entries restored now and thus written (contract instance entry
    // and persistent data entry).
    assert_eq!(resources.write_entries, 2);

    // Contract still can be called as usual:

    env.as_contract(&contract_id, || {
        assert_eq!(env.storage().persistent().get_ttl(&DataKey::MyKey), 5000);
    });
}
