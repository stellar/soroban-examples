#![cfg(test)]

extern crate std;

use ark_bn254::{Fq, Fq2};
use ark_ff::{BigInteger, PrimeField};
use core::str::FromStr;
use serde::Deserialize;
use soroban_sdk::{
    Env, U256, Vec,
    crypto::bn254::{
        BN254_G1_SERIALIZED_SIZE, BN254_G2_SERIALIZED_SIZE, Bn254G1Affine, Bn254G2Affine, Fr,
    },
};

use crate::{Groth16Verifier, Groth16VerifierClient, Proof};

#[derive(Deserialize)]
struct ProofJson {
    pi_a: [std::string::String; 3],
    pi_b: [[std::string::String; 2]; 3],
    pi_c: [std::string::String; 3],
    #[serde(rename = "publicSignals")]
    public_signals: std::vec::Vec<std::string::String>,
}

fn fq_to_bytes_be(fq: &Fq) -> [u8; 32] {
    let bytes = fq.into_bigint().to_bytes_be();
    let mut out = [0u8; 32];
    let start = out.len().saturating_sub(bytes.len());
    out[start..].copy_from_slice(&bytes);
    out
}

fn g1_from_coords(env: &Env, x: &str, y: &str) -> Bn254G1Affine {
    let ark_g1 = ark_bn254::G1Affine::new(Fq::from_str(x).unwrap(), Fq::from_str(y).unwrap());
    let mut buf = [0u8; BN254_G1_SERIALIZED_SIZE];
    buf[..32].copy_from_slice(&fq_to_bytes_be(&ark_g1.x));
    buf[32..].copy_from_slice(&fq_to_bytes_be(&ark_g1.y));
    Bn254G1Affine::from_array(env, &buf)
}

fn g2_from_coords(env: &Env, x1: &str, x2: &str, y1: &str, y2: &str) -> Bn254G2Affine {
    let x = Fq2::new(Fq::from_str(x1).unwrap(), Fq::from_str(x2).unwrap());
    let y = Fq2::new(Fq::from_str(y1).unwrap(), Fq::from_str(y2).unwrap());
    let ark_g2 = ark_bn254::G2Affine::new(x, y);
    let mut buf = [0u8; BN254_G2_SERIALIZED_SIZE];
    buf[0..32].copy_from_slice(&fq_to_bytes_be(&ark_g2.x.c1));
    buf[32..64].copy_from_slice(&fq_to_bytes_be(&ark_g2.x.c0));
    buf[64..96].copy_from_slice(&fq_to_bytes_be(&ark_g2.y.c1));
    buf[96..128].copy_from_slice(&fq_to_bytes_be(&ark_g2.y.c0));
    Bn254G2Affine::from_array(env, &buf)
}

fn create_client(e: &Env) -> Groth16VerifierClient<'_> {
    let contract_id = e.register(Groth16Verifier {}, ());
    Groth16VerifierClient::new(e, &contract_id)
}

#[test]
fn test() {
    let env = Env::default();

    let proof_json_str = include_str!("../../../data/gnark_bn254/proof.json");
    let proof_json: ProofJson = serde_json::from_str(proof_json_str).unwrap();

    let proof = Proof {
        a: g1_from_coords(&env, &proof_json.pi_a[0], &proof_json.pi_a[1]),
        b: g2_from_coords(
            &env,
            &proof_json.pi_b[0][0],
            &proof_json.pi_b[0][1],
            &proof_json.pi_b[1][0],
            &proof_json.pi_b[1][1],
        ),
        c: g1_from_coords(&env, &proof_json.pi_c[0], &proof_json.pi_c[1]),
    };

    let client = create_client(&env);

    let mut output = Vec::new(&env);
    for s in &proof_json.public_signals {
        let val: u32 = s.parse().unwrap();
        output.push_back(Fr::from_u256(U256::from_u32(&env, val)));
    }
    let res = client.verify_proof(&proof, &output);
    assert_eq!(res, true);

    let output = Vec::from_array(&env, [Fr::from_u256(U256::from_u32(&env, 22))]);
    let res = client.verify_proof(&proof, &output);
    assert_eq!(res, false);
}
