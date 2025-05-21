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

fn create_client(e: &Env) -> Groth16VerifierClient {
    Groth16VerifierClient::new(e, &e.register(Groth16Verifier {}, ()))
}

#[test]
fn test() {
    // Initialize the test environment
    let env = Env::default();

    // Load verification key components (copied from `data/verification_key.json`)
    // These values are pre-computed for the circuit that verifies a*b = c
    // where a=3, b=11, c=33 and only c is public.
    let alphax = "851850525556173310373115880154698084608631105506432893865500290442025919078535925294035153152030470398262539759609";
    let alphay = "2637289349983507610125993281171282870664683328789064436670091381805667870657250691837988574635646688089951719927247";

    let betax1 = "1312620381151154625549413690218290437739613987001512553647554932245743783919690104921577716179019375920325686841943";
    let betax2 = "1853421227732662200477195678252233549930451033531229987959164216695698667330234953033341200627605777603511819497457";
    let betay1 = "3215807833988244618006117550809420301978856703407297742347804415291049013404133666905173282837707341742014140541018";
    let betay2 = "812366606879346135498483310623227330050424196838294715759414425317592599094348477520229174120664109186562798527696";

    let gammax1 = "352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160";
    let gammax2 = "3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758";
    let gammay1 = "1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905";
    let gammay2 = "927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582";

    let deltax1 = "2981843938988033214458466658185878126396080429969635248100956025957789319926032198626745120548947333202362392267114";
    let deltax2 = "2236695112259305382987038341098587500598216646308901956168137697892380899086228863246537938263638056666003066263342";
    let deltay1 = "717163810166643254871951856655865822196000925757284470845197358532703820821048809982340614428800986999944933231635";
    let deltay2 = "3496058064578305387608803828034117220735807855182872031001942587835768203820179263722136810383631418598310938506798";

    let ic0x = "829685638389803071404995253486571779300247099942205634643821309129201420207693030476756893332812706176564514055395";
    let ic0y = "3455508165409829148751617737772894557887792278044850553785496869183933597103951941805834639972489587640583544390358";

    let ic1x = "2645559270376031734407122278942646687260452979296081924477586893972449945444985371392950465676350735694002713633589";
    let ic1y = "2241039659097418315097403108596818813895651201896886552939297756980670248638746432560267634304593609165964274111037";

    // Construct the verification key from the pre-computed components
    let vk = VerificationKey {
        alpha: g1_from_coords(&env, alphax, alphay),
        beta: g2_from_coords(&env, betax1, betax2, betay1, betay2),
        gamma: g2_from_coords(&env, gammax1, gammax2, gammay1, gammay2),
        delta: g2_from_coords(&env, deltax1, deltax2, deltay1, deltay2),
        ic: Vec::from_array(
            &env,
            [
                g1_from_coords(&env, ic0x, ic0y),
                g1_from_coords(&env, ic1x, ic1y),
            ],
        ),
    };

    // Load proof components (copied from `data/proof.json`)
    let pi_ax = "314442236668110257304682488877371582255161413673331360366570443799415414639292047869143313601702131653514009114222";
    let pi_ay = "2384632327855835824635705027009217874826122107057894594162233214798350178691568018290025994699762298534539543934607";
    let pi_bx1 = "428844167033934720609657613212495751617651348480870890908850335525890280786532876634895457032623422366474694342656";
    let pi_bx2 = "3083139526360252775789959298805261067575555607578161553873977966165446991459924053189383038704105379290158793353905";
    let pi_by1 = "1590919422794657666432683000821892403620510405626533455397042191265963587891653562867091397248216891852168698286910";
    let pi_by2 = "3617931039814164588401589536353142503544155307022467123698224064329647390280346725086550997337076315487486714327146";
    let pi_cx = "3052934797502613468327963344215392478880720823583493172692775426011388142569325036386650708808320216973179639719187";
    let pi_cy = "2028185281516938724429867827057869371578022471499780916652824405212207527699373814371051328341613972789943854539597";

    // Construct the proof from the pre-computed components
    let proof = Proof {
        a: g1_from_coords(&env, &pi_ax, &pi_ay),
        b: g2_from_coords(&env, &pi_bx1, &pi_bx2, &pi_by1, &pi_by2),
        c: g1_from_coords(&env, &pi_cx, &pi_cy),
    };

    // Create the contract client
    let client = create_client(&env);

    // Test Case 1: Verify the proof with the correct public output (33, copied from `data/public.json`)
    let output = Vec::from_array(&env, [Fr::from_u256(U256::from_u32(&env, 33))]);
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
    let output = Vec::from_array(&env, [Fr::from_u256(U256::from_u32(&env, 22))]);
    let res = client.verify_proof(&vk, &proof, &output);
    assert_eq!(res, false);
}
