#![no_std]
use soroban_sdk::{contractimpl, contracttype, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Name {
    None,
    First(First),
    FirstLast(FirstLast),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct First {
    pub first: Symbol,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FirstLast {
    pub first: Symbol,
    pub last: Symbol,
}

pub struct CustomTypesContract;

const NAME: Symbol = Symbol::from_str("NAME");

#[contractimpl(export_if = "export")]
impl CustomTypesContract {
    pub fn store(env: Env, name: Name) {
        env.contract_data().set(NAME, name);
    }

    pub fn retrieve(env: Env) -> Name {
        env.contract_data()
            .get(NAME) // Get the value associated with key NAME.
            .unwrap_or(Ok(Name::None)) // If no value, use None instead.
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, FixedBinary};

    #[test]
    fn test() {
        let env = Env::default();
        let contract_id = FixedBinary::from_array(&env, [0; 32]);
        env.register_contract(&contract_id, CustomTypesContract);

        assert_eq!(retrieve::invoke(&env, &contract_id), Name::None);

        store::invoke(
            &env,
            &contract_id,
            &Name::First(First {
                first: Symbol::from_str("firstonly"),
            }),
        );

        assert_eq!(
            retrieve::invoke(&env, &contract_id),
            Name::First(First {
                first: Symbol::from_str("firstonly"),
            }),
        );

        store::invoke(
            &env,
            &contract_id,
            &Name::FirstLast(FirstLast {
                first: Symbol::from_str("first"),
                last: Symbol::from_str("last"),
            }),
        );

        assert_eq!(
            retrieve::invoke(&env, &contract_id),
            Name::FirstLast(FirstLast {
                first: Symbol::from_str("first"),
                last: Symbol::from_str("last"),
            }),
        );
    }
}
