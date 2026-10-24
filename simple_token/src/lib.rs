#![no_std]
use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, Address, Env, String,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Amount must be strictly greater than zero.
    NonPositiveAmount = 1,
    /// Balance of the sender is insufficient for this transfer.
    InsufficientBalance = 2,
    /// Token decimals cannot exceed 18.
    DecimalTooLarge = 3,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address),
    TotalSupply,
    Name,
    Symbol,
    Decimal,
}

#[contractevent(topics = ["transfer"])]
pub struct TransferEvent {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
}

#[contract]
pub struct SimpleToken;

#[contractimpl]
impl SimpleToken {
    /// Constructor initializes a token with a finite total supply minted to `initial_owner`.
    ///
    /// This demonstrates:
    /// - Constructor pattern (`__constructor`)
    /// - Initializing persistent instance storage
    /// - Establishing an immutable total supply without infinite minting capabilities
    pub fn __constructor(
        env: Env,
        initial_owner: Address,
        initial_supply: i128,
        name: String,
        symbol: String,
        decimal: u32,
    ) {
        if decimal > 18 {
            panic!("Decimal must not be greater than 18");
        }
        if initial_supply <= 0 {
            panic!("Initial supply must be positive");
        }

        // Store metadata in instance storage.
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::Decimal, &decimal);
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &initial_supply);

        // Credit the entire finite supply to the initial owner.
        env.storage()
            .instance()
            .set(&DataKey::Balance(initial_owner.clone()), &initial_supply);

        // Publish an initial transfer event from initial_owner to initial_owner.
        TransferEvent {
            from: initial_owner.clone(),
            to: initial_owner,
            amount: initial_supply,
        }
        .publish(&env);
    }

    /// Returns the balance of an address.
    /// Returns 0 if the address has no balance.
    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Balance(id))
            .unwrap_or(0)
    }

    /// Transfers `amount` from `from` to `to`.
    ///
    /// Demonstrates:
    /// - Caller authorization via `from.require_auth()`
    /// - Input validation (strictly positive amount)
    /// - Balance checks and state updates
    /// - Event emission
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        from.require_auth();

        if amount <= 0 {
            return Err(Error::NonPositiveAmount);
        }

        let from_key = DataKey::Balance(from.clone());
        let from_balance: i128 = env.storage().instance().get(&from_key).unwrap_or(0);

        if from_balance < amount {
            return Err(Error::InsufficientBalance);
        }

        let to_key = DataKey::Balance(to.clone());
        let to_balance: i128 = env.storage().instance().get(&to_key).unwrap_or(0);

        env.storage()
            .instance()
            .set(&from_key, &(from_balance - amount));
        env.storage()
            .instance()
            .set(&to_key, &(to_balance + amount));

        TransferEvent { from, to, amount }.publish(&env);

        Ok(())
    }

    /// Returns the fixed total supply of this token.
    pub fn total_supply(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }

    /// Returns the name of the token.
    pub fn name(env: Env) -> String {
        env.storage().instance().get(&DataKey::Name).unwrap()
    }

    /// Returns the symbol of the token.
    pub fn symbol(env: Env) -> String {
        env.storage().instance().get(&DataKey::Symbol).unwrap()
    }

    /// Returns the number of decimals of the token.
    pub fn decimals(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Decimal).unwrap_or(0)
    }
}

#[cfg(test)]
mod test;
