#![no_std]
use soroban_sdk::{
    contract, contractclient, contracterror, contractimpl, contracttype, Address, Env, IntoVal,
};

#[contractclient(name = "MintClient")]
trait MintInterface {
    fn mint(env: Env, to: Address, amount: i128);
}

#[contracterror]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Error {
    NotAuthorizedMinter = 1,
    DailyLimitInsufficient = 2,
    NegativeAmount = 3,
}

#[contracttype]
pub enum StorageKey {
    /// Admin. Value is an Address.
    Admin,
    /// Minters are stored keyed by the contract and minter addresses. Value is
    /// a MinterConfig.
    Minter(Address, Address),
    /// Minter stats are stored keyed by contract and minter addresses, epoch
    /// length, and epoch, which is the ledger number divided by the number of
    /// ledgers in the epoch.  Value is a MinterStats.
    MinterStats(Address, Address, u32, u32),
}

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MinterConfig {
    limit: i128,
    epoch_length: u32,
}

#[contracttype]
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MinterStats {
    consumed_limit: i128,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Set the admin.
    pub fn set_admin(env: Env, new_admin: Address) {
        if let Some(admin) = env
            .storage()
            .instance()
            .get::<_, Address>(&StorageKey::Admin)
        {
            admin.require_auth();
        };
        env.storage().instance().set(&StorageKey::Admin, &new_admin);
    }

    /// Return the admin address.
    pub fn admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get::<_, Address>(&StorageKey::Admin)
            .unwrap()
    }

    /// Set the config of a minter for the given contract. Requires auth from
    /// the admin.
    pub fn set_minter(env: Env, contract: Address, minter: Address, config: MinterConfig) {
        Self::admin(env.clone()).require_auth();
        env.storage()
            .persistent()
            .set(&StorageKey::Minter(contract, minter), &config);
    }

    /// Returns the config, current epoch, and current epoch's stats for a
    /// minter.
    pub fn minter(
        env: Env,
        contract: Address,
        minter: Address,
    ) -> Result<(MinterConfig, u32, MinterStats), Error> {
        let config = env
            .storage()
            .persistent()
            .get::<_, MinterConfig>(&StorageKey::Minter(contract.clone(), minter.clone()))
            .ok_or(Error::NotAuthorizedMinter)?;
        let epoch = env.ledger().sequence() / config.epoch_length;
        let stats = env
            .storage()
            .temporary()
            .get::<_, MinterStats>(&StorageKey::MinterStats(
                contract.clone(),
                minter.clone(),
                config.epoch_length,
                epoch,
            ))
            .unwrap_or_default();
        Ok((config, epoch, stats))
    }

    /// Calls the 'mint' function of the 'contract' with 'to' and 'amount'.
    /// Authorized by the 'minter'. Uses some of the authorized 'minter's
    /// current epoch's limit.
    pub fn mint(
        env: Env,
        contract: Address,
        minter: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), Error> {
        // Verify minter is authenticated, and authorizing args.
        minter.require_auth_for_args((&contract, &to, amount).into_val(&env));

        // Verify amount is positive.
        if amount < 0 {
            return Err(Error::NegativeAmount);
        }

        // Verify minter is authorized by contract.
        let admin = Self::admin(env.clone());
        if admin != minter {
            let Some(config) = env
                .storage()
                .persistent()
                .get::<_, MinterConfig>(&StorageKey::Minter(contract.clone(), minter.clone()))
            else {
                return Err(Error::NotAuthorizedMinter);
            };

            // Check and track daily limit.
            let epoch = env.ledger().sequence() / config.epoch_length;
            let minter_stats_key = StorageKey::MinterStats(
                contract.clone(),
                minter.clone(),
                config.epoch_length,
                epoch,
            );
            let minter_stats = env
                .storage()
                .temporary()
                .get::<_, MinterStats>(&minter_stats_key)
                .unwrap_or_default();
            let new_minter_stats = MinterStats {
                consumed_limit: minter_stats.consumed_limit + amount,
            };
            if new_minter_stats.consumed_limit > config.limit {
                return Err(Error::DailyLimitInsufficient);
            }
            env.storage()
                .temporary()
                .set::<_, MinterStats>(&minter_stats_key, &new_minter_stats);
            env.storage()
                .temporary()
                .extend_ttl(&minter_stats_key, 0, epoch * config.epoch_length);
        }

        // Perform the mint.
        let client = MintClient::new(&env, &contract);
        client.mint(&to, &amount);
        Ok(())
    }
}

mod test;
