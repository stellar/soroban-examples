#![cfg(test)]

extern crate std;

use ark_bls12_381::{Fq, Fq2, Fr as ArkFr};
use ark_ff::{BigInteger, PrimeField};
use ark_serialize::CanonicalSerialize;
use core::str::FromStr;
use serde::Deserialize;
use soroban_sdk::{
    Bytes, Env, U256, Vec,
    crypto::bls12_381::{Fr, G1_SERIALIZED_SIZE, G1Affine, G2_SERIALIZED_SIZE, G2Affine},
};
use std::vec::Vec as AllocVec;

use crate::{Groth16Verifier, Groth16VerifierClient, Proof};

#[derive(Deserialize)]
struct ProofJson {
    pi_a: [std::string::String; 3],
    pi_b: [[std::string::String; 2]; 3],
    pi_c: [std::string::String; 3],
    #[serde(rename = "publicSignals")]
    public_signals: AllocVec<std::string::String>,
}

fn g1_from_coords(env: &Env, x: &str, y: &str) -> G1Affine {
    let ark_g1 = ark_bls12_381::G1Affine::new(Fq::from_str(x).unwrap(), Fq::from_str(y).unwrap());
    let mut buf = [0u8; G1_SERIALIZED_SIZE];
    ark_g1.serialize_uncompressed(&mut buf[..]).unwrap();
    G1Affine::from_array(env, &buf)
}

fn g2_from_coords(env: &Env, x1: &str, x2: &str, y1: &str, y2: &str) -> G2Affine {
    let x = Fq2::new(Fq::from_str(x1).unwrap(), Fq::from_str(x2).unwrap());
    let y = Fq2::new(Fq::from_str(y1).unwrap(), Fq::from_str(y2).unwrap());
    let ark_g2 = ark_bls12_381::G2Affine::new(x, y);
    let mut buf = [0u8; G2_SERIALIZED_SIZE];
    ark_g2.serialize_uncompressed(&mut buf[..]).unwrap();
    G2Affine::from_array(env, &buf)
}

fn create_client(e: &Env) -> Groth16VerifierClient<'_> {
    let contract_id = e.register(Groth16Verifier {}, ());
    Groth16VerifierClient::new(e, &contract_id)
}

fn fr_from_str(env: &Env, s: &str) -> Fr {
    // Parse string -> arkworks Fr -> bytes -> Soroban Fr
    let ark_fr = ArkFr::from_str(s).unwrap();
    let bigint = ark_fr.into_bigint();
    let bytes = bigint.to_bytes_le();
    let mut u256_bytes = [0u8; 32];
    u256_bytes[..bytes.len().min(32)].copy_from_slice(&bytes[..bytes.len().min(32)]);
    u256_bytes.reverse(); // little-endian -> big-endian for U256
    let bytes_obj = Bytes::from_array(&env, &u256_bytes);
    Fr::from_u256(U256::from_be_bytes(&env, &bytes_obj))
}

#[test]
fn test() {
    // Initialize the test environment
    let env = Env::default();

    // Load proof from JSON file
    let proof_json_str = include_str!("../../../data/arkworks/proof.json");
    let proof_json: ProofJson = serde_json::from_str(proof_json_str).unwrap();

    // Extract proof components from JSON
    let pi_ax = &proof_json.pi_a[0];
    let pi_ay = &proof_json.pi_a[1];
    let pi_bx1 = &proof_json.pi_b[0][0];
    let pi_bx2 = &proof_json.pi_b[0][1];
    let pi_by1 = &proof_json.pi_b[1][0];
    let pi_by2 = &proof_json.pi_b[1][1];
    let pi_cx = &proof_json.pi_c[0];
    let pi_cy = &proof_json.pi_c[1];

    // Construct the proof from JSON data
    let proof = Proof {
        a: g1_from_coords(&env, pi_ax, pi_ay),
        b: g2_from_coords(&env, pi_bx1, pi_bx2, pi_by1, pi_by2),
        c: g1_from_coords(&env, pi_cx, pi_cy),
    };

    // Create the contract client
    let client = create_client(&env);

    // Test Case 1: Verify the proof with all correct public signals from JSON
    let mut public_signals = Vec::new(&env);
    for signal_str in &proof_json.public_signals {
        let fr = fr_from_str(&env, signal_str);
        public_signals.push_back(fr);
    }
    let res = client.verify_proof(&proof, &public_signals);
    assert_eq!(res, true);

    // Test Case 2: Verify the proof with incorrect public signals (change first signal)
    let mut incorrect_signals = Vec::new(&env);
    // Use incorrect value for first signal, keep others the same
    incorrect_signals.push_back(fr_from_str(
        &env,
        "999999999999999999999999999999999999999999999999999999999999999999999",
    ));
    for signal_str in proof_json.public_signals.iter().skip(1) {
        let fr = fr_from_str(&env, signal_str);
        incorrect_signals.push_back(fr);
    }
    let res = client.verify_proof(&proof, &incorrect_signals);
    assert_eq!(res, false);
}
