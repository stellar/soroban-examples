#![no_std]
use soroban_sdk::{contractimpl, contracttype, symbol, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Name {
    None,
    FirstLast(FirstLast),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FirstLast {
    pub first: Symbol,
    pub last: Symbol,
}

pub struct CustomTypesContract;

const NAME: Symbol = symbol!("NAME");

#[contractimpl]
impl CustomTypesContract {
    pub fn store(env: Env, name: Name) {
        env.data().set(NAME, name);
    }

    pub fn retrieve(env: Env) -> Name {
        env.data()
            .get(NAME) // Get the value associated with key NAME.
            .unwrap_or(Ok(Name::None)) // If no value, use None instead.
            .unwrap()
    }
}

mod test;
