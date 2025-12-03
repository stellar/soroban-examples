pragma circom 2.2.0;

include "../commitment.circom";

template TestCommitment() {
    signal input value;
    signal input label;
    signal input secret;
    signal input nullifier;

    component commitmentHasher = CommitmentHasher();
    commitmentHasher.value <== value;
    commitmentHasher.label <== label;
    commitmentHasher.secret <== secret;
    commitmentHasher.nullifier <== nullifier;

    signal output commitment;
    signal output nullifierHash;
    
    commitment <== commitmentHasher.commitment;
    nullifierHash <== commitmentHasher.nullifierHash;
}

component main = TestCommitment();