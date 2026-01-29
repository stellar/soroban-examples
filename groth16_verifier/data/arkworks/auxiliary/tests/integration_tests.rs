use ark_bls12_381::{Bls12_381, Fr};
use ark_crypto_primitives::crh::{CRHScheme, TwoToOneCRHScheme};
use ark_groth16::Groth16;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem};
use ark_snark::SNARK;
use ark_snarkjs::{export_proof, export_vk};
use ark_std::rand::rngs::OsRng;
use merkle::circuit::MerkleTreeVerification;
use merkle::common::{LeafHash, TwoToOneHash};
use merkle::*;
use std::fs;

// cargo test merkle_tree -- --nocapture

#[test]
fn merkle_tree() {
    let mut rng = OsRng;

    // Sample public parameters for the hash functions.
    let leaf_crh_params = <LeafHash as CRHScheme>::setup(&mut rng).unwrap();
    let two_to_one_crh_params = <TwoToOneHash as TwoToOneCRHScheme>::setup(&mut rng).unwrap();

    // Construct the tree.
    let tree = SimpleMerkleTree::new(
        &leaf_crh_params,
        &two_to_one_crh_params,
        &[
            &[1u8],
            &[2u8],
            &[3u8],
            &[10u8],
            &[9u8],
            &[17u8],
            &[70u8],
            &[45u8],
        ],
    )
    .unwrap();

    // Generate membership proof for leaf == 9 (index 4).
    let proof = tree.generate_proof(4).unwrap();
    let root = tree.root();

    // Create circuit with dummy proof for parameter generation
    // We use the same proof but with a dummy leaf value
    let dummy_circuit = MerkleTreeVerification {
        leaf_crh_params: Some(leaf_crh_params.clone()),
        two_to_one_crh_params: Some(two_to_one_crh_params.clone()),
        root,
        leaf: 9u8,                                // Use the actual leaf value
        authentication_path: Some(proof.clone()), // Use a clone of the proof
    };

    // Generate Groth16 parameters
    let groth_params =
        Groth16::<Bls12_381>::generate_random_parameters_with_reduction(dummy_circuit, &mut rng)
            .unwrap();

    // Create circuit with witness for actual proof generation
    let circuit = MerkleTreeVerification {
        leaf_crh_params: Some(leaf_crh_params.clone()),
        two_to_one_crh_params: Some(two_to_one_crh_params.clone()),
        root,
        leaf: 9u8,
        authentication_path: Some(proof.clone()),
    };

    // Get public inputs from the constraint system
    // We need to build a constraint system to extract the actual public inputs
    let cs_for_inputs = ConstraintSystem::new_ref();
    let circuit_for_inputs = MerkleTreeVerification {
        leaf_crh_params: Some(leaf_crh_params.clone()),
        two_to_one_crh_params: Some(two_to_one_crh_params.clone()),
        root,
        leaf: 9u8,
        authentication_path: Some(proof.clone()),
    };
    circuit_for_inputs
        .generate_constraints(cs_for_inputs.clone())
        .unwrap();

    // Extract public inputs from the constraint system
    // instance_assignment[0] is always 1 (the constant one), so we skip it
    let public_inputs: Vec<Fr> = cs_for_inputs.borrow().unwrap().instance_assignment[1..].to_vec();

    eprintln!("Total public inputs count: {}", public_inputs.len());
    eprintln!("Public inputs: {:?}", public_inputs);

    // Generate proof using the original circuit
    let groth_proof = Groth16::<Bls12_381>::prove(&groth_params, circuit, &mut rng).unwrap();

    eprintln!("Groth proof: {:?}", groth_proof);
    eprintln!("Groth parameters: {:?}", groth_params.vk);
    eprintln!("Public inputs: {:?}", public_inputs);

    // Verify proof before export
    let verified =
        Groth16::<Bls12_381>::verify(&groth_params.vk, &public_inputs, &groth_proof).unwrap();
    assert!(verified, "Proof verification failed");

    // Export to JSON
    export_proof::<Bls12_381, _>(&groth_proof, &public_inputs, "../proof.json").unwrap();

    // Use the number of public inputs (verified to match VK)
    export_vk::<Bls12_381, _>(
        &groth_params.vk,
        public_inputs.len(),
        "../verification_key.json",
    )
    .unwrap();
}
