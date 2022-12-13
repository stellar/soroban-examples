#![no_std]

use soroban_sdk::{contractimpl, contracttype, Account, BytesN, Env, Vec};

mod atomic_swap {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/soroban_atomic_swap_contract.wasm"
    );
}

#[derive(Clone)]
#[contracttype]
pub struct AccountSwap {
    pub account: Account,
    pub amount: i128,
    pub min_recv: i128,
}

pub struct AtomicMultiSwapContract;

#[contractimpl]
impl AtomicMultiSwapContract {
    // Swap token A for token B atomically. Settle for the minimum requested price
    // for each party (this is an arbitrary choice to demonstrate the power of
    // approve; full amounts could be swapped as well).
    pub fn swap(
        env: Env,
        swap_contract: BytesN<32>,
        token_a: BytesN<32>,
        token_b: BytesN<32>,
        accounts_a: Vec<AccountSwap>,
        mut accounts_b: Vec<AccountSwap>,
    ) {
        let swap_client = atomic_swap::Client::new(&env, &swap_contract);
        for acc_a in accounts_a.iter() {
            let acc_a = acc_a.unwrap();
            for i in 0..accounts_b.len() {
                let acc_b = accounts_b.get(i).unwrap().unwrap();

                if acc_a.amount >= acc_b.min_recv && acc_a.min_recv <= acc_b.amount {
                    swap_client.swap(
                        &acc_a.account,
                        &acc_b.account,
                        &token_a,
                        &token_b,
                        &acc_a.amount,
                        &acc_a.min_recv,
                        &acc_b.amount,
                        &acc_b.min_recv,
                    );
                    accounts_b.remove(i);
                    break;
                }
            }
        }
    }
}

mod test;
