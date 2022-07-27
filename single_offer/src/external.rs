#![cfg(feature = "testutils")]

use ed25519_dalek::{Keypair, Signer};
use num_bigint::BigInt;
use std::vec::Vec;
use stellar_contract_sdk::{Binary, Env, TryIntoVal};
use stellar_token_contract::external::{Authorization, Identifier, U256};
use stellar_xdr::{HostFunction, ScMap, ScMapEntry, ScObject, ScVal, WriteXdr};

use crate::Price;

pub fn register_test_contract(e: &Env, contract_id: &U256) {
    let mut bin = Binary::new(e);
    for b in contract_id {
        bin.push(*b);
    }

    e.register_contract(bin.into(), crate::SingleOffer {});
}
pub enum MessageWithoutNonce {
    Withdraw(BigInt),
    UpdatePrice(u32, u32),
}
pub struct Message(pub BigInt, pub MessageWithoutNonce);

impl TryInto<ScVal> for &Message {
    type Error = ();
    fn try_into(self) -> Result<ScVal, Self::Error> {
        let mut map = Vec::new();
        match self {
            Message(nonce, MessageWithoutNonce::Withdraw(amount)) => {
                map.push(ScMapEntry {
                    key: "domain".try_into()?,
                    val: 0u32.into(),
                });
                map.push(ScMapEntry {
                    key: "nonce".try_into()?,
                    val: nonce.try_into()?,
                });
                map.push(ScMapEntry {
                    key: "parameters".try_into()?,
                    val: (amount,).try_into()?,
                });
            }
            Message(nonce, MessageWithoutNonce::UpdatePrice(n, d)) => {
                map.push(ScMapEntry {
                    key: "domain".try_into()?,
                    val: 1u32.into(),
                });
                map.push(ScMapEntry {
                    key: "nonce".try_into()?,
                    val: nonce.try_into()?,
                });
                map.push(ScMapEntry {
                    key: "parameters".try_into()?,
                    val: (n, d).try_into()?,
                });
            }
        };
        let scmap = ScVal::Object(Some(ScObject::Map(ScMap(map.try_into().map_err(|_| ())?))));
        ("V0", scmap).try_into()
    }
}

pub type U512 = [u8; 64];

impl Message {
    pub fn sign(&self, kp: &Keypair) -> Result<U512, ()> {
        let mut buf = Vec::<u8>::new();
        let val: ScVal = self.try_into()?;
        val.write_xdr(&mut buf).map_err(|_| ())?;
        Ok(kp.sign(&buf).to_bytes())
    }
}

pub fn initialize(
    e: &mut Env,
    contract_id: &U256,
    admin: Identifier,
    token_a: &U256,
    token_b: &U256,
    n: u32,
    d: u32,
) {
    e.invoke_contract_external(
        HostFunction::Call,
        (contract_id, "initialize", &admin, token_a, token_b, n, d)
            .try_into()
            .unwrap(),
    );
}

pub fn nonce(e: &mut Env, contract_id: &U256) -> BigInt {
    e.invoke_contract_external(
        HostFunction::Call,
        (contract_id, "nonce").try_into().unwrap(),
    )
    .try_into()
    .unwrap()
}

pub fn trade(e: &mut Env, contract_id: &U256, to: Identifier, min: u32) {
    e.invoke_contract_external(
        HostFunction::Call,
        (contract_id, "trade", &to, min).try_into().unwrap(),
    );
}

pub fn withdraw(e: &mut Env, contract_id: &U256, admin: Authorization, amount: &BigInt) {
    e.invoke_contract_external(
        HostFunction::Call,
        (contract_id, "withdraw", &admin, amount)
            .try_into()
            .unwrap(),
    );
}

pub fn updt_price(e: &mut Env, contract_id: &U256, admin: Authorization, n: &u32, d: &u32) {
    e.invoke_contract_external(
        HostFunction::Call,
        (contract_id, "updt_price", &admin, n, d)
            .try_into()
            .unwrap(),
    );
}

pub fn get_price(e: &mut Env, contract_id: &U256) -> Price {
    e.invoke_contract_external(
        HostFunction::Call,
        (contract_id, "get_price").try_into().unwrap(),
    )
    .try_into_val(e)
    .unwrap()
}
