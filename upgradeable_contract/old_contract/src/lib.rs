#![no_std]

use soroban_sdk::{contractimpl, contracttype, Address, BytesN, Env};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,
}

pub struct UpgradeableContract;

#[contractimpl]
impl UpgradeableContract {
    pub fn init(e: Env, admin: Address) {
        e.storage().set(&DataKey::Admin, &admin);
    }

    pub fn version() -> u32 {
        1
    }

    pub fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {
        let admin: Address = e.storage().get_unchecked(&DataKey::Admin).unwrap();
        admin.require_auth();

        e.update_current_contract_wasm(&new_wasm_hash);
    }
}

mod test;
