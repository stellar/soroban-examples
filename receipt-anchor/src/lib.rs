#![no_std]

//! # Receipt anchor
//!
//! A small example of committing to a contract invocation, so that a third
//! party can later check — from public data alone — that a specific
//! invocation produced a specific result.
//!
//! The contract does one thing. On `anchor` it hashes a fixed tuple of
//! invocation facts:
//!
//! ```text
//! commitment = sha256( "receipt-anchor/1"
//!                    ++ xdr(target)         // the contract that was called
//!                    ++ xdr(fn_name)        // the function that was called
//!                    ++ sha256(xdr(args))   // the arguments, canonically encoded
//!                    ++ sha256(xdr(result)) // the result, canonically encoded
//!                    ++ xdr(ledger) )       // the ledger the invocation happened at
//! ```
//!
//! and stores the receipt under the caller's address. `verify` recomputes the
//! same 32 bytes from the same inputs, so a mismatch is detectable without
//! trusting the caller, the contract, or this code.
//!
//! What this is not: a policy engine, an allowlist, a settlement surface, or a
//! token contract. It moves no funds and grants no authority — it pins a fact
//! about an invocation. Whether an invocation *should* happen is decided by
//! somebody else's policy; this is the layer that records what did happen.

use soroban_sdk::{
    contract, contractimpl, contracttype, xdr::ToXdr, Address, Bytes, BytesN, Env, Symbol, Val, Vec,
};

/// Domain separator. These fixed bytes are the prefix of every commitment
/// preimage, so a hash produced here cannot be confused with a hash of the
/// same data computed for another purpose.
pub const DOMAIN: &[u8] = b"receipt-anchor/1";

/// A stored commitment to one invocation.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Receipt {
    pub target: Address,
    pub fn_name: Symbol,
    pub ledger: u32,
    pub args_hash: BytesN<32>,
    pub result_hash: BytesN<32>,
    pub commitment: BytesN<32>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// One receipt, indexed per caller in the order it was anchored.
    Receipt(Address, u32),
    /// How many receipts a caller has anchored.
    Count(Address),
}

#[contract]
pub struct ReceiptAnchor;

#[contractimpl]
impl ReceiptAnchor {
    /// Anchor one invocation and return its commitment.
    ///
    /// Requires the caller's authorization: a receipt is a statement about
    /// what the caller did, so the caller signs for it. That keeps a stranger
    /// from writing into someone else's receipt list.
    pub fn anchor(
        env: Env,
        caller: Address,
        target: Address,
        fn_name: Symbol,
        args: Vec<Val>,
        result: Val,
        ledger: u32,
    ) -> BytesN<32> {
        caller.require_auth();

        let args_hash = Self::hash_xdr(&env, args.clone());
        let result_hash = Self::hash_xdr(&env, result);
        let commitment = Self::commit_parts(
            &env,
            target.clone(),
            fn_name.clone(),
            args_hash.clone(),
            result_hash.clone(),
            ledger,
        );

        let index = Self::count(env.clone(), caller.clone());
        env.storage().persistent().set(
            &DataKey::Receipt(caller.clone(), index),
            &Receipt {
                target,
                fn_name,
                ledger,
                args_hash,
                result_hash,
                commitment: commitment.clone(),
            },
        );
        env.storage()
            .persistent()
            .set(&DataKey::Count(caller), &(index + 1));

        commitment
    }

    /// Recompute the commitment for one invocation.
    ///
    /// Pure: the same inputs produce the same 32 bytes on any machine, so an
    /// off-chain verifier can derive the expected value itself instead of
    /// trusting a call to this contract.
    pub fn commitment(
        env: Env,
        target: Address,
        fn_name: Symbol,
        args: Vec<Val>,
        result: Val,
        ledger: u32,
    ) -> BytesN<32> {
        let args_hash = Self::hash_xdr(&env, args);
        let result_hash = Self::hash_xdr(&env, result);
        Self::commit_parts(&env, target, fn_name, args_hash, result_hash, ledger)
    }

    /// Check a stored receipt against the invocation it claims to describe.
    ///
    /// Returns `false` on every mismatch: a different argument, a different
    /// result, a different ledger, a different target or function name. There
    /// is no partial credit and no error to ignore.
    pub fn verify(env: Env, receipt: Receipt, args: Vec<Val>, result: Val) -> bool {
        let expected = Self::commitment(
            env,
            receipt.target.clone(),
            receipt.fn_name.clone(),
            args,
            result,
            receipt.ledger,
        );
        expected == receipt.commitment
    }

    /// How many receipts an address has anchored.
    pub fn count(env: Env, caller: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::Count(caller))
            .unwrap_or(0)
    }

    /// Read one of a caller's receipts, in the order it was anchored.
    pub fn receipt(env: Env, caller: Address, index: u32) -> Option<Receipt> {
        env.storage()
            .persistent()
            .get(&DataKey::Receipt(caller, index))
    }

    /// The commitment preimage, given the two component hashes.
    ///
    /// Split out so `anchor` hashes each component exactly once while still
    /// producing the same value `commitment` produces.
    fn commit_parts(
        env: &Env,
        target: Address,
        fn_name: Symbol,
        args_hash: BytesN<32>,
        result_hash: BytesN<32>,
        ledger: u32,
    ) -> BytesN<32> {
        let mut preimage = Bytes::from_slice(env, DOMAIN);
        preimage.append(&target.to_xdr(env));
        preimage.append(&fn_name.to_xdr(env));
        // Fixed-width 32-byte components, so no two different tuples can
        // concatenate to the same preimage by shifting a boundary.
        preimage.append(&Bytes::from_array(env, &args_hash.to_array()));
        preimage.append(&Bytes::from_array(env, &result_hash.to_array()));
        preimage.append(&ledger.to_xdr(env));

        env.crypto().sha256(&preimage).into()
    }

    /// SHA-256 of a value's canonical XDR encoding.
    fn hash_xdr(env: &Env, value: impl ToXdr) -> BytesN<32> {
        let xdr = value.to_xdr(env);
        env.crypto().sha256(&xdr).into()
    }
}

mod test;
