use base64::engine::Engine;
use base64::{self, engine::general_purpose};
use clap::Parser;
use num_bigint::BigUint;
use num_traits::Num;
use serde::Deserialize;
use std::fs;

// imports related to constructing VK, Proof and Public Signals
use ark_bls12_381::{Fq, Fq2};
use ark_serialize::CanonicalSerialize;
use core::str::FromStr;
use soroban_sdk::crypto::bls12_381::Fr;
use soroban_sdk::crypto::bls12_381::{G1Affine, G2Affine, G1_SERIALIZED_SIZE, G2_SERIALIZED_SIZE};
use soroban_sdk::U256;
use soroban_sdk::{Bytes, Env, Vec};
use zk::{Proof, PublicSignals, VerificationKey};

#[derive(Parser)]
struct Args {
    filetype: String,
    filename: String,
}

#[derive(Deserialize)]
struct VerificationKeyJson {
    vk_alpha_1: [String; 3],
    vk_beta_2: [[String; 2]; 3],
    vk_gamma_2: [[String; 2]; 3],
    vk_delta_2: [[String; 2]; 3],
    #[serde(rename = "IC")]
    ic: std::vec::Vec<[String; 3]>,
    #[serde(rename = "nPublic")]
    n_public: u32,
}

#[derive(Deserialize)]
struct ProofJson {
    pi_a: [String; 3],
    pi_b: [[String; 2]; 3],
    pi_c: [String; 3],
    #[serde(rename = "protocol")]
    _protocol: String,
    #[serde(rename = "curve")]
    _curve: String,
}

// Remove the old PublicOutputJson struct and replace with type alias
type PublicOutputJson = std::vec::Vec<String>;

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

fn validate_vk(vk: &VerificationKeyJson) {
    let expected_ic_size = (vk.n_public + 1) as usize;
    if vk.ic.len() != expected_ic_size {
        panic!("Invalid verification key: IC array has {} elements but nPublic={} requires {} elements", 
               vk.ic.len(), vk.n_public, expected_ic_size);
    }
}

fn print_vk(json_str: &String) {
    let vk: VerificationKeyJson = serde_json::from_str(json_str).expect("Invalid JSON");

    // Validate the verification key structure
    validate_vk(&vk);

    println!("// CODE START");
    println!("let alphax = \"{}\";", vk.vk_alpha_1[0]);
    println!("let alphay = \"{}\";", vk.vk_alpha_1[1]);
    println!("\n");
    println!("let betax1 = \"{}\";", vk.vk_beta_2[0][0]);
    println!("let betax2 = \"{}\";", vk.vk_beta_2[0][1]);
    println!("let betay1 = \"{}\";", vk.vk_beta_2[1][0]);
    println!("let betay2 = \"{}\";", vk.vk_beta_2[1][1]);
    println!("\n");
    println!("let gammax1 = \"{}\";", vk.vk_gamma_2[0][0]);
    println!("let gammax2 = \"{}\";", vk.vk_gamma_2[0][1]);
    println!("let gammay1 = \"{}\";", vk.vk_gamma_2[1][0]);
    println!("let gammay2 = \"{}\";", vk.vk_gamma_2[1][1]);
    println!("\n");
    println!("let deltax1 = \"{}\";", vk.vk_delta_2[0][0]);
    println!("let deltax2 = \"{}\";", vk.vk_delta_2[0][1]);
    println!("let deltay1 = \"{}\";", vk.vk_delta_2[1][0]);
    println!("let deltay2 = \"{}\";", vk.vk_delta_2[1][1]);
    println!("\n");

    // Generate IC variables based on nPublic
    // The IC array has nPublic + 1 elements (first is generator point)
    for i in 0..=vk.n_public {
        println!("let ic{}x = \"{}\";", i, vk.ic[i as usize][0]);
        println!("let ic{}y = \"{}\";", i, vk.ic[i as usize][1]);
        println!("\n");
    }

    println!("// CODE END");
}

fn vk_to_bytes(json_str: &String) -> Bytes {
    let env = Env::default();

    let vk_json: VerificationKeyJson = serde_json::from_str(json_str).expect("Invalid JSON");

    // Validate the verification key structure
    validate_vk(&vk_json);

    let alphax = vk_json.vk_alpha_1[0].clone();
    let alphay = vk_json.vk_alpha_1[1].clone();
    let betax1 = vk_json.vk_beta_2[0][0].clone();
    let betax2 = vk_json.vk_beta_2[0][1].clone();
    let betay1 = vk_json.vk_beta_2[1][0].clone();
    let betay2 = vk_json.vk_beta_2[1][1].clone();
    let gammax1 = vk_json.vk_gamma_2[0][0].clone();
    let gammax2 = vk_json.vk_gamma_2[0][1].clone();
    let gammay1 = vk_json.vk_gamma_2[1][0].clone();
    let gammay2 = vk_json.vk_gamma_2[1][1].clone();
    let deltax1 = vk_json.vk_delta_2[0][0].clone();
    let deltax2 = vk_json.vk_delta_2[0][1].clone();
    let deltay1 = vk_json.vk_delta_2[1][0].clone();
    let deltay2 = vk_json.vk_delta_2[1][1].clone();

    // Build IC array dynamically based on nPublic
    let mut ic_array = Vec::new(&env);
    for i in 0..=vk_json.n_public {
        let icx = vk_json.ic[i as usize][0].clone();
        let icy = vk_json.ic[i as usize][1].clone();
        ic_array.push_back(g1_from_coords(&env, &icx, &icy));
    }

    let vk = VerificationKey {
        alpha: g1_from_coords(&env, &alphax, &alphay),
        beta: g2_from_coords(&env, &betax1, &betax2, &betay1, &betay2),
        gamma: g2_from_coords(&env, &gammax1, &gammax2, &gammay1, &gammay2),
        delta: g2_from_coords(&env, &deltax1, &deltax2, &deltay1, &deltay2),
        ic: ic_array,
    };

    return vk.to_bytes(&env);
}

fn proof_to_bytes(json_str: &String) -> Bytes {
    let env = Env::default();
    let proof_json: ProofJson = serde_json::from_str(json_str).expect("Invalid JSON");
    let pi_ax = proof_json.pi_a[0].clone();
    let pi_ay = proof_json.pi_a[1].clone();
    let pi_bx1 = proof_json.pi_b[0][0].clone();
    let pi_bx2 = proof_json.pi_b[0][1].clone();
    let pi_by1 = proof_json.pi_b[1][0].clone();
    let pi_by2 = proof_json.pi_b[1][1].clone();
    let pi_cx = proof_json.pi_c[0].clone();
    let pi_cy = proof_json.pi_c[1].clone();

    let proof = Proof {
        a: g1_from_coords(&env, &pi_ax, &pi_ay),
        b: g2_from_coords(&env, &pi_bx1, &pi_bx2, &pi_by1, &pi_by2),
        c: g1_from_coords(&env, &pi_cx, &pi_cy),
    };
    proof.to_bytes(&env)
}

fn print_proof(json_str: &String) {
    let proof: ProofJson = serde_json::from_str(json_str).expect("Invalid JSON");

    println!("// CODE START");
    println!("let pi_ax = \"{}\";", proof.pi_a[0]);
    println!("let pi_ay = \"{}\";", proof.pi_a[1]);
    println!("\n");
    println!("let pi_bx1 = \"{}\";", proof.pi_b[0][0]);
    println!("let pi_bx2 = \"{}\";", proof.pi_b[0][1]);
    println!("let pi_by1 = \"{}\";", proof.pi_b[1][0]);
    println!("let pi_by2 = \"{}\";", proof.pi_b[1][1]);
    println!("\n");
    println!("let pi_cx = \"{}\";", proof.pi_c[0]);
    println!("let pi_cy = \"{}\";", proof.pi_c[1]);
    println!("// CODE END");
}

fn print_public_output(json_str: &String) {
    let public_output: PublicOutputJson = serde_json::from_str(json_str).expect("Invalid JSON");

    println!("// CODE START");
    println!("// Public output signals:");
    for (i, signal) in public_output.iter().enumerate() {
        // Parse decimal string to BigUint
        let value = BigUint::from_str_radix(&signal, 10).unwrap();
        let mut bytes = value.to_bytes_be();
        // Pad to 32 bytes
        if bytes.len() < 32 {
            let mut padded = std::vec![0u8; 32 - bytes.len()];
            padded.extend_from_slice(&bytes);
            bytes = padded;
        }
        // Format as hex for Rust array
        let bytes_str = bytes
            .iter()
            .map(|b| format!("0x{:02x}", b))
            .collect::<std::vec::Vec<_>>()
            .join(", ");
        println!(
            "let public_{} = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[{}]));",
            i, bytes_str
        );
    }

    println!("\n// Create output vector for verification:");
    print!("let output = Vec::from_array(&env, [");
    for (i, _) in public_output.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("Fr::from_u256(public_{})", i);
    }
    println!("]);");
    println!("// CODE END");
}

fn public_output_to_bytes(json_str: &String) -> Bytes {
    let env = Env::default();
    let public_output: PublicOutputJson = serde_json::from_str(json_str).expect("Invalid JSON");
    let mut pub_signals = Vec::new(&env);
    for signal in public_output.iter() {
        let value = num_bigint::BigUint::from_str_radix(signal, 10).unwrap();
        let mut bytes = value.to_bytes_be();
        // Pad to 32 bytes
        if bytes.len() < 32 {
            let mut padded = std::vec![0u8; 32 - bytes.len()];
            padded.extend_from_slice(&bytes);
            bytes = padded;
        }
        let arr: [u8; 32] = bytes.try_into().expect("slice with incorrect length");
        let u256 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &arr));
        let fr = Fr::from_u256(u256);
        pub_signals.push_back(fr);
    }
    let public_signals = PublicSignals { pub_signals };
    public_signals.to_bytes(&env)
}

fn main() {
    let args = Args::parse();
    let json_str = fs::read_to_string(&args.filename).expect("Failed to read file");

    if args.filetype == "vk" {
        print_vk(&json_str);
        let vk_bytes = vk_to_bytes(&json_str);
        let vk_vec: std::vec::Vec<u8> = vk_bytes.iter().collect();
        let vk_base64 = general_purpose::STANDARD.encode(&vk_vec);
        let vk_hex = hex::encode(&vk_vec);
        println!("\nVK Base64 encoding:\n{}", vk_base64);
        println!("VK Hex encoding:\n{}", vk_hex);
    }

    if args.filetype == "proof" {
        print_proof(&json_str);
        let proof_bytes = proof_to_bytes(&json_str);
        let proof_vec: std::vec::Vec<u8> = proof_bytes.iter().collect();
        let proof_base64 = general_purpose::STANDARD.encode(&proof_vec);
        let proof_hex = hex::encode(&proof_vec);
        println!("\nProof Base64 encoding:\n{}", proof_base64);
        println!("Proof Hex encoding:\n{}", proof_hex);
    }

    if args.filetype == "public" {
        print_public_output(&json_str);
        let public_bytes = public_output_to_bytes(&json_str);
        let public_vec: std::vec::Vec<u8> = public_bytes.iter().collect();
        let public_base64 = general_purpose::STANDARD.encode(&public_vec);
        let public_hex = hex::encode(&public_vec);
        println!("\nPublic signals Base64 encoding:\n{}", public_base64);
        println!("Public signals Hex encoding:\n{}", public_hex);
    }
}
