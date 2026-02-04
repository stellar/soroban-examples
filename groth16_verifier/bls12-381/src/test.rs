#![cfg(test)]
extern crate std;

use ark_bls12_381::{Fq, Fq2};
use ark_serialize::CanonicalSerialize;
use core::str::FromStr;
use soroban_sdk::{
    crypto::bls12_381::{Fr, G1Affine, G2Affine, G1_SERIALIZED_SIZE, G2_SERIALIZED_SIZE},
    Env, Vec, U256,
};

use crate::{Groth16Verifier, Groth16VerifierClient, Proof, VerificationKey};

const ALPHA_X: &str = "851850525556173310373115880154698084608631105506432893865500290442025919078535925294035153152030470398262539759609";
const ALPHA_Y: &str = "2637289349983507610125993281171282870664683328789064436670091381805667870657250691837988574635646688089951719927247";

const BETA_X1: &str = "1312620381151154625549413690218290437739613987001512553647554932245743783919690104921577716179019375920325686841943";
const BETA_X2: &str = "1853421227732662200477195678252233549930451033531229987959164216695698667330234953033341200627605777603511819497457";
const BETA_Y1: &str = "3215807833988244618006117550809420301978856703407297742347804415291049013404133666905173282837707341742014140541018";
const BETA_Y2: &str = "812366606879346135498483310623227330050424196838294715759414425317592599094348477520229174120664109186562798527696";

const GAMMA_X1: &str = "352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160";
const GAMMA_X2: &str = "3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758";
const GAMMA_Y1: &str = "1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905";
const GAMMA_Y2: &str = "927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582";

const DELTA_X1: &str = "2981843938988033214458466658185878126396080429969635248100956025957789319926032198626745120548947333202362392267114";
const DELTA_X2: &str = "2236695112259305382987038341098587500598216646308901956168137697892380899086228863246537938263638056666003066263342";
const DELTA_Y1: &str = "717163810166643254871951856655865822196000925757284470845197358532703820821048809982340614428800986999944933231635";
const DELTA_Y2: &str = "3496058064578305387608803828034117220735807855182872031001942587835768203820179263722136810383631418598310938506798";

const IC0_X: &str = "829685638389803071404995253486571779300247099942205634643821309129201420207693030476756893332812706176564514055395";
const IC0_Y: &str = "3455508165409829148751617737772894557887792278044850553785496869183933597103951941805834639972489587640583544390358";
const IC1_X: &str = "2645559270376031734407122278942646687260452979296081924477586893972449945444985371392950465676350735694002713633589";
const IC1_Y: &str = "2241039659097418315097403108596818813895651201896886552939297756980670248638746432560267634304593609165964274111037";

const PI_A_X: &str = "314442236668110257304682488877371582255161413673331360366570443799415414639292047869143313601702131653514009114222";
const PI_A_Y: &str = "2384632327855835824635705027009217874826122107057894594162233214798350178691568018290025994699762298534539543934607";
const PI_B_X1: &str = "428844167033934720609657613212495751617651348480870890908850335525890280786532876634895457032623422366474694342656";
const PI_B_X2: &str = "3083139526360252775789959298805261067575555607578161553873977966165446991459924053189383038704105379290158793353905";
const PI_B_Y1: &str = "1590919422794657666432683000821892403620510405626533455397042191265963587891653562867091397248216891852168698286910";
const PI_B_Y2: &str = "3617931039814164588401589536353142503544155307022467123698224064329647390280346725086550997337076315487486714327146";
const PI_C_X: &str = "3052934797502613468327963344215392478880720823583493172692775426011388142569325036386650708808320216973179639719187";
const PI_C_Y: &str = "2028185281516938724429867827057869371578022471499780916652824405212207527699373814371051328341613972789943854539597";

fn g1_from_coords(env: &Env, x: &str, y: &str) -> G1Affine {
    let ark_g1 = ark_bls12_381::G1Affine::new(Fq::from_str(x).unwrap(), Fq::from_str(y).unwrap());
    let mut buf = [0u8; G1_SERIALIZED_SIZE];
    ark_g1.serialize_uncompressed(&mut buf[..]).unwrap();
    G1Affine::from_array(env, &buf)
}

fn g2_from_coords(env: &Env, x1: &str, x2: &str, y1: &str, y2: &str) -> G2Affine {
    let x = Fq2::new(Fq::from_str(x1).unwrap(), Fq::from_str(x2).unwrap());
    let y = Fq2::new(Fq::from_str(y1).unwrap(), Fq::from_str(y2).unwrap());
    let ark_g2 = ark_bls12_381::G2Affine::new(x, y);
    let mut buf = [0u8; G2_SERIALIZED_SIZE];
    ark_g2.serialize_uncompressed(&mut buf[..]).unwrap();
    G2Affine::from_array(env, &buf)
}

fn create_client(e: &Env) -> Groth16VerifierClient<'_> {
    Groth16VerifierClient::new(e, &e.register(Groth16Verifier {}, ()))
}

mod groth16_verifier_contract {
    soroban_sdk::contractimport!(
        file = "../opt/soroban_groth16_verifier_bls12_381.wasm"
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

    // Print out the budget report showing CPU and memory cost breakdown for
    // different operations (zero-value operations omitted for brevity)
    env.cost_estimate().budget().print();
    /*
    =================================================================
    Cpu limit: 100000000; used: 40968821
    Mem limit: 41943040; used: 297494
    =================================================================
    CostType                           cpu_insns      mem_bytes
    MemAlloc                           12089          3401
    MemCpy                             3091           0
    MemCmp                             928            0
    VisitObject                        5917           0
    ComputeSha256Hash                  3738           0
    Bls12381EncodeFp                   2644           0
    Bls12381DecodeFp                   29550          0
    Bls12381G1CheckPointOnCurve        13538          0
    Bls12381G1CheckPointInSubgroup     3652550        0
    Bls12381G2CheckPointOnCurve        23684          0
    Bls12381G2CheckPointInSubgroup     4231288        0
    Bls12381G1ProjectiveToAffine       185284         0
    Bls12381G1Add                      7689           0
    Bls12381G1Mul                      2458985        0
    Bls12381Pairing                    30335852       294093
    Bls12381FrFromU256                 1994           0
    // ... zero-value rows omitted ...
    =================================================================
    */

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
