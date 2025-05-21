#![cfg(test)]
extern crate std;

use crate::{Bn254Contract, Bn254ContractClient, MockProof};
use ark_bn254::{G1Affine, G2Affine};
use ark_ff::UniformRand;
use ark_serialize::CanonicalSerialize;
use soroban_sdk::{BytesN, Env};

#[test]
fn test_running_contract_as_native() {
    let env = Env::default();
    let client = Bn254ContractClient::new(&env, &env.register(Bn254Contract {}, ()));

    // Generate random points
    let mut rng = ark_std::test_rng();
    let g1 = G1Affine::rand(&mut rng);
    let g2 = G2Affine::rand(&mut rng);

    // Serialize points
    let mut g1_bytes = [0u8; 64];
    let mut g2_bytes = [0u8; 128];
    g1.serialize_uncompressed(&mut g1_bytes[..]).unwrap();
    g2.serialize_uncompressed(&mut g2_bytes[..]).unwrap();
    // Create proof
    let proof = MockProof {
        g1: BytesN::from_array(&env, &g1_bytes),
        g2: BytesN::from_array(&env, &g2_bytes),
    };
    // Verify the proof
    let result = client.mock_verify(&proof);
    std::println!("`mock_verify` returned '{}'", result);

    env.cost_estimate().budget().print();

    // Below is the printout of the budget.
    //
    // Note it reports using very little resources (~12k/100M cpu insns),
    // because most of the computation is running natively (like in the host
    // but not budgeted)
    /*
    =================================================================
    Cpu limit: 100000000; used: 12150
    Mem limit: 41943040; used: 1193
    =================================================================
    *** Note: Zero-cost entries have been removed for clarity ***
    CostType                           cpu_insns      mem_bytes
    MemAlloc                           4901           1193
    MemCpy                             2048           0
    MemCmp                             548            0
    VisitObject                        915            0
    ComputeSha256Hash                  3738           0
    =================================================================
    */
}

mod bn254_contract {
    soroban_sdk::contractimport!(file = "opt/soroban_ark_bn254_contract.wasm");
}

#[test]
fn test_running_contract_as_wasm() {
    let env = Env::default();
    // This contract is too expensive to run with production budget, let's first
    // reset the budget to unlimited
    env.cost_estimate().budget().reset_unlimited();

    let contract_id = env.register(bn254_contract::WASM, ());
    let client = bn254_contract::Client::new(&env, &contract_id);

    // Generate random points
    let mut rng = ark_std::test_rng();
    let g1 = G1Affine::rand(&mut rng);
    let g2 = G2Affine::rand(&mut rng);
    // Serialize points
    let mut g1_bytes = [0u8; 64];
    let mut g2_bytes = [0u8; 128];
    g1.serialize_uncompressed(&mut g1_bytes[..]).unwrap();
    g2.serialize_uncompressed(&mut g2_bytes[..]).unwrap();

    // Create proof
    let proof = bn254_contract::MockProof {
        g1: BytesN::from_array(&env, &g1_bytes),
        g2: BytesN::from_array(&env, &g2_bytes),
    };

    let res = client.mock_verify(&proof);
    std::println!("`mock_verify` returned '{}'", res);

    env.cost_estimate().budget().print();
    // Below is the printout of the budget (the actual numbers may vary
    // depending on your Rust version and the Soroban version).
    //
    // Note most of the costs (555/560M cpu) come from wasm execution
    // (`WasmInsnExec`).
    /*
    =================================================================
    Cpu limit: 18446744073709551615; used: 560718778
    Mem limit: 18446744073709551615; used: 2543057
    =================================================================
    *** Note: Zero-cost entries have been removed for clarity ***
    CostType                           cpu_insns      mem_bytes
    WasmInsnExec                       555373528      0
    MemAlloc                           164860         1232486
    MemCpy                             10210          0
    MemCmp                             788            0
    DispatchHostFunction               1550           0
    VisitObject                        1098           0
    InvokeVmFunction                   1948           14
    ComputeSha256Hash                  3738           0
    ParseWasmInstructions              4505732        1143957
    ParseWasmFunctions                 375991         33002
    ParseWasmGlobals                   4133           314
    ParseWasmTableEntries              1171           245
    ParseWasmTypes                     116095         7073
    ParseWasmDataSegments              37083          4542
    ParseWasmElemSegments              2566           375
    ParseWasmImports                   16449          2419
    ParseWasmExports                   16772          1421
    ParseWasmDataSegmentBytes          960            8816
    InstantiateWasmInstructions        43030          70704
    InstantiateWasmFunctions           5253           10160
    InstantiateWasmGlobals             251            160
    InstantiateWasmTableEntries        128            40
    InstantiateWasmDataSegments        3599           20255
    InstantiateWasmElemSegments        331            106
    InstantiateWasmImports             19429          2288
    InstantiateWasmExports             11605          358
    InstantiateWasmDataSegmentBytes    480            4322
    =================================================================
    */
}
