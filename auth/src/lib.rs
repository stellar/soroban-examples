#![no_std]

mod test;

use soroban_sdk::{contractimpl, contracttype, Address, BigInt, Env};

#[contracttype]
pub enum DataKey {
    SavedNum(Address),
    Admin,
}

pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    /// Set the admin Address.
    ///
    /// May be called only once unauthenticated, and
    /// then only by current admin.
    pub fn set_admin(env: Env, new_admin: Address) {
        let admin = Self::admin(&env);
        if let Some(admin) = admin {
            assert_eq!(env.invoker(), admin);
        }
        env.data().set(DataKey::Admin, new_admin);
    }

    /// Set the number for an authenticated address.
    pub fn set_num(env: Env, num: BigInt) {
        let id = env.invoker();
        env.data().set(DataKey::SavedNum(id), num);
    }

    /// Get the number for an Address.
    pub fn num(env: Env, id: Address) -> Option<BigInt> {
        env.data().get(DataKey::SavedNum(id)).map(Result::unwrap)
    }

    /// Overwrite any number for an Address.
    /// Callable only by admin.
    pub fn overwrite(env: Env, id: Address, num: BigInt) {
        let admin = Self::admin(&env);
        assert_eq!(Some(env.invoker()), admin);

        env.data().set(DataKey::SavedNum(id), num);
    }

    fn admin(env: &Env) -> Option<Address> {
        env.data().get(DataKey::Admin).map(Result::unwrap)
    }
}
