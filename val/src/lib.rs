#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Val};

#[contracttype]
enum Data {
    Counter,
    Owner(u32),
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Mint an ID.
    pub fn mint(e: &Env, owner: Address) -> Val {
        // Increment the current count, using the new value as an ID for the minted token.
        let id = e
            .storage()
            .instance()
            .update(&Data::Counter, |v| v.unwrap_or(0) + 1);

        // Set the owner of the ID.
        e.storage().persistent().set(&Data::Owner(id), &owner);

        // Return the ID as a Val, because the type of the ID is an internal implementation detail
        // that no one should rely on.
        id.into()
    }

    // Get the owner of an ID.
    pub fn owner(e: &Env, id: Val) -> Address {
        let id = id.try_into().unwrap();
        let owner = e.storage().persistent().get(&Data::Owner(id));
        owner.unwrap()
    }

    // Transfer an ID to a new owner.
    pub fn transfer(e: &Env, id: Val, new_owner: Address) {
        let id = id.try_into().unwrap();
        e.storage()
            .persistent()
            .update(&Data::Owner(id), |maybe_owner| {
                let owner: Address = maybe_owner.unwrap();
                owner.require_auth();
                new_owner
            });
    }
}

mod test;
