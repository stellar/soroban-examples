#![no_std]
use soroban_sdk::{
    contract, contractclient, contracterror, contractimpl, contracttype, Address, Env, IntoVal,
};

#[contractclient(name = "MintClient")]
trait MintInterface {
    fn mint(env: Env, to: Address, amount: i128);
}

#[contracterror]
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Error {
    NotAuthorizedMinter = 1,
    DailyLimitInsufficient = 2,
}

#[contracttype]
pub enum StorageKey {
    /// Admin. Value is an Address.
    Admin,
    /// Minters are stored keyed by address. Value is a MinterConfig.
    Minter(Address),
    /// Minter stats are stored keyed by day, which is the ledger number divided
    /// by 17,280. Value is a MinterStats.
    MinterStats(Address, u32),
}

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MinterConfig {
    daily_limit: i128,
}

#[contracttype]
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MinterStats {
    consumed_daily_limit: i128,
}

const LEDGERS_PER_DAY: u32 = 17280;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get::<_, Address>(&StorageKey::Admin)
            .unwrap()
    }

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

    pub fn minter(env: Env, minter: Address) -> (MinterConfig, MinterStats) {
        let day = env.ledger().sequence() / LEDGERS_PER_DAY;
        (
            env.storage()
                .persistent()
                .get(&StorageKey::Minter(minter.clone()))
                .unwrap(),
            env.storage()
                .temporary()
                .get::<_, MinterStats>(&StorageKey::MinterStats(minter.clone(), day))
                .unwrap_or_default(),
        )
    }

    pub fn set_minter(env: Env, minter: Address, config: MinterConfig) {
        Self::admin(env.clone()).require_auth();
        env.storage()
            .persistent()
            .set(&StorageKey::Minter(minter), &config);
    }

    pub fn remove_minter(env: Env, minter: Address) {
        Self::admin(env.clone()).require_auth();
        env.storage()
            .persistent()
            .remove(&StorageKey::Minter(minter));
    }

    /// Call the 'mint' function of the 'contract' with 'to' and 'amount'.
    /// Authorized by the 'minter'.
    /// Uses some of the authorized 'minter's daily limit.
    pub fn mint(
        env: Env,
        minter: Address,
        contract: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), Error> {
        // Verify minter is authenticated, and authorizing args.
        minter.require_auth_for_args((&contract, &to, amount).into_val(&env));

        // Verify minter is authorized by contract.
        let admin = Self::admin(env.clone());
        if admin != minter {
            let Some(minter_config) = env
                .storage()
                .persistent()
                .get::<_, MinterConfig>(&StorageKey::Minter(minter.clone()))
            else {
                return Err(Error::NotAuthorizedMinter);
            };

            // Check and track daily limit.
            let day = env.ledger().sequence() / LEDGERS_PER_DAY;
            let minter_stats_key = StorageKey::MinterStats(minter.clone(), day);
            let minter_stats = env
                .storage()
                .temporary()
                .get::<_, MinterStats>(&minter_stats_key)
                .unwrap_or_default();
            let new_minter_stats = MinterStats {
                consumed_daily_limit: minter_stats.consumed_daily_limit + amount,
            };
            if new_minter_stats.consumed_daily_limit > minter_config.daily_limit {
                return Err(Error::DailyLimitInsufficient);
            }
            env.storage()
                .temporary()
                .set::<_, MinterStats>(&minter_stats_key, &new_minter_stats);
            env.storage()
                .temporary()
                .extend_ttl(&minter_stats_key, 0, day * LEDGERS_PER_DAY);
        }

        // Perform the mint.
        let client = MintClient::new(&env, &contract);
        client.mint(&to, &amount);
        Ok(())
    }
}

mod test;
