#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,
    NewAdmin,
}

#[contract]
pub struct UpgradeableContract;

#[contractimpl]
impl UpgradeableContract {
    // Note, that constructor is not called when the contract is upgraded.
    // Thus we introduce a new function `handle_upgrade` that brings the
    // freshly upgraded contract to proper state (specifically, initializes
    // the `NewAdmin` key).
    pub fn __constructor(e: Env, admin: Address) {
        e.storage().instance().set(&DataKey::Admin, &admin);
        e.storage().instance().set(&DataKey::NewAdmin, &admin);
    }

    pub fn handle_upgrade(e: Env) {
        let admin: Address = e.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        if !e.storage().instance().has(&DataKey::NewAdmin) {
            e.storage().instance().set(&DataKey::NewAdmin, &admin);
        }
    }

    pub fn version() -> u32 {
        2
    }

    pub fn new_v2_fn() -> u32 {
        1010101
    }

    pub fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {
        let admin: Address = e.storage().instance().get(&DataKey::NewAdmin).unwrap();
        admin.require_auth();

        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}
