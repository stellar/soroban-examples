#![no_std]

use soroban_sdk::{contract, contractevent, contractimpl, contracttype, Env, U256};

// ============================================================================
// Storage Keys
// ============================================================================

#[contracttype]
pub enum DataKey {
    TokenU32,
    TokenU256,
}

// ============================================================================
// Events
// ============================================================================

#[contractevent]
pub struct TokenU32Event {
    token_id: u32,
}

#[contractevent]
pub struct TokenU256Event {
    token_id: U256,
}

// ============================================================================
// Contract
// ============================================================================

#[contract]
pub struct TokenIdContract;

#[contractimpl]
impl TokenIdContract {
    // ========================================================================
    // u32 Token ID Functions
    // ========================================================================

    /// Store a u32 token ID.
    pub fn store_u32(env: &Env, token_id: u32) {
        env.storage()
            .persistent()
            .set(&DataKey::TokenU32, &token_id);
    }

    /// Load a u32 token ID from storage.
    pub fn load_u32(env: &Env) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::TokenU32)
            .unwrap_or(0)
    }

    /// Publish an event with a u32 token ID.
    pub fn event_u32(env: &Env, token_id: u32) {
        TokenU32Event { token_id }.publish(env);
    }

    // ========================================================================
    // U256 Token ID Functions
    // ========================================================================

    /// Store a U256 token ID.
    pub fn store_u256(env: &Env, token_id: U256) {
        env.storage()
            .persistent()
            .set(&DataKey::TokenU256, &token_id);
    }

    /// Load a U256 token ID from storage.
    pub fn load_u256(env: &Env) -> U256 {
        env.storage()
            .persistent()
            .get(&DataKey::TokenU256)
            .unwrap_or(U256::from_u32(env, 0))
    }

    /// Publish an event with a U256 token ID.
    pub fn event_u256(env: &Env, token_id: U256) {
        TokenU256Event { token_id }.publish(env);
    }
}

mod test;
