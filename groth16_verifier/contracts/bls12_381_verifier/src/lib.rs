#![no_std]

use soroban_sdk::{
    Address, Bytes, BytesN, Env, U256, Vec, contract, contracterror, contractimpl, contracttype,
    crypto::bls12_381::{Bls12381Fr, Bls12381G1Affine, Bls12381G2Affine},
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Groth16Error {
    MalformedVerifyingKey = 0,
    VerificationKeyNotSet = 1,
    NonCanonicalPublicInput = 2,
}

const FR_MODULUS_BE: [u8; 32] = [
    0x73, 0xed, 0xa7, 0x53, 0x29, 0x9d, 0x7d, 0x48, 0x33, 0x39, 0xd8, 0x08, 0x09, 0xa1, 0xd8, 0x05,
    0x53, 0xbd, 0xa4, 0x02, 0xff, 0xfe, 0x5b, 0xfe, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x01,
];

fn canonical_fr(env: &Env, bytes: BytesN<32>) -> Result<Bls12381Fr, Groth16Error> {
    let value = U256::from_be_bytes(env, &Bytes::from_array(env, &bytes.to_array()));
    let modulus = U256::from_be_bytes(env, &Bytes::from_array(env, &FR_MODULUS_BE));
    if value >= modulus {
        return Err(Groth16Error::NonCanonicalPublicInput);
    }
    Ok(Bls12381Fr::from_u256(value))
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    VerificationKey,
}

#[derive(Clone)]
#[contracttype]
pub struct VerificationKey {
    pub alpha: Bls12381G1Affine,
    pub beta: Bls12381G2Affine,
    pub gamma: Bls12381G2Affine,
    pub delta: Bls12381G2Affine,
    pub ic: Vec<Bls12381G1Affine>,
}

#[derive(Clone)]
#[contracttype]
pub struct Proof {
    pub a: Bls12381G1Affine,
    pub b: Bls12381G2Affine,
    pub c: Bls12381G1Affine,
}

#[contract]
pub struct Groth16Verifier;

#[contractimpl]
impl Groth16Verifier {
    pub fn __constructor(env: Env, admin: Address, verification_key: VerificationKey) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::VerificationKey, &verification_key);
    }

    pub fn set_verification_key(env: Env, verification_key: VerificationKey) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage()
            .instance()
            .set(&DataKey::VerificationKey, &verification_key);
    }

    pub fn verify_proof(
        env: Env,
        proof: Proof,
        public_inputs: Vec<BytesN<32>>,
    ) -> Result<bool, Groth16Error> {
        let vk: VerificationKey = env
            .storage()
            .instance()
            .get(&DataKey::VerificationKey)
            .ok_or(Groth16Error::VerificationKeyNotSet)?;

        if public_inputs.len() + 1 != vk.ic.len() {
            return Err(Groth16Error::MalformedVerifyingKey);
        }

        let mut pub_signals = Vec::new(&env);
        for input in public_inputs.iter() {
            pub_signals.push_back(canonical_fr(&env, input)?);
        }

        let bls = env.crypto().bls12_381();
        let mut vk_x = vk.ic.get(0).unwrap();
        for (signal, point) in pub_signals.iter().zip(vk.ic.iter().skip(1)) {
            let term = bls.g1_mul(&point, &signal);
            vk_x = bls.g1_add(&vk_x, &term);
        }

        let neg_a = -proof.a;
        let lhs = soroban_sdk::vec![&env, neg_a, vk.alpha, vk_x, proof.c];
        let rhs = soroban_sdk::vec![&env, proof.b, vk.beta, vk.gamma, vk.delta];

        Ok(bls.pairing_check(lhs, rhs))
    }
}
