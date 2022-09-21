#![no_std]

mod auth;

use crate::auth::{PayloadTrait, SignaturePayload, SignaturePayloadV0};
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, contracttype, BigInt, BytesN, Env, Map, RawVal, Symbol, Vec};

fn get_deposit_amounts(
    desired_a: BigInt,
    min_a: BigInt,
    desired_b: BigInt,
    min_b: BigInt,
    reserves: (BigInt, BigInt),
) -> (BigInt, BigInt) {
    if reserves.0 == 0 && reserves.1 == 0 {
        return (desired_a, desired_b);
    }

    let amount_b = desired_a.clone() * reserves.1.clone() / reserves.0.clone();
    if amount_b <= desired_b {
        if amount_b < min_b {
            panic!("amount_b less than min")
        }
        return (desired_a, amount_b);
    } else {
        let amount_a = desired_b.clone() * reserves.0 / reserves.1;
        if amount_a > desired_a || desired_a < min_a {
            panic!("amount_a invalid")
        }
        return (amount_a, desired_b);
    }
}

#[contracttype]
pub struct Deposit {
    pub token: BytesN<32>,
    pub nonce: BigInt,
    pub desired: BigInt,
    pub min: BigInt,
}

pub trait LiquidityPoolRouterTrait {
    fn deposit(
        e: Env,
        to: Identifier,
        deposit_a: Deposit,
        deposit_b: Deposit,
        sigs: Map<Identifier, Signature>,
    );
}

pub struct LiquidityPoolRouterPayload;

#[cfg_attr(feature = "export", contractimpl)]
#[cfg_attr(not(feature = "export"), contractimpl(export = false))]
impl PayloadTrait for LiquidityPoolRouterPayload {
    fn payload(
        e: Env,
        function: Symbol,
        args: Vec<RawVal>,
        callstack: Vec<(BytesN<32>, Symbol)>,
    ) -> SignaturePayload {
        SignaturePayload::V0(SignaturePayloadV0 {
            function,
            contract: e.get_current_contract(),
            network: e.ledger().network_passphrase(),
            args,
            salt: callstack.into(),
        })
    }
}
