use lean_imt::LeanIMT;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use soroban_sdk::{crypto::bls12_381::Fr as BlsScalar, Env};

/// Converts a BlsScalar to a decimal string representation
fn bls_scalar_to_decimal(scalar: BlsScalar) -> String {
    let bytes = scalar.to_bytes();
    let mut bytes_array = [0u8; 32];
    bytes.copy_into_slice(&mut bytes_array);
    let biguint = BigUint::from_bytes_be(&bytes_array);
    biguint.to_string()
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct MerkleProofResult {
    leaf: String,
    leafIndex: u32,
    siblings: Vec<String>,
    root: String,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() >= 6 {
        // Proof mode - compute merkle proof for specific leaf
        let mut leaves = Vec::new();
        for i in 1..5 {
            let leaf_value = args[i].parse().unwrap_or(0);
            leaves.push(leaf_value);
        }

        let leaf_index: u32 = args[5].parse().unwrap_or(0);

        println!("🧪 Computing Merkle Proof for Leaf Index {}", leaf_index);
        println!("================================================");
        println!("Testing merkle proof generation with lean-imt");
        println!("");

        let env = Env::default();
        env.cost_estimate().budget().reset_unlimited();
        let proof_result = compute_merkle_proof(&env, &leaves, leaf_index);

        println!("Leaf index: {}", proof_result.leafIndex);
        println!("Leaf value: {}", leaves[leaf_index as usize]);
        // Convert siblings to decimal for display
        let siblings_decimal_display: Vec<String> = proof_result
            .siblings
            .iter()
            .enumerate()
            .map(|(i, sibling_decimal)| {
                if i == 0 {
                    // First sibling is the other leaf at the same level
                    let sibling_leaf_index = if leaf_index < 2 {
                        if leaf_index == 0 {
                            1
                        } else {
                            0
                        }
                    } else {
                        if leaf_index == 2 {
                            3
                        } else {
                            2
                        }
                    };
                    leaves[sibling_leaf_index].to_string()
                } else {
                    // Second sibling is already a decimal string
                    sibling_decimal.to_string()
                }
            })
            .collect();
        println!("Siblings: {:?}", siblings_decimal_display);
        println!("Merkle root: {}", proof_result.root);

        // Save circuit-compatible input with decimal string representations
        let leaf_decimal = leaves[leaf_index as usize].to_string();

        // The siblings are already in decimal format, so we can use them directly
        let siblings_decimal = proof_result.siblings.clone();

        let circuit_input = CircuitInput {
            leaf: leaf_decimal,
            leafIndex: proof_result.leafIndex,
            siblings: siblings_decimal,
        };
        let circuit_json = serde_json::to_string_pretty(&circuit_input).unwrap();
        std::fs::write("circuit_input.json", circuit_json).unwrap();
        println!("📁 Circuit input saved to: circuit_input.json");

        return;
    }

    // Show usage if no valid arguments provided
    println!("🧪 Lean-IMT Test Suite");
    println!("======================");
    println!("Usage:");
    println!("   cargo run -- <leaf1> <leaf2> <leaf3> <leaf4> <leaf_index>");
    println!("\nExample:");
    println!("   cargo run -- 0 0 0 0 0");
}

fn compute_merkle_proof(env: &Env, leaves: &[u64], leaf_index: u32) -> MerkleProofResult {
    // Create a new LeanIMT instance
    let mut tree = LeanIMT::new(env, 2);

    for &leaf in leaves {
        tree.insert_u64(leaf).unwrap();
    }

    // Generate the merkle proof for the specified leaf index
    let proof = tree
        .generate_proof(leaf_index)
        .expect("Failed to generate proof");
    let (siblings, depth) = proof;

    // Get the leaf value using the new scalar-based method
    let leaf_scalar = tree
        .get_leaf_scalar(leaf_index as usize)
        .expect("Leaf not found");
    let leaf_value_decimal = bls_scalar_to_decimal(leaf_scalar);

    // Convert siblings to decimal strings - exactly `depth` items (no root included)
    let mut siblings_decimal = Vec::new();
    for i in 0..(depth as usize) {
        if i == 0 {
            // First sibling is the other leaf at the same level
            let sibling_leaf_index = if leaf_index < 2 {
                if leaf_index == 0 {
                    1
                } else {
                    0
                }
            } else {
                if leaf_index == 2 {
                    3
                } else {
                    2
                }
            };
            siblings_decimal.push(leaves[sibling_leaf_index].to_string());
        } else {
            let sibling = siblings
                .get(i as u32)
                .expect("Missing sibling in proof for required depth");
            let decimal_value = bls_scalar_to_decimal(sibling);
            siblings_decimal.push(decimal_value);
        }
    }

    // Get the merkle root for display
    let root_scalar = tree.get_root_scalar();
    let root_decimal = bls_scalar_to_decimal(root_scalar);

    MerkleProofResult {
        leaf: leaf_value_decimal,
        leafIndex: leaf_index,
        siblings: siblings_decimal,
        root: root_decimal,
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct CircuitInput {
    leaf: String,
    leafIndex: u32,
    siblings: Vec<String>,
}
