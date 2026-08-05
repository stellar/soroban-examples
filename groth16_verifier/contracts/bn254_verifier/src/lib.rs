#![no_std]

use soroban_sdk::{
    Address, Bytes, BytesN, Env, U256, Vec, contract, contracterror, contractimpl, contracttype,
    crypto::bn254::{Bn254Fr, Bn254G1Affine, Bn254G2Affine},
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
    0x30, 0x64, 0x4e, 0x72, 0xe1, 0x31, 0xa0, 0x29, 0xb8, 0x50, 0x45, 0xb6, 0x81, 0x81, 0x58, 0x5d,
    0x28, 0x33, 0xe8, 0x48, 0x79, 0xb9, 0x70, 0x91, 0x43, 0xe1, 0xf5, 0x93, 0xf0, 0x00, 0x00, 0x01,
];

fn canonical_fr(env: &Env, bytes: BytesN<32>) -> Result<Bn254Fr, Groth16Error> {
    let value = U256::from_be_bytes(env, &Bytes::from_array(env, &bytes.to_array()));
    let modulus = U256::from_be_bytes(env, &Bytes::from_array(env, &FR_MODULUS_BE));
    if value >= modulus {
        return Err(Groth16Error::NonCanonicalPublicInput);
    }
    Ok(Bn254Fr::from_u256(value))
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
    pub alpha: Bn254G1Affine,
    pub beta: Bn254G2Affine,
    pub gamma: Bn254G2Affine,
    pub delta: Bn254G2Affine,
    pub ic: Vec<Bn254G1Affine>,
}

#[derive(Clone)]
#[contracttype]
pub struct Proof {
    pub a: Bn254G1Affine,
    pub b: Bn254G2Affine,
    pub c: Bn254G1Affine,
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

        let bn = env.crypto().bn254();
        let mut vk_x = vk.ic.get(0).unwrap();
        for (signal, point) in pub_signals.iter().zip(vk.ic.iter().skip(1)) {
            let term = bn.g1_mul(&point, &signal);
            vk_x = bn.g1_add(&vk_x, &term);
        }

        let neg_a = -proof.a;
        let lhs = soroban_sdk::vec![&env, neg_a, vk.alpha, vk_x, proof.c];
        let rhs = soroban_sdk::vec![&env, proof.b, vk.beta, vk.gamma, vk.delta];

        Ok(bn.pairing_check(lhs, rhs))
    }
}
