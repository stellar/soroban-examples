pragma circom 2.2.0;

include "../merkleProof.circom";

/**
 * @title TestMerkleProof template
 * @dev Simple test circuit that instantiates the MerkleProof template
 * @notice This is used to test compatibility between lean-imt and merkleProof.circom
 * @param depth The depth of the Merkle tree (set to 2 for testing)
 */
template TestMerkleProof(depth) {
    // inputs 
    signal input leaf;             // leaf value to prove inclusion of (256-bit array)
    signal input leafIndex;        // index of leaf in the Merkle tree
    signal input siblings[depth];  // sibling values along the path to the root (256-bit arrays)

    // outputs
    signal output out;             // field element
    
    // Instantiate the MerkleProof template
    component merkleProof = MerkleProof(depth);
    merkleProof.leaf <== leaf;
    merkleProof.leafIndex <== leafIndex;
    merkleProof.siblings <== siblings;
    
    // Output the computed root
    out <== merkleProof.out;
}

// Main component for testing
component main {public [leaf, leafIndex, siblings]} = TestMerkleProof(2);
