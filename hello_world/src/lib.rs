#![no_std]
use stellar_contract_sdk::{contractimpl, contracttype, vec, Env, Symbol, Vec};

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

pub struct HelloContract;

#[contractimpl(export_if = "export")]
impl HelloContract {
    pub fn hello(env: Env, recipient: Recipient) -> (Vec<Symbol>, u32) {
        let greeting_words = vec![&env, Symbol::from_str("Hello")];

        let recipient_words = match recipient {
            Recipient::World => vec![&env, Symbol::from_str("World")],
            Recipient::Person(ref p) => vec![&env, p.first, p.last],
        };

        let words = vec![&env, greeting_words, recipient_words].concat();

        let count: u32 = Self::increment(&env, &recipient);

        (words, count)
    }

    fn increment(env: &Env, recipient: &Recipient) -> u32 {
        let mut count: u32 = 1;
        if env.contract_data().has(recipient.clone()) {
            let prev_count: u32 = env.contract_data().get(recipient.clone());
            count += prev_count;
        }
        env.contract_data().set(recipient.clone(), count);
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use stellar_contract_sdk::{vec, Binary, Env, IntoVal};

    #[test]
    fn test() {
        let env = Env::default();
        let contract_id = Binary::from_array(&env, [0; 32]);
        env.register_contract(contract_id.clone(), HelloContract);

        let (words, count) =
            __hello::call_external(&env, &contract_id, &Recipient::World.into_val(&env));
        assert_eq!(
            words,
            vec![&env, Symbol::from_str("Hello"), Symbol::from_str("World")]
        );
        assert_eq!(count, 1);

        let (words, count) =
            __hello::call_external(&env, &contract_id, &Recipient::World.into_val(&env));
        assert_eq!(
            words,
            vec![&env, Symbol::from_str("Hello"), Symbol::from_str("World")]
        );
        assert_eq!(count, 2);

        let (words, count) = __hello::call_external(
            &env,
            &contract_id,
            &Recipient::Person(Person {
                first: Symbol::from_str("Sour"),
                last: Symbol::from_str("Bun"),
            })
            .into_val(&env),
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
        assert_eq!(count, 1);
    }
}
