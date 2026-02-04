#![cfg(test)]
extern crate std;

use ark_bn254::{Fq, Fq2};
use ark_ff::{BigInteger, PrimeField};
use core::str::FromStr;
use soroban_sdk::{
    crypto::bn254::{
        Bn254G1Affine, Bn254G2Affine, Fr, BN254_G1_SERIALIZED_SIZE, BN254_G2_SERIALIZED_SIZE,
    },
    Env, Vec, U256,
};

use crate::{Groth16Verifier, Groth16VerifierClient, Proof, VerificationKey};

const ALPHA_X: &str = "20586411654224940726121807510656854616919838017362192913325723364246767479247";
const ALPHA_Y: &str = "14406135834237420410898393224422728106845847411380780163009899508656542500674";

const BETA_X1: &str = "10455785418957748805112801769878863871439287287746407726187317727278729655393";
const BETA_X2: &str = "7994732166366626571659429762957445732500444789770753725565126434770874060445";
const BETA_Y1: &str = "6171554364750907305893121967664455952867276595633237025381859103793190603186";
const BETA_Y2: &str = "1068707785281839727180321966526799350627265679563357300928626073084686493834";

const GAMMA_X1: &str = "10857046999023057135944570762232829481370756359578518086990519993285655852781";
const GAMMA_X2: &str = "11559732032986387107991004021392285783925812861821192530917403151452391805634";
const GAMMA_Y1: &str = "8495653923123431417604973247489272438418190587263600148770280649306958101930";
const GAMMA_Y2: &str = "4082367875863433681332203403145435568316851327593401208105741076214120093531";

const DELTA_X1: &str = "9728641619599932167558405925032483286291264581149582657399819891409232109311";
const DELTA_X2: &str = "19241695161990167743108773954643056384459332112654898578100132363962975297059";
const DELTA_Y1: &str = "14077793321404315919939614415644656852218893964079974656696758780514393147506";
const DELTA_Y2: &str = "477138352543384316457862019198400223292705943523596124653136815176264865609";

const IC0_X: &str = "3455198239811392358031940101382607447413352702429558918793594053377990534277";
const IC0_Y: &str = "5436753965720439827118819730906327482174186376678655329845964791381151291134";
const IC1_X: &str = "19346729925135133472054871844930104540224821692821486182760033487886118472382";
const IC1_Y: &str = "445070660711676321158305681137628146014387270520187998858724591986469989204";

const PI_A_X: &str = "15056319861143982370511851795100655862674952298862168185142634150526446898932";
const PI_A_Y: &str = "20524016742028017914278639014573029696397580893761136931992899239091515061583";
const PI_B_X1: &str = "1739426650003613298637521076298027612785708580102733347125679950635649770676";
const PI_B_X2: &str = "2964793715073763051183683532836939129978676824115763049942969571484972008340";
const PI_B_Y1: &str = "231714500818561961964881885199979457754178468650240635167721171785635224632";
const PI_B_Y2: &str = "16949586399773881191905677229599029076778019197530892858153210038293513748836";
const PI_C_X: &str = "19305753993632836293634613984434718007247114328469217861805683289168510519612";
const PI_C_Y: &str = "7647827534035254388770083633685274061394204244398195918459217009560805334325";

fn fq_to_bytes_be(fq: &Fq) -> [u8; 32] {
    let bytes = fq.into_bigint().to_bytes_be();
    let mut out = [0u8; 32];
    let start = out.len() - bytes.len();
    out[start..].copy_from_slice(&bytes);
    out
}

fn g1_from_coords(env: &Env, x: &str, y: &str) -> Bn254G1Affine {
    let ark_g1 = ark_bn254::G1Affine::new(Fq::from_str(x).unwrap(), Fq::from_str(y).unwrap());
    let mut buf = [0u8; BN254_G1_SERIALIZED_SIZE];
    let x_bytes = fq_to_bytes_be(&ark_g1.x);
    let y_bytes = fq_to_bytes_be(&ark_g1.y);
    buf[..32].copy_from_slice(&x_bytes);
    buf[32..].copy_from_slice(&y_bytes);
    Bn254G1Affine::from_array(env, &buf)
}

fn g2_from_coords(env: &Env, x1: &str, x2: &str, y1: &str, y2: &str) -> Bn254G2Affine {
    let x = Fq2::new(Fq::from_str(x1).unwrap(), Fq::from_str(x2).unwrap());
    let y = Fq2::new(Fq::from_str(y1).unwrap(), Fq::from_str(y2).unwrap());
    let ark_g2 = ark_bn254::G2Affine::new(x, y);
    let mut buf = [0u8; BN254_G2_SERIALIZED_SIZE];
    let x_c1 = fq_to_bytes_be(&ark_g2.x.c1);
    let x_c0 = fq_to_bytes_be(&ark_g2.x.c0);
    let y_c1 = fq_to_bytes_be(&ark_g2.y.c1);
    let y_c0 = fq_to_bytes_be(&ark_g2.y.c0);
    buf[0..32].copy_from_slice(&x_c1);
    buf[32..64].copy_from_slice(&x_c0);
    buf[64..96].copy_from_slice(&y_c1);
    buf[96..128].copy_from_slice(&y_c0);
    Bn254G2Affine::from_array(env, &buf)
}

fn create_client(e: &Env) -> Groth16VerifierClient<'_> {
    Groth16VerifierClient::new(e, &e.register(Groth16Verifier {}, ()))
}

mod groth16_verifier_contract {
    soroban_sdk::contractimport!(
        file = "../opt/soroban_groth16_verifier_bn254.wasm"
    );
}

fn build_vk(env: &Env) -> VerificationKey {
    VerificationKey {
        alpha: g1_from_coords(env, ALPHA_X, ALPHA_Y),
        beta: g2_from_coords(env, BETA_X1, BETA_X2, BETA_Y1, BETA_Y2),
        gamma: g2_from_coords(env, GAMMA_X1, GAMMA_X2, GAMMA_Y1, GAMMA_Y2),
        delta: g2_from_coords(env, DELTA_X1, DELTA_X2, DELTA_Y1, DELTA_Y2),
        ic: Vec::from_array(
            env,
            [
                g1_from_coords(env, IC0_X, IC0_Y),
                g1_from_coords(env, IC1_X, IC1_Y),
            ],
        ),
    }
}

fn build_proof(env: &Env) -> Proof {
    Proof {
        a: g1_from_coords(env, PI_A_X, PI_A_Y),
        b: g2_from_coords(env, PI_B_X1, PI_B_X2, PI_B_Y1, PI_B_Y2),
        c: g1_from_coords(env, PI_C_X, PI_C_Y),
    }
}

fn public_output(env: &Env, value: u32) -> Vec<Fr> {
    Vec::from_array(env, [Fr::from_u256(U256::from_u32(env, value))])
}

fn build_vk_wasm(env: &Env) -> groth16_verifier_contract::VerificationKey {
    groth16_verifier_contract::VerificationKey {
        alpha: g1_from_coords(env, ALPHA_X, ALPHA_Y).into(),
        beta: g2_from_coords(env, BETA_X1, BETA_X2, BETA_Y1, BETA_Y2).into(),
        gamma: g2_from_coords(env, GAMMA_X1, GAMMA_X2, GAMMA_Y1, GAMMA_Y2).into(),
        delta: g2_from_coords(env, DELTA_X1, DELTA_X2, DELTA_Y1, DELTA_Y2).into(),
        ic: Vec::from_array(
            env,
            [
                g1_from_coords(env, IC0_X, IC0_Y).into(),
                g1_from_coords(env, IC1_X, IC1_Y).into(),
            ],
        ),
    }
}

fn build_proof_wasm(env: &Env) -> groth16_verifier_contract::Proof {
    groth16_verifier_contract::Proof {
        a: g1_from_coords(env, PI_A_X, PI_A_Y).into(),
        b: g2_from_coords(env, PI_B_X1, PI_B_X2, PI_B_Y1, PI_B_Y2).into(),
        c: g1_from_coords(env, PI_C_X, PI_C_Y).into(),
    }
}

fn public_output_wasm(env: &Env, value: u32) -> Vec<U256> {
    Vec::from_array(env, [U256::from_u32(env, value)])
}

#[test]
fn test() {
    // Initialize the test environment
    let env = Env::default();

    // Load verification key components (copied from `data/verification_key.json`)
    // These values are pre-computed for the circuit that verifies a*b = c
    // where a=3, b=11, c=33 and only c is public.
    let vk = build_vk(&env);
    let proof = build_proof(&env);

    // Create the contract client
    let client = create_client(&env);

    // Test Case 1: Verify the proof with the correct public output (33, copied from `data/public.json`)
    let output = public_output(&env, 33);
    let res = client.verify_proof(&vk, &proof, &output);
    assert_eq!(res, true);

    // Test Case 2: Verify the proof with an incorrect public output (22)
    let output = public_output(&env, 22);
    let res = client.verify_proof(&vk, &proof, &output);
    assert_eq!(res, false);
}

#[test]
fn test_running_contract_as_wasm() {
    let env = Env::default();
    env.cost_estimate().budget().reset_unlimited();

    let contract_id = env.register(groth16_verifier_contract::WASM, ());
    let client = groth16_verifier_contract::Client::new(&env, &contract_id);

    let vk = build_vk_wasm(&env);
    let proof = build_proof_wasm(&env);

    let output = public_output_wasm(&env, 33);
    let res = client.verify_proof(&vk, &proof, &output);
    assert_eq!(res, true);

    env.cost_estimate().budget().print();
}
