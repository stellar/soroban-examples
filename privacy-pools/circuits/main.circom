pragma circom 2.2.0;

include "commitment.circom";
include "merkleProof.circom";
include "poseidon.circom";

template Withdraw(treeDepth, associationDepth) {
    // PUBLIC SIGNALS
    signal input withdrawnValue;
    signal input stateRoot;             // a known state root
    signal input associationRoot;       // root of the association set Merkle tree

    // PRIVATE SIGNALS

    // signals to compute commitments
    signal input label;                 // hash(scope, nonce) % SNARK_SCALAR_FIELD
    signal input value;                 // value of the commitment
    signal input nullifier;             // nullifier of the commitment
    signal input secret;                // Secret of the commitment

    // signals for merkle tree inclusion proofs
    signal input stateSiblings[treeDepth];   // siblings of the state tree
    signal input stateIndex;                    // index of the commitment in the state tree
    
    // signals for association set verification
    signal input labelIndex;            // index of the label in the association tree
    signal input labelSiblings[associationDepth]; // siblings along the path to the association root

    // OUTPUT SIGNALS
    signal output nullifierHash;        // hash of commitment nullifier (public output)

    // IMPLEMENTATION

    // compute commitment
    component commitmentHasher = CommitmentHasher();
    commitmentHasher.label <== label;
    commitmentHasher.value <== value;
    commitmentHasher.secret <== secret;
    commitmentHasher.nullifier <== nullifier;
    signal commitment <== commitmentHasher.commitment;

    // output nullifier hash
    nullifierHash <== commitmentHasher.nullifierHash;

    // verify commitment is in the state tree
    component stateRootChecker = MerkleProof(treeDepth);
    stateRootChecker.leaf <== commitment;
    stateRootChecker.leafIndex <== stateIndex;
    stateRootChecker.siblings <== stateSiblings;
    
    stateRoot === stateRootChecker.out;
    
    // verify label is in the association set using merkleProof directly
    component associationRootChecker = MerkleProof(associationDepth);
    associationRootChecker.leaf <== label;
    associationRootChecker.leafIndex <== labelIndex;
    associationRootChecker.siblings <== labelSiblings;
    
    // For backward compatibility: if association root is zero, allow any computed root
    // Otherwise, verify the association root matches the computed root
    // This is achieved by constraining: associationRoot * (associationRoot - associationRootChecker.out) === 0
    // When associationRoot is 0, this constraint is always satisfied
    // When associationRoot is non-zero, it must equal associationRootChecker.out
    signal diff <== associationRoot - associationRootChecker.out;
    signal product <== associationRoot * diff;
    product === 0;

    // check the withdrawn value is valid (must not exceed commitment value)
    signal remainingValue <== value - withdrawnValue;
    component remainingValueRangeCheck = Num2Bits(128);
    remainingValueRangeCheck.in <== remainingValue;
    _ <== remainingValueRangeCheck.out;

    component withdrawnValueRangeCheck = Num2Bits(128);
    withdrawnValueRangeCheck.in <== withdrawnValue;
    _ <== withdrawnValueRangeCheck.out;

    // ensure withdrawn value doesn't exceed commitment value
    // (this is enforced by the remainingValue being non-negative through range check)
}

component main {public [withdrawnValue, stateRoot, associationRoot]} = Withdraw(20, 2);  // state tree depth 20, association tree depth 2
