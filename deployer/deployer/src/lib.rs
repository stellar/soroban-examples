#![no_std]

use soroban_sdk::{contractimpl, Bytes, BytesN, Env, RawVal, Symbol, Vec};

pub struct Deployer;

#[contractimpl]
impl Deployer {
    /// Deploy the contract wasm and after deployment invoke the init function
    /// of the contract with the given arguments. Returns the contract ID and
    /// result of the init function.
    pub fn deploy(
        env: Env,
        salt: Bytes,
        wasm_hash: BytesN<32>,
        init_fn: Symbol,
        init_args: Vec<RawVal>,
    ) -> (BytesN<32>, RawVal) {
        // Deploy the contract using the installed WASM code with given hash.
        let id = env.deployer().with_current_contract(salt).deploy(wasm_hash);
        // Invoke the init function with the given arguments.
        let res: RawVal = env.invoke_contract(&id, &init_fn, init_args);
        // Return the contract ID of the deployed contract and the result of
        // invoking the init result.
        (id, res)
    }
}

mod test;
