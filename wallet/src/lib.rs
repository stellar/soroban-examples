//! This contract implements a simple smart wallet and mainly demonstrates more
//! complex auth scheme with multiple signers that authorize payments in immediate
//! or delayed (async) fashion.
#![no_std]

use soroban_auth::{verify, Identifier, Signature};
use soroban_sdk::{
    contracterror, contractimpl, contracttype, map, symbol, vec, BigInt, BytesN, Env, Map, Vec,
};
mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    // Weight assigned to a wallet admin.
    AdminW(Identifier),
    // Threshold (minimum sum of weights) for execution a transaction.
    Threshold,
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

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    // Initialization errors
    InvalidAdminWeight = 1,
    AdminWeightsBelowThreshold = 2,
    InvalidAdminCount = 3,
    InvalidThreshold = 4,
    AlreadyInitialized = 5,
    // Payment errors
    StoredPaymentMismatch = 6,
    DuplicateSigner = 7,
    UnauthorizedSigner = 8,
    PaymentAlreadyExecuted = 9,
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
    pub fn initialize(env: Env, admins: Vec<Admin>, threshold: u32) -> Result<(), Error> {
        check_initialization_params(&env, &admins, threshold)?;

        let mut weight_sum = 0;
        for maybe_admin in admins.iter() {
            let admin = maybe_admin.unwrap();
            if admin.weight == 0 {
                return Err(Error::InvalidAdminWeight);
            }
            if admin.weight > MAX_WEIGHT {
                return Err(Error::InvalidAdminWeight);
            }
            weight_sum += admin.weight;
            // Record admin weight (and effectively admin identifier too).
            env.data().set(DataKey::AdminW(admin.id), admin.weight);
        }
        // Do a basic sanity check to make sure we don't create a locked wallet.
        if weight_sum < threshold {
            return Err(Error::AdminWeightsBelowThreshold);
        }
        env.data().set(DataKey::Threshold, threshold);
        Ok(())
    }

    // Stores a provided payment or executes it when enough signer weight is
    // accumulated.
    // Returns `true` when the payment was executed and `false` otherwise.
    //
    // Every wallet admin signs `pay` as if it was called by them only, i.e.
    // they should sign `pay` function call with argument tuple of
    // `(admin_id, payment_id, payment)`. Then the signatures of the wallet
    // admins can be batched together in the same `pay` call.
    // This allows using the same signature set in any `pay` call scenario,
    // i.e. it's possible to execute the payment immediately after gathering all
    // the signatures off-chain, or it's possible to call `pay` for every admin
    // separately until it executes, or any combinaton of the above options.
    //
    // Note on replay prevention: this call doesn't need additional replay
    // prevention (nonces) as only the first call of `pay` per signer is
    //  meaningful (all the further calls will just fail).
    pub fn pay(
        env: Env,
        signatures: Vec<Signature>,
        payment_id: i64,
        payment: Payment,
    ) -> Result<bool, Error> {
        let mut weight_sum =
            validate_and_compute_signature_weight(&env, &signatures, payment_id, &payment)?;
        let mut is_existing_payment = false;
        let mut signer_ids = vec![&env];
        if let Some(maybe_previous_signers) = env.data().get(&DataKey::PaySigners(payment_id)) {
            is_existing_payment = true;
            // If there were previous signers for this payment id, we need to check that
            // the payment still hasn't been executed (it should be removed on execution)
            // and that it matches the payment signed by the new signers.
            let stored_payment: Payment = env
                .data()
                .get(&DataKey::Payment(payment_id))
                .ok_or(Error::PaymentAlreadyExecuted)?
                .unwrap();
            if stored_payment != payment {
                return Err(Error::StoredPaymentMismatch);
            }
            let previous_signers: WeightedSigners = maybe_previous_signers.unwrap();
            signer_ids = previous_signers.signers;
            // Check that no new signers have already signed this payment and
            // panic if that's not the case.
            // This is only one option of how to handle this; an alternative approach
            // сould be to only account for weight of the new signers, but that's likely
            // more error-prone (there shouldn't be a reason for an admin to
            // resubmit the signature).
            for maybe_signature in signatures.iter() {
                let id = maybe_signature.unwrap().identifier(&env);
                if signer_ids.contains(&id) {
                    return Err(Error::DuplicateSigner);
                }
            }
            weight_sum += previous_signers.weight;
        }

        for maybe_signature in signatures.iter() {
            signer_ids.push_back(maybe_signature.unwrap().identifier(&env));
        }
        // Update signer data. This also serves as a protection from
        // re-executing the payment with the same id (a separate entry could
        // serve this purpose as well).
        env.data().set(
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
            env.data().remove(&DataKey::Payment(payment_id));
            return Ok(true);
        }
        if !is_existing_payment {
            env.data().set(DataKey::Payment(payment_id), payment);
        }

        Ok(false)
    }
}

fn check_initialization_params(
    env: &Env,
    admins: &Vec<Admin>,
    threshold: u32,
) -> Result<(), Error> {
    if threshold == 0 || threshold > MAX_WEIGHT * MAX_ADMINS {
        return Err(Error::InvalidThreshold);
    }
    if admins.len() == 0 || admins.len() > MAX_ADMINS {
        return Err(Error::InvalidAdminCount);
    }
    if env.data().has(DataKey::Threshold) {
        return Err(Error::AlreadyInitialized);
    }
    Ok(())
}

// Performs auth and duplication check on the provided signatures and
// returns their combined weight.
fn validate_and_compute_signature_weight(
    env: &Env,
    signatures: &Vec<Signature>,
    payment_id: i64,
    payment: &Payment,
) -> Result<u32, Error> {
    let mut weight_sum = 0;
    let mut unique_ids: Map<Identifier, ()> = map![&env];

    for maybe_signature in signatures.iter() {
        let signature = maybe_signature.unwrap();
        let id = signature.identifier(&env);
        // Accumulate the weights and take care of non-authorized accounts
        // at the same time (non-authorized accounts won't have weight).
        weight_sum += read_weight(env, &id)?;

        verify(
            &env,
            &signature,
            symbol!("pay"),
            (&id, &payment_id, payment),
        );
        unique_ids.set(id, ());
    }
    if unique_ids.len() != signatures.len() {
        return Err(Error::DuplicateSigner);
    }

    Ok(weight_sum)
}

fn execute_payment(env: &Env, payment: Payment) {
    let client = token::Client::new(&env, payment.token);
    client.xfer(
        &Signature::Invoker,
        &BigInt::zero(&env),
        &payment.receiver,
        &payment.amount,
    );
}

fn read_threshold(env: &Env) -> u32 {
    env.data().get_unchecked(DataKey::Threshold).unwrap()
}

fn read_weight(env: &Env, id: &Identifier) -> Result<u32, Error> {
    Ok(env
        .data()
        .get(DataKey::AdminW(id.clone()))
        .ok_or(Error::UnauthorizedSigner)?
        .unwrap())
}

mod test;
