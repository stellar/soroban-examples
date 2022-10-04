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
    /// Set the admin invoker. May be called only once.
    pub fn init(env: Env, admin: Invoker) {
        if env.data().has(DataKey::Admin) {
            panic!("admin is already set")
        }

        env.data().set(DataKey::Admin, admin);
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
        if env.invoker() != env.data().get(DataKey::Admin).unwrap().unwrap() {
            panic!("invoker is not admin")
        }

        env.data().set(DataKey::SavedNum(id), num);
    }
}
