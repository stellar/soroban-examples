#![no_std]
use soroban_sdk::{contractimpl, contracttype, vec, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Name {
    First(Symbol),
    FirstLast(Name),
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
    pub fn store(env: Env, name: Name) -> Option<Name> {
        env.contract_data().set(NAME, name)
    }

    pub fn retrieve(env: Env, to: Symbol) -> Option<Name> {
        env.contract_data().get(NAME)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{vec, Env, FixedBinary};

    #[test]
    fn test() {
        let env = Env::default();
        let contract_id = FixedBinary::from_array(&env, [0; 32]);
        env.register_contract(&contract_id, HelloContract);

        assert_eq!(
            store::invoke(
                &env,
                &contract_id,
                Name::First(Symbol::from_str("firstonly")),
            ),
            None
        );

        assert_eq!(
            store::invoke(
                &env,
                &contract_id,
                Name::FirstLast(FirstLast {
                    first: Symbol::from_str("first"),
                    last: Symbol::from_str("last")
                }),
            ),
            Name::First(Symbol::from_str("firstonly"))
        );
    }
}
