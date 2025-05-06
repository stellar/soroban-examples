#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    crypto::bls12_381::{Fr, G1Affine, G2Affine},
    vec, Env, Vec,
};

#[derive(Clone)]
#[contracttype]
pub struct VerificationKey {
    pub alpha: G1Affine,
    pub beta: G2Affine,
    pub gamma: G2Affine,
    pub delta: G2Affine,
    pub ic: Vec<G1Affine>,
}

#[derive(Clone)]
#[contracttype]
pub struct Proof {
    pub a: G1Affine,
    pub b: G2Affine,
    pub c: G1Affine,
}

#[contract]
pub struct Groth16Verifier;

#[contractimpl]
impl Groth16Verifier {
    pub fn verify_proof(env: Env, vk: VerificationKey, proof: Proof, pub_signals: Vec<Fr>) -> bool {
        let bls = env.crypto().bls12_381();

        // vk_x = IC0 + pub_signals[0]*IC1
        // Here we have only one public input. If there were more, we'd sum them similarly.
        let mut vk_x = vk.ic.get(0).unwrap();
        if let Some(s) = pub_signals.get(0) {
            let term = bls.g1_mul(&vk.ic.get(1).unwrap(), &s);
            vk_x = bls.g1_add(&vk_x, &term);
        }

        // We need to compute the pairing:
        // e(-A, B) * e(alpha, beta) * e(vk_x, gamma) * e(C, delta) == 1
        let neg_a = -proof.a;
        let vp1 = vec![&env, neg_a, vk.alpha, vk_x, proof.c];
        let vp2 = vec![&env, proof.b, vk.beta, vk.gamma, vk.delta];

        bls.pairing_check(vp1, vp2)
    }
}

mod test;
