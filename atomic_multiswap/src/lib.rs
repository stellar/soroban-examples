//! This contract performs a batch of atomic token swaps between multiple
//! parties and does a simple price matching.
//! Parties don't need to know each other and also don't need to know their
//! signatures are used in this contract; they sign the `AtomicSwap` contract
//! invocation that guarantees that their token will be swapped with someone
//! while following the price limit.
//! This example demonstrates how authorized calls can be batched together.
#![no_std]

use soroban_sdk::{contractimpl, contracttype, Address, BytesN, Env, Vec};

mod atomic_swap {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/soroban_atomic_swap_contract.wasm"
    );
}

#[derive(Clone)]
#[contracttype]
pub struct SwapSpec {
    pub address: Address,
    pub amount: i128,
    pub min_recv: i128,
}

pub struct AtomicMultiSwapContract;

#[contractimpl]
impl AtomicMultiSwapContract {
    // Swap token A for token B atomically between the parties that want to
    // swap A->B and parties that want to swap B->A.
    // All the parties should have authorized the `swap` via `swap_contract`,
    // but they don't need to authorize `multi_swap` itself.
    pub fn multi_swap(
        env: Env,
        swap_contract: BytesN<32>,
        token_a: BytesN<32>,
        token_b: BytesN<32>,
        swaps_a: Vec<SwapSpec>,
        swaps_b: Vec<SwapSpec>,
    ) {
        let mut swaps_b = swaps_b;
        let swap_client = atomic_swap::Client::new(&env, &swap_contract);
        for acc_a in swaps_a.iter() {
            let acc_a = acc_a.unwrap();
            for i in 0..swaps_b.len() {
                let acc_b = swaps_b.get(i).unwrap().unwrap();

                if acc_a.amount >= acc_b.min_recv && acc_a.min_recv <= acc_b.amount {
                    // As this is a simple 'batching' contract, there is no need
                    // for all swaps to succeed, hence we handle the failures
                    // gracefully to try and clear as many swaps as possible.
                    if swap_client
                        .try_swap(
                            &acc_a.address,
                            &acc_b.address,
                            &token_a,
                            &token_b,
                            &acc_a.amount,
                            &acc_a.min_recv,
                            &acc_b.amount,
                            &acc_b.min_recv,
                        )
                        .is_ok()
                    {
                        swaps_b.remove(i);
                        break;
                    }
                }
            }
        }
    }
}

mod test;
