//! This contract implements a simple smart wallet and mainly demonstrates more
//! complex auth scheme with multiple signers that authorize payments in immediate
//! or delayed (async) fashion.
#![no_std]
#[cfg(feature = "testutils")]
extern crate std;

use soroban_auth::{
    check_auth, NonceAuth, {Identifier, Signature},
};
use soroban_sdk::{
    contractimpl, contracttype, map, symbol, vec, BigInt, BytesN, Env, IntoVal, Map, Vec,
};
mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_contract.wasm");
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    // Weight assigned to a wallet admin.
    AdminW(Identifier),
    // Threshold (minimum sum of weights) for execution a transaction.
    Threshold,
    Nonce(Identifier),
    // `Payment`s keyed by payment identifier.
    Payment(i64),
    // `WeightedSigners` keyed by payment identifier.
    PaySigners(i64),
}

#[derive(Clone, PartialEq)]
#[contracttype]
pub struct Payment {
    pub receiver: Identifier,
    pub token: BytesN<32>,
    pub amount: BigInt,
}

#[derive(Clone)]
#[contracttype]
pub struct WeightedSigners {
    pub signers: Vec<Identifier>,
    pub weight: u32,
}

#[derive(Clone)]
#[contracttype]
pub struct Admin {
    pub id: Identifier,
    pub weight: u32,
}

const MAX_ADMINS: u32 = 20;
const MAX_WEIGHT: u32 = 100;

pub struct WalletContract;

// Contract usage:
// - Call `initialize` once to setup the contract admins, their weights and
//   payment weight threshold. For simplicity, this setup is immutable.
// - Fund the wallet contract as needed using token contract functionality.
// - To execute the payment:
//   1. Distribute a pair of `(payment_id, Payment)` to the wallet admins for
//      signing. `payment_id` should be unique for every payment. `payment_id`
//      management is not implemented here for the sake of conciseness and could
//      happen both off-chain or in the contract itself.
//   2. Call `pay` one or many times with arbitrary batches of admin signatures
//      until enough admin weight accumulated (i.e. at least `threshold`) to
//      actually execute it.
#[contractimpl]
impl WalletContract {
    // Performs contract intialization.
    // Call `initialize` and supply ids and weights of the admins, as well as the
    // threshold needed to execute payments. The payment may only be executed when
    // unique admins with combined weight exceeding `threshold` have signed it.
    pub fn initialize(env: Env, admins: Vec<Admin>, threshold: u32) {
        check_initialization_params(&env, &admins, threshold);

        let mut weight_sum = 0;
        for maybe_admin in admins.iter() {
            let admin = maybe_admin.unwrap();
            if admin.weight == 0 {
                panic!("weight should be non-zero");
            }
            if admin.weight > MAX_WEIGHT {
                panic!("too high admin weight");
            }
            weight_sum += admin.weight;
            // Record admin weight (and effectively admin identifier too).
            env.contract_data()
                .set(DataKey::AdminW(admin.id), admin.weight);
        }
        // Do a basic sanity check to make sure we don't create a locked wallet.
        if weight_sum < threshold {
            panic!("admin weight is lower than threshold");
        }
        env.contract_data().set(DataKey::Threshold, threshold);
    }

    // Helper to get nonce for any provided admin identifier.
    // The nonce should then be used in `pay`.
    pub fn get_nonce(env: Env, admin: Identifier) -> BigInt {
        read_nonce(&env, admin)
    }

    // Stores a provided payment or executes it when enough signer weight is
    // accumulated.
    // Returns `true` when the payment was executed and `false` otherwise.
    //
    // Every wallet admin signs `pay` as if it was called by them only, i.e.
    // they should sign `pay` function call with argument tuple of
    // `(id, nonce, payment_id, payment)`. Then the signatures and nonces of
    // the wallet admins can be batched together in the same `pay` call.
    // This allows using the same signature set in any `pay` call scenario,
    // i.e. it's possible to execute the payment immediately after gathering all
    // the signatures off-chain, or it's possible to call `pay` for every admin
    // separately until it executes, or any combinaton of the above options.
    pub fn pay(
        env: Env,
        signatures_with_nonces: Vec<(Signature, BigInt)>,
        payment_id: i64,
        payment: Payment,
    ) -> bool {
        let mut weight_sum = validate_and_compute_signature_weight(
            &env,
            &signatures_with_nonces,
            payment_id,
            &payment,
        );
        let mut is_existing_payment = false;
        let mut signer_ids = vec![&env];
        if let Some(maybe_previous_signers) =
            env.contract_data().get(&DataKey::PaySigners(payment_id))
        {
            is_existing_payment = true;
            // If there were previous signers for this payment id, we need to check that
            // the payment still hasn't been executed (it should be removed on execution)
            // and that it matches the payment signed by the new signers.
            let stored_payment: Payment = env
                .contract_data()
                .get_unchecked(&DataKey::Payment(payment_id))
                .unwrap();
            if stored_payment != payment {
                panic!("stored payment doesn't match new payment with same id");
            }
            let previous_signers: WeightedSigners = maybe_previous_signers.unwrap();
            signer_ids = previous_signers.signers;
            // Check that no new signers have already signed this payment and
            // panic if that's not the case.
            // This is only one option of how to handle this; an alternative approach
            // сould be to only account for weight of the new signers, but that's likely
            // more error-prone (there shouldn't be a reason for an admin to sign the same
            // payment twice).
            for maybe_signature_with_nonce in signatures_with_nonces.iter() {
                let id = maybe_signature_with_nonce.unwrap().0.get_identifier(&env);
                if signer_ids.contains(&id) {
                    panic!("one of the signers has already signed this payment");
                }
            }
            weight_sum += previous_signers.weight;
        }

        for signature in signatures_with_nonces.iter() {
            signer_ids.push_back(signature.unwrap().0.get_identifier(&env));
        }
        // Update signer data. This also serves as a protection from
        // re-executing the payment with the same id (that could be a separate
        // entry too).
        env.contract_data().set(
            DataKey::PaySigners(payment_id),
            WeightedSigners {
                signers: signer_ids,
                weight: weight_sum,
            },
        );

        let threshold = read_threshold(&env);
        // When there is enough signature weight to authorize this payment
        // execute the payment immediately.
        if weight_sum >= threshold {
            execute_payment(&env, payment);
            // Remove the payment to mark it executed (signers are still there).
            env.contract_data().remove(&DataKey::Payment(payment_id));
            return true;
        }
        if !is_existing_payment {
            env.contract_data()
                .set(DataKey::Payment(payment_id), payment);
        }

        false
    }
}

fn check_initialization_params(env: &Env, admins: &Vec<Admin>, threshold: u32) {
    if threshold == 0 {
        panic!("threshold has to be non-zero");
    }
    if admins.len() == 0 {
        panic!("at least one admin needs to be provided");
    }
    if admins.len() > MAX_ADMINS {
        panic!("too many admins");
    }
    if threshold > MAX_WEIGHT * MAX_ADMINS {
        panic!("threshold is too high");
    }
    if env.contract_data().has(DataKey::Threshold) {
        panic!("contract has already been initialized");
    }
}

// Performs auth and duplication check on the provided signatures and
// returns their combined weight.
fn validate_and_compute_signature_weight(
    env: &Env,
    signatures_with_nonce: &Vec<(Signature, BigInt)>,
    payment_id: i64,
    payment: &Payment,
) -> u32 {
    let mut weight_sum = 0;
    let mut unique_ids: Map<Identifier, ()> = map![&env];

    for maybe_signature_with_nonce in signatures_with_nonce.iter() {
        let signature_with_nonce = maybe_signature_with_nonce.unwrap();
        let id = signature_with_nonce.0.get_identifier(&env);
        // Accumulate the weights and take care of non-authorized accounts
        // at the same time (non-authorized accounts won't have weight).
        weight_sum += env
            .contract_data()
            .get_unchecked::<DataKey, u32>(DataKey::AdminW(id.clone()))
            .unwrap();

        check_auth(
            &env,
            &NonceForSignature(signature_with_nonce.0.clone()),
            signature_with_nonce.1.clone(),
            symbol!("pay"),
            (&id, &signature_with_nonce.1, &payment_id, payment).into_val(env),
        );
        unique_ids.set(id, ());
    }
    if unique_ids.len() != signatures_with_nonce.len() {
        panic!("duplicate signatures provided");
    }

    weight_sum
}

fn execute_payment(env: &Env, payment: Payment) {
    let client = token::ContractClient::new(&env, payment.token);
    client.xfer(
        &Signature::Contract,
        &BigInt::zero(&env),
        &payment.receiver,
        &payment.amount,
    );
}

fn read_threshold(env: &Env) -> u32 {
    env.contract_data()
        .get_unchecked(DataKey::Threshold)
        .unwrap()
}

fn read_nonce(e: &Env, id: Identifier) -> BigInt {
    let key = DataKey::Nonce(id);
    if let Some(nonce) = e.contract_data().get(key) {
        nonce.unwrap()
    } else {
        BigInt::zero(e)
    }
}

struct NonceForSignature(Signature);

impl NonceAuth for NonceForSignature {
    fn read_nonce(e: &Env, id: Identifier) -> BigInt {
        read_nonce(e, id)
    }

    fn read_and_increment_nonce(&self, e: &Env, id: Identifier) -> BigInt {
        let key = DataKey::Nonce(id.clone());
        let nonce = Self::read_nonce(e, id);
        e.contract_data()
            .set(key, nonce.clone() + BigInt::from_u32(e, 1));
        nonce
    }

    fn signature(&self) -> &Signature {
        &self.0
    }
}

mod test;
