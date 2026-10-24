#![no_std]
use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, Address, Bytes, BytesN,
    Env, Vec,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Invalid range provided: min must be less than or equal to max.
    InvalidRange = 1,
    /// Player already has an active, unrevealed commitment.
    AlreadyCommitted = 2,
    /// Player does not have an active commitment to reveal.
    NotCommitted = 3,
    /// Reveal attempted too early; must wait until the target ledger.
    RevealTooEarly = 4,
    /// This commitment has already been revealed.
    AlreadyRevealed = 5,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Commitment {
    pub player: Address,
    pub committed_seed: BytesN<32>,
    pub reveal_ledger: u32,
    pub revealed_number: Option<u64>,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Commitment(Address),
}

#[contractevent(topics = ["prng", "commit"])]
pub struct CommitEvent {
    pub player: Address,
    pub reveal_ledger: u32,
}

#[contractevent(topics = ["prng", "reveal"])]
pub struct RevealEvent {
    pub player: Address,
    pub number: u64,
}

#[contract]
pub struct PrngContract;

#[contractimpl]
impl PrngContract {
    /// Generate a pseudorandom u64 within an inclusive range [min, max].
    ///
    /// Note: While convenient, calling this directly within an economically
    /// valuable single-turn transaction makes the call susceptible to
    /// "blind-rerun" attacks (where an attacker contract aborts via try_call
    /// if the outcome is unfavorable and retries until success).
    /// For high-stakes decisions, prefer the two-phase commit-reveal scheme below.
    pub fn gen_number(env: Env, min: u64, max: u64) -> Result<u64, Error> {
        if min > max {
            return Err(Error::InvalidRange);
        }
        Ok(env.prng().gen_range(min..=max))
    }

    /// Shuffles a vector of numbers randomly using the PRNG.
    pub fn shuffle_deck(env: Env, mut deck: Vec<u32>) -> Vec<u32> {
        env.prng().shuffle(&mut deck);
        deck
    }

    /// Phase 1: Commit.
    ///
    /// The player registers a random commitment. The contract records the current
    /// player, generates a private 32-byte seed from the PRNG, and locks the
    /// commitment to a future ledger (`current_ledger + 1`).
    ///
    /// Because the outcome is not calculated or revealed in this transaction,
    /// an attacker cannot inspect or try_call-abort this invocation to bias the result.
    pub fn commit(env: Env, player: Address) -> Result<(), Error> {
        player.require_auth();

        let key = DataKey::Commitment(player.clone());
        if let Some(existing) = env.storage().instance().get::<_, Commitment>(&key) {
            if existing.revealed_number.is_none() {
                return Err(Error::AlreadyCommitted);
            }
        }

        let committed_seed: BytesN<32> = env.prng().gen();
        let reveal_ledger = env.ledger().sequence() + 1;

        let commitment = Commitment {
            player: player.clone(),
            committed_seed,
            reveal_ledger,
            revealed_number: None,
        };

        env.storage().instance().set(&key, &commitment);

        CommitEvent {
            player,
            reveal_ledger,
        }
        .publish(&env);

        Ok(())
    }

    /// Phase 2: Reveal.
    ///
    /// After the target ledger sequence is reached (`current_ledger >= reveal_ledger`),
    /// the commitment can be revealed. The contract re-seeds the PRNG with the committed
    /// seed and draws the random number in [min, max].
    ///
    /// Because the seed was fixed in a previous transaction and cannot be changed,
    /// the revealed number is deterministic with respect to the commitment, preventing
    /// replay / retry exploitation.
    pub fn reveal(env: Env, player: Address, min: u64, max: u64) -> Result<u64, Error> {
        if min > max {
            return Err(Error::InvalidRange);
        }

        let key = DataKey::Commitment(player.clone());
        let mut commitment: Commitment = env
            .storage()
            .instance()
            .get(&key)
            .ok_or(Error::NotCommitted)?;

        if commitment.revealed_number.is_some() {
            return Err(Error::AlreadyRevealed);
        }

        if env.ledger().sequence() < commitment.reveal_ledger {
            return Err(Error::RevealTooEarly);
        }

        // Reseed PRNG with the pre-committed seed.
        let seed_bytes = Bytes::from_slice(&env, &commitment.committed_seed.to_array());
        env.prng().seed(seed_bytes);

        let number: u64 = env.prng().gen_range(min..=max);
        commitment.revealed_number = Some(number);
        env.storage().instance().set(&key, &commitment);

        RevealEvent { player, number }.publish(&env);

        Ok(number)
    }

    /// Returns the current commitment for a player, if any.
    pub fn get_commitment(env: Env, player: Address) -> Result<Commitment, Error> {
        let key = DataKey::Commitment(player);
        env.storage()
            .instance()
            .get(&key)
            .ok_or(Error::NotCommitted)
    }
}

#[cfg(test)]
mod test;
