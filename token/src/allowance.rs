use crate::storage_types::{AllowanceDataKey, AllowanceValue, DataKey};
use soroban_sdk::{Address, Env};

pub fn read_allowance(e: &Env, from: Address, spender: Address) -> AllowanceValue {
    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    if let Some(allowance) = e.storage().temporary().get::<_, AllowanceValue>(&key) {
        if allowance.live_until_ledger < e.ledger().sequence() {
            AllowanceValue {
                amount: 0,
                live_until_ledger: allowance.live_until_ledger,
            }
        } else {
            allowance
        }
    } else {
        AllowanceValue {
            amount: 0,
            live_until_ledger: 0,
        }
    }
}

pub fn write_allowance(
    e: &Env,
    from: Address,
    spender: Address,
    amount: i128,
    live_until_ledger: u32,
) {
    let allowance = AllowanceValue {
        amount,
        live_until_ledger,
    };

    if amount > 0 && live_until_ledger < e.ledger().sequence() {
        panic!("live_until_ledger is less than ledger seq when amount > 0")
    }

    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    e.storage().temporary().set(&key.clone(), &allowance);

    if amount > 0 {
        let live_for = live_until_ledger
            .checked_sub(e.ledger().sequence())
            .unwrap();

        e.storage().temporary().extend_ttl(&key, live_for, live_for)
    }
}

pub fn spend_allowance(e: &Env, from: Address, spender: Address, amount: i128) {
    let allowance = read_allowance(e, from.clone(), spender.clone());
    if allowance.amount < amount {
        panic!("insufficient allowance");
    }
    if amount > 0 {
        write_allowance(
            e,
            from,
            spender,
            allowance.amount - amount,
            allowance.live_until_ledger,
        );
    }
}
