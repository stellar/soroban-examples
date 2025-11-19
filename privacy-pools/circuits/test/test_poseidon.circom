pragma circom 2.2.0;

include "../poseidon255.circom";

template TestPoseidon() {
    // inputs 
    signal input in1;
    signal input in2;
    
    // outputs
    signal output out1;
    signal output out2;
    
    // components
    component hasher1 = Poseidon255(1);    // poseidon hash for 1 input

    // implementation
    hasher1.in[0] <== in1;
    out1 <== hasher1.out;

    component hasher2 = Poseidon255(2);    // poseidon hash for 2 inputs

    hasher2.in[0] <== in1;
    hasher2.in[1] <== in2;
    out2 <== hasher2.out;
}

component main { public [in1, in2] } = TestPoseidon();
