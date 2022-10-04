#![no_std]

mod test;

use soroban_sdk::{contractimpl, contracttype, BigInt, Env, Invoker};

#[contracttype]
pub enum DataKey {
    SavedNum(Invoker),
    Admin,
}

pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    /// Set the admin invoker.
    ///
    /// May be called only once unauthenticated, and
    /// then only by current admin.
    pub fn set_admin(env: Env, new_admin: Invoker) {
        let admin = Self::admin(&env);
        if let Some(admin) = admin {
            assert_eq!(env.invoker(), admin);
        }
        env.data().set(DataKey::Admin, new_admin);
    }

    /// Set the number for an authenticated invoker.
    pub fn set_num(env: Env, num: BigInt) {
        let id = env.invoker();
        env.data().set(DataKey::SavedNum(id), num);
    }

    /// Get the number for an invoker.
    pub fn num(env: Env, id: Invoker) -> Option<BigInt> {
        env.data().get(DataKey::SavedNum(id)).map(Result::unwrap)
    }

    /// Overwrite any number for an invoker.
    /// Callable only by admin.
    pub fn overwrite(env: Env, id: Invoker, num: BigInt) {
        let admin = Self::admin(&env);
        assert_eq!(Some(env.invoker()), admin);

        env.data().set(DataKey::SavedNum(id), num);
    }

    fn admin(env: &Env) -> Option<Invoker> {
        env.data().get(DataKey::Admin).map(Result::unwrap)
    }
}
