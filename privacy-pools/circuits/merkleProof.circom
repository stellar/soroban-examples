pragma circom 2.2.0;

include "poseidon255.circom";
include "comparators.circom";
include "mux1.circom";

/**
 * @title MerkleProof template
 * @dev Template for generating and verifying inclusion proofs in a Lean Incremental Merkle Tree
 * @notice This circuit follows the LeanIMT design where:
 *   1. Every node with two children is the hash of its left and right nodes
 *   2. Every node with one child has the same value as its child node
 *   3. Tree is always built from leaves to root
 *   4. Tree is always balanced by construction
 *   5. Tree depth is dynamic and can increase with insertion of new leaves
 * @param depth The depth of the Merkle tree
 */
template MerkleProof(depth) {
    // inputs 
    signal input leaf;                  // leaf value to prove inclusion of
    signal input leafIndex;             // index of leaf in the Merkle tree
    signal input siblings[depth];       // sibling values along the path to the root

    // outputs
    signal output out;
    
    // internal signals
    signal nodes[depth + 1]; // stores computed node values at each level
    signal indices[depth];   // stores path indices for each level

    // components
    component hashInCorrectOrder[depth]; // orders node pairs for hashing
    component hashes[depth]; // Hash components (can use any hash function)

    // implmenentation
    component indexToPath = Num2Bits(depth);
    indexToPath.in <== leafIndex;
    indices <== indexToPath.out;

    // Init leaf with value
    nodes[0] <== leaf;

    for (var i = 0; i < depth; i++) {
        // prepare pairs for both possible orderings
        var childrenToSort[2][2] = [ [nodes[i], siblings[i]], [siblings[i], nodes[i]] ];
        hashInCorrectOrder[i] = MultiMux1(2);
        hashInCorrectOrder[i].c <== childrenToSort;
        hashInCorrectOrder[i].s <== indices[i];
        
        // hash the nodes using the specified hash function
        hashes[i] = Poseidon255(2);
        hashes[i].in <== hashInCorrectOrder[i].out;

        nodes[i + 1] <== hashes[i].out;
    }

    out <== nodes[depth];
}
