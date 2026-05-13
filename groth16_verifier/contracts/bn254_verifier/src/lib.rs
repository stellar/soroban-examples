#![no_std]

use soroban_sdk::{
    Address, Env, Vec, contract, contracterror, contractimpl, contracttype,
    crypto::bn254::{Bn254Fr, Bn254G1Affine, Bn254G2Affine},
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Groth16Error {
    MalformedVerifyingKey = 0,
    VerificationKeyNotSet = 1,
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
        pub_signals: Vec<Bn254Fr>,
    ) -> Result<bool, Groth16Error> {
        let vk: VerificationKey = env
            .storage()
            .instance()
            .get(&DataKey::VerificationKey)
            .ok_or(Groth16Error::VerificationKeyNotSet)?;

        if pub_signals.len() + 1 != vk.ic.len() {
            return Err(Groth16Error::MalformedVerifyingKey);
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
