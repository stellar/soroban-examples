//! This is a basic custom account contract that implements the
//! `FastAggregateVerify` function in [BLS
//! Signatures](https://www.ietf.org/archive/id/draft-irtf-cfrg-bls-signature-05.html#name-fastaggregateverify)
//!
//! ⚠️ WARNING: it is indended for demonstration purpose only. It is not
//! security-audited and not safe to use in production (e.g. there is no proof
//! of possesion for the public key described in section 3.3).
#![no_std]
use soroban_sdk::{
    auth::{Context, CustomAccountInterface},
    bytesn, contract, contracterror, contractimpl, contracttype,
    crypto::{
        bls12_381::{G1Affine, G2Affine},
        Hash,
    },
    vec, Bytes, BytesN, Env, Vec,
};

#[contract]
pub struct IncrementContract;

// `DST `is the domain separation tag, intended to keep hashing inputs of your
// contract separate. Refer to section 3.1 in the [Hashing to Elliptic
// Curves](https://datatracker.ietf.org/doc/html/rfc9380) on requirements of
// DST.
const DST: &str = "BLSSIG-V01-CS01-with-BLS12381G2_XMD:SHA-256_SSWU_RO_";

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Owners,
    Counter,
    Dst,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AccError {
    InvalidSignature = 1,
}

#[contractimpl]
impl IncrementContract {
    pub fn init(env: Env, agg_pk: BytesN<96>) {
        // Initialize the account contract essentials: the aggregated pubkey and
        // the DST. Because the message to be signed (which is
        // the hash of some call stack) is the same for all signers, we can
        // simply aggregate all signers (adding up the G1 pubkeys) and store it.
        env.storage().persistent().set(&DataKey::Owners, &agg_pk);
        env.storage()
            .instance()
            .set(&DataKey::Dst, &Bytes::from_slice(&env, DST.as_bytes()));
        // initialize the counter, i.e. the business logic this signer contract
        // guards
        env.storage().instance().set(&DataKey::Counter, &0_u32);
    }

    pub fn increment(env: Env) -> u32 {
        env.current_contract_address().require_auth();
        let mut count: u32 = env.storage().instance().get(&DataKey::Counter).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&DataKey::Counter, &count);
        count
    }
}

#[contractimpl]
impl CustomAccountInterface for IncrementContract {
    type Signature = BytesN<192>;

    type Error = AccError;

    #[allow(non_snake_case)]
    fn __check_auth(
        env: Env,
        signature_payload: Hash<32>,
        agg_sig: Self::Signature,
        _auth_contexts: Vec<Context>,
    ) -> Result<(), AccError> {
        // The sdk module containing access to the bls12_381 functions
        let bls = env.crypto().bls12_381();

        // Retrieve the aggregated pubkey and the DST from storage
        let agg_pk: BytesN<96> = env.storage().persistent().get(&DataKey::Owners).unwrap();
        let dst: Bytes = env.storage().instance().get(&DataKey::Dst).unwrap();

        // This is the negative of g1 (generator point of the G1 group)
        let neg_g1 = G1Affine::from_bytes(bytesn!(&env, 0x17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb114d1d6855d545a8aa7d76c8cf2e21f267816aef1db507c96655b9d5caac42364e6f38ba0ecb751bad54dcd6b939c2ca));
        // Hash the signature_payload i.e. the msg being signed and to be
        // verified into a point in G2
        let msg_g2 = bls.hash_to_g2(&signature_payload.into(), &dst);

        // Prepare inputs to the pairing function
        let vp1 = vec![&env, G1Affine::from_bytes(agg_pk), neg_g1];
        let vp2 = vec![&env, msg_g2, G2Affine::from_bytes(agg_sig)];

        // Perform the pairing check, i.e. e(pk, msg)*e(-g1, sig) == 1, which is
        // equivalent to checking `e(pk, msg) == e(g1, sig)`.
        // The LHS = e(sk * g1, msg) = sk * e(g1, msg) = e(g1, sk * msg) = e(g1, sig),
        // thus it must equal to the RHS if the signature matches.
        if !bls.pairing_check(vp1, vp2) {
            return Err(AccError::InvalidSignature);
        }
        Ok(())
    }
}

mod test;
