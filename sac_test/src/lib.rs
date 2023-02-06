#![no_std]
use soroban_sdk::{contractimpl, Address, BytesN, Env};

mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}

pub struct SACTest;

#[contractimpl]
impl SACTest {
    pub fn burn_self(env: Env, token: BytesN<32>, amount: i128) {
        let client = token::Client::new(&env, &token);
        client.burn(&env.current_contract_address(), &amount);
    }

    pub fn xfer(env: Env, token: BytesN<32>, to: Address, amount: i128) {
        let client = token::Client::new(&env, &token);
        client.xfer(&env.current_contract_address(), &to, &amount);
    }
}

#[test]
fn test() {
    use soroban_sdk::testutils::Address as _;

    let env: Env = Default::default();
    let token_admin = Address::random(&env);
    let token = token::Client::new(
        &env,
        &env.register_stellar_asset_contract(token_admin.clone()),
    );

    let contract = SACTestClient::new(&env, &env.register_contract(None, SACTest));
    let contract_address = Address::from_contract_id(&env, &contract.contract_id);
    token.mint(&token_admin, &contract_address, &1000);

    contract.burn_self(&token.contract_id, &400);
    assert_eq!(token.balance(&contract_address), 600);

    let user = Address::random(&env);
    contract.xfer(&token.contract_id, &user, &100);
    assert_eq!(token.balance(&contract_address), 500);
    assert_eq!(token.balance(&user), 100);
}
