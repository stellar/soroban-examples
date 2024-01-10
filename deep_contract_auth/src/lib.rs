#![no_std]
/// This example demonstrates how a contract can authorize deep subcontract
/// calls on its behalf.
///
/// By default, only direct calls that contract makes are authorized. However,
/// in some scenarios one may want to authorize a deeper call (a common example
/// would be token transfer).
///
/// Here we provide the abstract example: contract A calls contract B, then
/// contract B calls contract C. Both contract B and contract C `require_auth`
/// for contract A address and contract A provides proper authorization to make
/// the calls succeed.

pub mod contract_a {

    use soroban_sdk::{
        auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
        contract, contractimpl, vec, Address, Env, IntoVal, Symbol,
    };

    use crate::contract_b::ContractBClient;

    #[contract]
    pub struct ContractA;

    #[contractimpl]
    impl ContractA {
        pub fn call_b(env: Env, contract_b_address: Address, contract_c_address: Address) {
            // This function authorizes sub-contract calls that are made from
            // the next call A performs on behalf of the current contract.
            // Note, that these *do not* contain direct calls because they are
            // always authorized. So here we pre-authorize call of contract C
            // that will be performed by contract B.
            env.authorize_as_current_contract(vec![
                &env,
                InvokerContractAuthEntry::Contract(SubContractInvocation {
                    context: ContractContext {
                        contract: contract_c_address.clone(),
                        fn_name: Symbol::new(&env, "authorized_fn_c"),
                        args: (env.current_contract_address(),).into_val(&env),
                    },
                    // `sub_invocations` can be used to authorize even deeper
                    // calls.
                    sub_invocations: vec![&env],
                }),
            ]);
            let client = ContractBClient::new(&env, &contract_b_address);
            client.authorized_fn_b(&env.current_contract_address(), &contract_c_address);
        }
    }
}

pub mod contract_b {
    use soroban_sdk::{contract, contractimpl, Address, Env};

    use crate::contract_c::ContractCClient;

    #[contract]
    pub struct ContractB;

    #[contractimpl]
    impl ContractB {
        pub fn authorized_fn_b(env: Env, authorizer: Address, contract_c_address: Address) {
            authorizer.require_auth();
            let client = ContractCClient::new(&env, &contract_c_address);
            client.authorized_fn_c(&authorizer);
        }
    }
}

pub mod contract_c {

    use soroban_sdk::{contract, contractimpl, Address, Env};

    #[contract]
    pub struct ContractC;

    #[contractimpl]
    impl ContractC {
        pub fn authorized_fn_c(_env: Env, authorizer: Address) {
            authorizer.require_auth();
        }
    }
}

mod test;
