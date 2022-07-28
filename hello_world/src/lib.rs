#![no_std]
use soroban_sdk::{contractimpl, contracttype, vec, Env, Symbol, Vec};

pub struct HelloContract;

#[contractimpl(export_if = "export")]
impl HelloContract {
    pub fn hello(env: Env, to: Recipient) -> Vec<Symbol> {
        const GREETING: Symbol = Symbol::from_str("Hello");

        match to {
            Recipient::World => vec![&env, GREETING, Symbol::from_str("World")],
            Recipient::Person(ref p) => vec![&env, GREETING, p.first, p.last],
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Recipient {
    World,
    Person(Person),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Person {
    pub first: Symbol,
    pub last: Symbol,
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

        let words = hello::invoke(&env, &contract_id, &Recipient::World);
        assert_eq!(
            words,
            vec![&env, Symbol::from_str("Hello"), Symbol::from_str("World"),]
        );

        let words = hello::invoke(
            &env,
            &contract_id,
            &Recipient::Person(Person {
                first: Symbol::from_str("Sour"),
                last: Symbol::from_str("Bun"),
            }),
        );
        assert_eq!(
            words,
            vec![
                &env,
                Symbol::from_str("Hello"),
                Symbol::from_str("Sour"),
                Symbol::from_str("Bun")
            ]
        );
    }
}
