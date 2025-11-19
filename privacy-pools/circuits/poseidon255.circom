pragma circom 2.0.0;

include "./poseidon255_constants.circom";

template x5() {
    signal input in;
    signal output out;

    signal in2 <== in * in;
    signal in4 <== in2 * in2;

    out <== in4 * in;
}

template m(t, M) {
    signal input in[t];
    signal output out[t];

    var res;
    for (var i=0; i<t; i++) {
        res = 0;
        for (var j=0; j<t; j++) {
            res = res + in[j] * M[i][j];
        }
        res ==> out[i];
    }
}

template ark(t, C, index) {
    signal input in[t];
    signal output out[t];

    for (var i=0; i<t; i++) {
        out[i] <== in[i] + C[index * t + i];
    }
}


template Poseidon255(nInputs) {
    signal input in[nInputs];
    signal output out;

    var N_P_ARRAY[16] = [56, 56, 56, 56, 57, 57, 57, 57, 57, 57, 57, 57, 57, 57, 57, 57];

    var t = nInputs + 1;
    var N_P = N_P_ARRAY[nInputs - 1];
    var N_F = 8;
    var C[t * (N_P + N_F)] = CONSTANTS(t);

    var M[t][t] = MATRIX(t);

    component x5F[N_F][t];
    component x5P[N_P];

    component ark[N_F + N_P];
    component m[N_F + N_P];

    var index;

    for (var i=0; i<N_P + N_F; i++) {
        ark[i] = ark(t, C, i);

        for (var j=0; j<t; j++) {
            if (i == 0) {
                if (j == 0) {
                    ark[i].in[j] <== 0; 
                } else {
                    ark[i].in[j] <== in[j - 1];
                }
            } else {
                ark[i].in[j] <== m[i - 1].out[j]; 
            }
        }
        
        m[i] = m(t, M);

        if (i < N_F / 2 || i >= N_F / 2 + N_P) {
            index = i < N_F / 2 ? i : i - N_P;

            for (var j=0; j<t; j++) {
                x5F[index][j] = x5();
                x5F[index][j].in <== ark[i].out[j]; 
                x5F[index][j].out ==> m[i].in[j];   
            }
        } else {
            index = i - N_F / 2;
            x5P[index] = x5();

            x5P[index].in <== ark[i].out[0];
            x5P[index].out ==> m[i].in[0];

            for (var j=1; j<t; j++) {
                ark[i].out[j] ==> m[i].in[j];
            }
        }
    }
    

    out <== m[N_F + N_P - 1].out[0];
}