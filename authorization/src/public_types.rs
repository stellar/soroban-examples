use soroban_sdk::{contracttype, BytesN, Env, EnvVal, Symbol, Vec};

pub type U256 = BytesN<32>;
pub type U512 = BytesN<64>;

#[derive(Clone)]
#[contracttype]
pub struct KeyedEd25519Signature {
    pub public_key: U256,
    pub signature: U512,
}

pub type AccountAuthorization = Vec<KeyedEd25519Signature>;

#[derive(Clone)]
#[contracttype]
pub struct KeyedAccountAuthorization {
    pub public_key: U256,
    pub signatures: AccountAuthorization,
}

#[derive(Clone)]
#[contracttype]
pub enum KeyedAuthorization {
    Contract,
    Ed25519(KeyedEd25519Signature),
    Account(KeyedAccountAuthorization),
}

impl KeyedAuthorization {
    pub fn get_identifier(&self, env: &Env) -> Identifier {
        match self {
            KeyedAuthorization::Contract => Identifier::Contract(env.get_invoking_contract()),
            KeyedAuthorization::Ed25519(kea) => Identifier::Ed25519(kea.public_key.clone()),
            KeyedAuthorization::Account(kaa) => Identifier::Account(kaa.public_key.clone()),
        }
    }
}

#[derive(Clone, PartialEq)]
#[contracttype]
pub enum Identifier {
    Contract(U256),
    Ed25519(U256),
    Account(U256),
}

// TODO: This is missing fields
#[derive(Clone)]
#[contracttype]
pub struct MessageV0 {
    pub function: Symbol,
    pub parameters: Vec<EnvVal>,
}

#[derive(Clone)]
#[contracttype]
pub enum Message {
    V0(MessageV0),
}
