//! This contract demonstrates the 'token-timelock' concept and implements a
//! simplified claimable balance (similar to
//! https://developers.stellar.org/docs/glossary/claimable-balance).
//! The contract deposits a token amount from the funder on initialization,
//! which can be claimed by a designated claimant only after a specific timestamp.
#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Balance,
}

#[derive(Clone)]
#[contracttype]
pub struct ClaimableBalance {
    pub token: Address,
    pub amount: i128,
    pub claimant: Address,
    pub unlock_time: u64,
}

#[contract]
pub struct TokenTimelockContract;

#[contractimpl]
impl TokenTimelockContract {
    /// Initializes the timelock by transferring `amount` of `token` from `from`
    /// to this contract address, unlocking for `claimant` after `unlock_time`.
    pub fn __constructor(
        env: Env,
        from: Address,
        token: Address,
        amount: i128,
        claimant: Address,
        unlock_time: u64,
    ) {
        assert!(amount > 0, "amount must be positive");
        // Make sure `from` address authorized the constructor invocation.
        from.require_auth();

        // Transfer token from `from` to this contract address.
        token::Client::new(&env, &token).transfer(&from, &env.current_contract_address(), &amount);

        // Store the claimable balance details.
        env.storage().instance().set(
            &DataKey::Balance,
            &ClaimableBalance {
                token,
                amount,
                claimant,
                unlock_time,
            },
        );
    }

    /// Claims the balance after the timelock has expired.
    pub fn claim(env: Env, claimant: Address) {
        // Make sure claimant has authorized this call, ensuring their identity.
        claimant.require_auth();

        let claimable_balance: ClaimableBalance = env
            .storage()
            .instance()
            .get(&DataKey::Balance)
            .expect("contract has no balance");

        if env.ledger().timestamp() < claimable_balance.unlock_time {
            panic!("time predicate is not fulfilled");
        }

        if claimant != claimable_balance.claimant {
            panic!("claimant is not allowed to claim this balance");
        }

        // Transfer the stored token amount to claimant.
        token::Client::new(&env, &claimable_balance.token).transfer(
            &env.current_contract_address(),
            &claimant,
            &claimable_balance.amount,
        );

        // Remove the balance entry to prevent any further claims.
        env.storage().instance().remove(&DataKey::Balance);
    }
}

mod test;
