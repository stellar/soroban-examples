#![cfg(test)]

use ark_bls12_381::{Fq, Fq2};
use ark_serialize::CanonicalSerialize;
use core::str::FromStr;
use soroban_sdk::{
    Env, U256, Vec,
    crypto::bls12_381::{Fr, G1_SERIALIZED_SIZE, G1Affine, G2_SERIALIZED_SIZE, G2Affine},
};

use crate::{Groth16Verifier, Groth16VerifierClient, Proof};

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

#[test]
fn test() {
    // Initialize the test environment
    let env = Env::default();

    // Load proof components (copied from `data/proof.json`)
    let pi_ax = "314442236668110257304682488877371582255161413673331360366570443799415414639292047869143313601702131653514009114222";
    let pi_ay = "2384632327855835824635705027009217874826122107057894594162233214798350178691568018290025994699762298534539543934607";
    let pi_bx1 = "428844167033934720609657613212495751617651348480870890908850335525890280786532876634895457032623422366474694342656";
    let pi_bx2 = "3083139526360252775789959298805261067575555607578161553873977966165446991459924053189383038704105379290158793353905";
    let pi_by1 = "1590919422794657666432683000821892403620510405626533455397042191265963587891653562867091397248216891852168698286910";
    let pi_by2 = "3617931039814164588401589536353142503544155307022467123698224064329647390280346725086550997337076315487486714327146";
    let pi_cx = "3052934797502613468327963344215392478880720823583493172692775426011388142569325036386650708808320216973179639719187";
    let pi_cy = "2028185281516938724429867827057869371578022471499780916652824405212207527699373814371051328341613972789943854539597";

    // Construct the proof from the pre-computed components
    let proof = Proof {
        a: g1_from_coords(&env, &pi_ax, &pi_ay),
        b: g2_from_coords(&env, &pi_bx1, &pi_bx2, &pi_by1, &pi_by2),
        c: g1_from_coords(&env, &pi_cx, &pi_cy),
    };

    // Create the contract client
    let client = create_client(&env);

    // Test Case 1: Verify the proof with the correct public output (33, copied from `data/circom/public.json`)
    let output = Vec::from_array(&env, [Fr::from_u256(U256::from_u32(&env, 33))]);
    let res = client.verify_proof(&proof, &output);
    assert_eq!(res, true);

    // Test Case 2: Verify the proof with an incorrect public output (22)
    let output = Vec::from_array(&env, [Fr::from_u256(U256::from_u32(&env, 22))]);
    let res = client.verify_proof(&proof, &output);
    assert_eq!(res, false);
}
