use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use ark_bn254::{Fq, Fq2, Fr as ArkFr};
use ark_ff::{BigInteger, PrimeField};
use bn254_verifier::{Groth16Verifier, Groth16VerifierClient, Proof, VerificationKey};
use serde::Deserialize;
use soroban_sdk::{
    Address, Bytes, Env, U256, Vec,
    crypto::bn254::{
        BN254_G1_SERIALIZED_SIZE, BN254_G2_SERIALIZED_SIZE, Bn254Fr, Bn254G1Affine, Bn254G2Affine,
    },
};

pub struct Fixture {
    pub verification_key: VerificationKey,
    pub proof: Proof,
    pub public_signals: Vec<Bn254Fr>,
}

#[derive(Deserialize)]
struct VerificationKeyJson {
    vk_alpha_1: [String; 3],
    vk_beta_2: [[String; 2]; 3],
    vk_gamma_2: [[String; 2]; 3],
    vk_delta_2: [[String; 2]; 3],
    #[serde(rename = "IC")]
    ic: std::vec::Vec<[String; 3]>,
}

#[derive(Deserialize)]
struct ProofJson {
    pi_a: [String; 3],
    pi_b: [[String; 2]; 3],
    pi_c: [String; 3],
    #[serde(rename = "publicSignals")]
    public_signals: std::vec::Vec<String>,
}

pub fn load_fixture(env: &Env) -> Fixture {
    let dir = fixture_dir();
    let proof_json: ProofJson = serde_json::from_str(&read_file(dir.join("proof.json"))).unwrap();
    let verification_key_json: VerificationKeyJson =
        serde_json::from_str(&read_file(dir.join("verification_key.json"))).unwrap();

    let mut signals = Vec::new(env);
    for signal in proof_json.public_signals {
        signals.push_back(fr_from_str(env, &signal));
    }

    let mut ic = Vec::new(env);
    for point in verification_key_json.ic {
        ic.push_back(g1_from_coords(env, &point[0], &point[1]));
    }

    Fixture {
        verification_key: VerificationKey {
            alpha: g1_from_coords(
                env,
                &verification_key_json.vk_alpha_1[0],
                &verification_key_json.vk_alpha_1[1],
            ),
            beta: g2_from_coords(
                env,
                &verification_key_json.vk_beta_2[0][0],
                &verification_key_json.vk_beta_2[0][1],
                &verification_key_json.vk_beta_2[1][0],
                &verification_key_json.vk_beta_2[1][1],
            ),
            gamma: g2_from_coords(
                env,
                &verification_key_json.vk_gamma_2[0][0],
                &verification_key_json.vk_gamma_2[0][1],
                &verification_key_json.vk_gamma_2[1][0],
                &verification_key_json.vk_gamma_2[1][1],
            ),
            delta: g2_from_coords(
                env,
                &verification_key_json.vk_delta_2[0][0],
                &verification_key_json.vk_delta_2[0][1],
                &verification_key_json.vk_delta_2[1][0],
                &verification_key_json.vk_delta_2[1][1],
            ),
            ic,
        },
        proof: Proof {
            a: g1_from_coords(env, &proof_json.pi_a[0], &proof_json.pi_a[1]),
            b: g2_from_coords(
                env,
                &proof_json.pi_b[0][0],
                &proof_json.pi_b[0][1],
                &proof_json.pi_b[1][0],
                &proof_json.pi_b[1][1],
            ),
            c: g1_from_coords(env, &proof_json.pi_c[0], &proof_json.pi_c[1]),
        },
        public_signals: signals,
    }
}

pub fn deploy<'a>(
    env: &Env,
    admin: &Address,
    verification_key: &VerificationKey,
) -> Groth16VerifierClient<'a> {
    let contract_id = env.register(Groth16Verifier, (admin, verification_key));
    Groth16VerifierClient::new(env, &contract_id)
}

pub fn replace_first_signal(env: &Env, signals: &Vec<Bn254Fr>, replacement: &str) -> Vec<Bn254Fr> {
    let mut updated = Vec::new(env);
    if signals.is_empty() {
        return updated;
    }

    updated.push_back(fr_from_str(env, replacement));
    for signal in signals.iter().skip(1) {
        updated.push_back(signal);
    }
    updated
}

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("gnark")
}

fn read_file(path: PathBuf) -> String {
    fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
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

fn fr_from_str(env: &Env, s: &str) -> Bn254Fr {
    let ark_fr = ArkFr::from_str(s).unwrap();
    let bigint = ark_fr.into_bigint();
    let bytes = bigint.to_bytes_le();
    let mut u256_bytes = [0u8; 32];
    let copy_len = bytes.len().min(32);
    u256_bytes[..copy_len].copy_from_slice(&bytes[..copy_len]);
    u256_bytes.reverse();
    let bytes_obj = Bytes::from_array(env, &u256_bytes);
    Bn254Fr::from_u256(U256::from_be_bytes(env, &bytes_obj))
}
