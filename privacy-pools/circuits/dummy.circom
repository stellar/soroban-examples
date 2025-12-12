pragma circom 2.2.0;

include "commitment.circom";
include "comparators.circom";

// This circuit is for withdrawing by demonstrating only that the withdrawer
// knows the secret and nullifier, without proving inclusion of the commitment in the Merkle tree.
// This is for testing purposes only, to enable testing depositing and withdrawing coins prior to 
// testing the full Merkle tree implementation.

template Dummy() {
    // Inputs
    signal input withdrawnValue;
    signal input label;                 // hash(scope, nonce) % SNARK_SCALAR_FIELD
    signal input value;         // value of the existing commitment
    signal input nullifier;     // nullifier of the existing commitment
    signal input secret;        // Secret of the existing commitment

    // Outputs
    signal output existingNullifierHash;    // hash of existing commitment nullifier

    // IMPLEMENTATION

    component commitmentHasher = CommitmentHasher();
    commitmentHasher.value <== withdrawnValue;
    commitmentHasher.label <== label;
    commitmentHasher.secret <== secret;
    commitmentHasher.nullifier <== nullifier;

    signal existingCommitment <== commitmentHasher.commitment;
    existingNullifierHash <== commitmentHasher.nullifierHash;

    // check the withdrawn value is valid
    signal remainingValue <== value - withdrawnValue;
    component remainingValueRangeCheck = Num2Bits(128);
    remainingValueRangeCheck.in <== remainingValue;
    _ <== remainingValueRangeCheck.out;

    component withdrawnValueRangeCheck = Num2Bits(128);
    withdrawnValueRangeCheck.in <== withdrawnValue;
    _ <== withdrawnValueRangeCheck.out;
}

component main = Dummy();