#![no_std]

use soroban_sdk::{contractimpl, Account, Address, BytesN, Env, IntoVal};

mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}

pub struct AtomicSwapContract;

#[contractimpl]
impl AtomicSwapContract {
    // Swap token A for token B atomically. Settle for the minimum requested price
    // for each party (this is an arbitrary choice to demonstrate the power of
    // approve; full amounts could be swapped as well).
    pub fn swap(
        env: Env,
        a: Account,
        b: Account,
        token_a: BytesN<32>,
        token_b: BytesN<32>,
        amount_a: i128,
        min_b_for_a: i128,
        amount_b: i128,
        min_a_for_b: i128,
    ) {
        if amount_b < min_b_for_a {
            panic!("not enough token B for token A");
        }
        if amount_a < min_a_for_b {
            panic!("not enough token A for token B");
        }

        a.authorize((&token_a, &token_b, amount_a, min_b_for_a).into_val(&env));
        b.authorize((&token_b, &token_a, amount_b, min_a_for_b).into_val(&env));

        move_token(&env, token_a, &a, &b.address(), amount_a, min_a_for_b);
        move_token(&env, token_b, &b, &a.address(), amount_b, min_b_for_a);
    }
}

fn move_token(
    env: &Env,
    token: BytesN<32>,
    from: &Account,
    to: &Address,
    approve_amount: i128,
    xfer_amount: i128,
) {
    let token = token::Client::new(&env, &token);
    let contract_account = env.current_contract_account();
    token.incr_allow(from, &contract_account.address(), &approve_amount);
    token.xfer_from(&contract_account, &from.address(), to, &xfer_amount);
}

mod test;
