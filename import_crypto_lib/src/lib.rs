#![no_std]
use ark_bn254::{Bn254, Fq12, G1Affine, G2Affine};
use ark_ec::pairing::Pairing;
use ark_ff::Field;
use ark_serialize::CanonicalDeserialize;
use soroban_sdk::{contract, contractimpl, contracttype, BytesN, Env};

extern crate alloc;

#[derive(Clone)]
#[contracttype]
pub struct DummyProof {
    pub g1: BytesN<64>,
    pub g2: BytesN<128>,
}

#[contract]
pub struct Bn254Contract;

#[contractimpl]
impl Bn254Contract {
    // A dummy proof that just performs pairing on the two input points
    pub fn dummy_verify(_env: Env, proof: DummyProof) -> bool {
        let mut g1_slice = [0; 64];
        proof.g1.copy_into_slice(&mut g1_slice);
        let g1 = G1Affine::deserialize_uncompressed(&g1_slice[..]).unwrap();
        let mut g2_slice = [0; 128];
        proof.g2.copy_into_slice(&mut g2_slice);
        let g2 = G2Affine::deserialize_uncompressed(&g2_slice[..]).unwrap();
        let res = Bn254::pairing(g1, g2);
        res.0.cmp(&Fq12::ONE) == core::cmp::Ordering::Equal
    }
}

mod test;
