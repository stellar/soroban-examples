#![cfg(test)]
use super::*;
use ark_bls12_381::{Fq, Fq2};
use ark_serialize::CanonicalSerialize;
use core::str::FromStr;
use soroban_sdk::{
    vec, Address, Bytes, BytesN, Env, String,
    crypto::bls12_381::{G1Affine, G2Affine, G1_SERIALIZED_SIZE, G2_SERIALIZED_SIZE, Fr},
    U256, symbol_short
};
use soroban_sdk::testutils::Address as TestAddress;

// Mock token contract for testing
#[contract]
pub struct MockToken;

#[contractimpl]
impl MockToken {
    pub fn initialize(env: &Env, admin: Address, decimal: u32, name: String, symbol: String) {
        env.storage().instance().set(&symbol_short!("admin"), &admin);
        env.storage().instance().set(&symbol_short!("decimal"), &decimal);
        env.storage().instance().set(&symbol_short!("name"), &name);
        env.storage().instance().set(&symbol_short!("symbol"), &symbol);
    }

    pub fn mint(env: &Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&symbol_short!("admin")).unwrap();
        admin.require_auth();
        
        let current_balance = env.storage().instance().get(&to).unwrap_or(0);
        env.storage().instance().set(&to, &(current_balance + amount));
    }

    pub fn balance(env: &Env, id: Address) -> i128 {
        env.storage().instance().get(&id).unwrap_or(0)
    }

    pub fn transfer(env: &Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        
        let from_balance = env.storage().instance().get(&from).unwrap_or(0);
        if from_balance < amount {
            panic!("insufficient balance");
        }
        
        let to_balance = env.storage().instance().get(&to).unwrap_or(0);
        env.storage().instance().set(&from, &(from_balance - amount));
        env.storage().instance().set(&to, &(to_balance + amount));
    }
}

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

fn init_vk(env: &Env) -> Bytes {
    let alphax = "2625583050305146829700663917277485398332586266229739236073977691599912239208704058548731458555934906273399977862822";
    let alphay = "1155364156944807367912876641032696519500054551629402873339575774959620483194368919563799050765095981406853619398751";
    
    
    let betax1 = "1659696755509039809248937927616726274238080235224171061036366585278216098417245587200210264410333778948851576160490";
    let betax2 = "1338363397031837211155983756179787835339490797745307535810204658838394402900152502268197396587061400659003281046656";
    let betay1 = "1974652615426136516341494326987376616840373177388374023461177997087381634383568759591087499459321812809521924259354";
    let betay2 = "3301884318087924474550898163462840036865878131635519297186391370517333773367262804074867347346141727012544462046142";
    
    
    let gammax1 = "352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160";
    let gammax2 = "3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758";
    let gammay1 = "1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905";
    let gammay2 = "927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582";
    
    
    let deltax1 = "1376803025567697113962382808101382244407999553265814114410330199802380988422048592491723065201944241242807445085937";
    let deltax2 = "1346483904056427274533832444387548951949325611993929907307901094149417278437618387449430870966524567368298714772450";
    let deltay1 = "1004876005709132503456894124088472268025739477644520910795870610385214307580754420176277586937546025567153768782375";
    let deltay2 = "126428580471956932357872436889209275406644708533169111784451949978392116445413216577181375113072093548182325364110";
    
    
    let ic0x = "3016150855432099473366983590639984454445110713446116968678013830037499756970735510342946153662213254695645747659489";
    let ic0y = "2820795672496629499377706910031044467758970893150633089988753208354373639051656661550202755258868913684121546797627";
    
    
    let ic1x = "1336810373888476991419645991074784633541531290550983502964830498876235492295204792980144598483429069131843626536651";
    let ic1y = "2838615121655609150816825426011763750666988460990399173048700262467858449610912137330266853298721768776086117102205";
    
    
    let ic2x = "103308996316173432883692026940520191319847527705470511218923126305412004868038731131332735500707093962334994531935";
    let ic2y = "2892888261581573039139228884268590149336994797805728801583227846595095045267223027324114452528341414049731172434491";
    
    
    let ic3x = "3015269266411103012646633302042724335764446851997211638416217070469815701271414388882692961765827831766188301718395";
    let ic3y = "10947886247891553991240488983005313781747685258749153949337312859863147539758543496059681340802082310265382873415";
    
    
    let ic4x = "1330406027349769923217351930865688615377332470795353972004918788360621816896279894386381516261079313798852653036135";
    let ic4y = "449041364235447728533773614670944963736157861384434972011215642158885121694498677792845574489296997555219941440652";

    let vk = VerificationKey {
        alpha: g1_from_coords(env, alphax, alphay),
        beta: g2_from_coords(env, betax1, betax2, betay1, betay2),
        gamma: g2_from_coords(env, gammax1, gammax2, gammay1, gammay2),
        delta: g2_from_coords(env, deltax1, deltax2, deltay1, deltay2),
        ic: Vec::from_array(
            &env,
            [
                g1_from_coords(env, ic0x, ic0y),
                g1_from_coords(env, ic1x, ic1y),
                g1_from_coords(env, ic2x, ic2y),
                g1_from_coords(env, ic3x, ic3y),
                g1_from_coords(env, ic4x, ic4y),
            ],
        ),
    };
    
    return vk.to_bytes(env);
}

fn init_proof(env: &Env) -> Bytes {
    let pi_ax = "3715067111669429099143660292312560879278005885424244545853210753301208542464332585402000828501729384779388248424548";
    let pi_ay = "2304729745680077083820540855269729239655611670667256496201020285848324806725315676270307139770423770554419947845051";
    
    
    let pi_bx1 = "546948967795056500471438349391767829557921771234200894052272708691369196816927165036458634970281077970134888842977";
    let pi_bx2 = "1130504290159238320450836502854516585927623604106595183027398728356656149042520513028386639075397398144180322380090";
    let pi_by1 = "934396592261487771365983194402893122146998737643904360883262048740486962118159649053901392040540166080209690832416";
    let pi_by2 = "1951290257443989607308331017610132130130996497126880116124205045861603943582427984537131136413847579176510200723509";
    
    
    let pi_cx = "545119382812692703315440826169530225910956480136825107331722114490093466818310795200352991927301980554566285298628";
    let pi_cy = "2957628432301596611505157939835811913321049285327021355514067420637658823870969135530713763413509807510382436458123";

    // Construct the proof from the pre-computed components
    let proof = Proof {
        a: g1_from_coords(env, &pi_ax, &pi_ay),
        b: g2_from_coords(env, &pi_bx1, &pi_bx2, &pi_by1, &pi_by2),
        c: g1_from_coords(env, &pi_cx, &pi_cy),
    };

    return proof.to_bytes(env);
}

fn init_pub_signals(env: &Env) -> Bytes {
    let public_0 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[0x60, 0x1a, 0x57, 0x3e, 0x7c, 0x20, 0x81, 0x0f, 0xc5, 0x10, 0xf2, 0x51, 0xc5, 0x2f, 0x99, 0xa5, 0xf9, 0x35, 0x84, 0xf7, 0x07, 0x47, 0xf5, 0xee, 0x84, 0xdf, 0x5d, 0xb4, 0x48, 0x2e, 0x9f, 0xe6]));
    let public_1 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x9a, 0xca, 0x00]));
    let public_2 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[0x45, 0xc3, 0x62, 0xdf, 0x5b, 0xdc, 0xaf, 0x41, 0x8a, 0x40, 0xe2, 0x7c, 0xf5, 0x9c, 0x8f, 0x71, 0x29, 0x45, 0xde, 0x9d, 0x17, 0x90, 0x4f, 0x16, 0xfb, 0xa8, 0xdc, 0x69, 0xd1, 0x02, 0xd4, 0x08]));
    let public_3 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[0x0c, 0x62, 0x9c, 0xe5, 0x84, 0xbe, 0xbb, 0xc0, 0xd7, 0x6e, 0x7a, 0x23, 0xbc, 0x66, 0x7c, 0x57, 0xc7, 0xe9, 0xf2, 0xcb, 0x6f, 0x6d, 0xc9, 0x3f, 0xbd, 0xe9, 0x00, 0x68, 0xb8, 0x2f, 0x74, 0xf6])); // Association root
    
    // Create output vector for verification: [nullifierHash, withdrawnValue, stateRoot, associationRoot]
    let output = Vec::from_array(&env, [Fr::from_u256(public_0), Fr::from_u256(public_1), Fr::from_u256(public_2), Fr::from_u256(public_3)]);
    
    let pub_signals = PublicSignals {
        pub_signals: output
    };

    return pub_signals.to_bytes(env);
}

fn init_erronous_pub_signals(env: &Env) -> Bytes {
    let public_0 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[0x65, 0x18, 0x92, 0xef, 0x37, 0x4f, 0x78, 0x93, 0x82, 0x36, 0xd4, 0x83, 0x2b, 0x62, 0xd3, 0x5f, 0xb7, 0x9c, 0x54, 0xf8, 0x72, 0xe3, 0x0f, 0x5a, 0xa9, 0xab, 0xf9, 0xe6, 0xab, 0x15, 0xcb, 0x41]));
    let public_1 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x9a, 0xca, 0x00]));
    let public_2 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[0x43, 0xc7, 0x5b, 0x13, 0x4d, 0x38, 0x9a, 0x5f, 0x97, 0x8c, 0xec, 0x2a, 0x75, 0x91, 0x10, 0xe9, 0x9d, 0x1b, 0x9b, 0x7b, 0xe0, 0x34, 0x45, 0xbd, 0xb9, 0x64, 0xd3, 0x43, 0x92, 0xc5, 0x79, 0x63]));
    let public_3 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[0x0c, 0x62, 0x9c, 0xe5, 0x84, 0xbe, 0xbb, 0xc0, 0xd7, 0x6e, 0x7a, 0x23, 0xbc, 0x66, 0x7c, 0x57, 0xc7, 0xe9, 0xf2, 0xcb, 0x6f, 0x6d, 0xc9, 0x3f, 0xbd, 0xe9, 0x00, 0x68, 0xb8, 0x2f, 0x74, 0xf6])); // Same association root as correct proof
    
    // Create output vector for verification: [nullifierHash, withdrawnValue, stateRoot, associationRoot]
    let output = Vec::from_array(&env, [Fr::from_u256(public_0), Fr::from_u256(public_1), Fr::from_u256(public_2), Fr::from_u256(public_3)]);
    
    let pub_signals = PublicSignals {
        pub_signals: output
    };

    return pub_signals.to_bytes(env);
}

fn setup_test_environment(env: &Env) -> (Address, Address, Address) {
    // Deploy mock token
    let token_admin = Address::generate(env);
    let token_id = env.register(MockToken, ());
    let token_client = MockTokenClient::new(env, &token_id);
    
    // Initialize token
    token_client.initialize(
        &token_admin,
        &7u32,
        &String::from_str(env, "Test Token"),
        &String::from_str(env, "TEST")
    );
    
    // Deploy privacy pools contract
    let admin = Address::generate(env);
    let privacy_pools_id = env.register(PrivacyPoolsContract, (init_vk(env), token_id.clone(), admin.clone()));
    
    (token_id, privacy_pools_id, admin)
}

#[test]
fn test_deposit_and_withdraw_correct_proof() {
    let env = Env::default();
    let (token_id, contract_id, admin) = setup_test_environment(&env);
    env.cost_estimate().budget().print();
    
    // Create test addresses
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    
    let client = PrivacyPoolsContractClient::new(&env, &contract_id);
    let token_client = MockTokenClient::new(&env, &token_id);

    // Mint tokens to alice
    env.mock_all_auths();
    token_client.mint(&alice, &1000000000);

    // Test initial balance
    assert_eq!(client.get_balance(), 0);
    assert_eq!(token_client.balance(&alice), 1000000000);

    // Test deposit
    let commitment = BytesN::from_array(&env, &[
        0x5c, 0xd2, 0x32, 0x9b, 0x7b, 0xcc, 0x98, 0x1a,
        0xea, 0xbb, 0xe6, 0x7f, 0xd2, 0xd1, 0xca, 0x42,
        0x5d, 0x35, 0x1f, 0xab, 0x7b, 0x17, 0x66, 0x7e,
        0xef, 0x82, 0x93, 0x94, 0x43, 0x51, 0x05, 0x74
    ]);
    
    // Mock authentication for alice
    env.mock_all_auths();
    client.deposit(&alice, &commitment);
    
    // Check commitments
    let commitments = client.get_commitments();
    assert_eq!(commitments.len(), 1);
    assert_eq!(commitments.get(0).unwrap(), commitment);

    // Check balances after deposit
    assert_eq!(token_client.balance(&alice), 0); // Alice's balance should be 0
    assert_eq!(token_client.balance(&contract_id), 1000000000); // Contract should have the tokens

    // Set association root to match the proof
    let association_root = BytesN::from_array(&env, &[
        0x0c, 0x62, 0x9c, 0xe5, 0x84, 0xbe, 0xbb, 0xc0,
        0xd7, 0x6e, 0x7a, 0x23, 0xbc, 0x66, 0x7c, 0x57,
        0xc7, 0xe9, 0xf2, 0xcb, 0x6f, 0x6d, 0xc9, 0x3f,
        0xbd, 0xe9, 0x00, 0x68, 0xb8, 0x2f, 0x74, 0xf6
    ]);
    env.mock_all_auths();
    let set_result = client.set_association_root(&admin, &association_root);
    assert_eq!(
        set_result,
        vec![
            &env,
            String::from_str(&env, SUCCESS_ASSOCIATION_ROOT_SET)
        ]
    );

    // Test withdraw
    let proof = init_proof(&env);
    let pub_signals = init_pub_signals(&env);
    let pub_signals_struct = PublicSignals::from_bytes(&env, &pub_signals);
    let nullifier = pub_signals_struct.pub_signals.get(0).unwrap().to_bytes();

    let result = client.withdraw(&bob, &proof, &pub_signals);
    // Success is now logged as a diagnostic event, so we return an empty vec
    assert_eq!(result, vec![&env]);

    // Check balances after withdrawal
    assert_eq!(token_client.balance(&bob), 1000000000); // Bob should have the tokens
    assert_eq!(token_client.balance(&contract_id), 0); // Contract should have 0 tokens

    // Check nullifiers
    let nullifiers = client.get_nullifiers();
    assert_eq!(nullifiers.len(), 1);
    assert_eq!(nullifiers.get(0).unwrap(), nullifier);
}

#[test]
fn test_deposit_and_withdraw_wrong_proof() {
    let env = Env::default();
    let (token_id, contract_id, admin) = setup_test_environment(&env);
    
    // Create test addresses
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    
    let client = PrivacyPoolsContractClient::new(&env, &contract_id);
    let token_client = MockTokenClient::new(&env, &token_id);

    // Mint tokens to alice
    env.mock_all_auths();
    token_client.mint(&alice, &1000000000);

    // Test initial balance
    assert_eq!(client.get_balance(), 0);
    assert_eq!(token_client.balance(&alice), 1000000000);

    // Test deposit
    let commitment = BytesN::from_array(&env, &[
        0x5c, 0xd2, 0x32, 0x9b, 0x7b, 0xcc, 0x98, 0x1a,
        0xea, 0xbb, 0xe6, 0x7f, 0xd2, 0xd1, 0xca, 0x42,
        0x5d, 0x35, 0x1f, 0xab, 0x7b, 0x17, 0x66, 0x7e,
        0xef, 0x82, 0x93, 0x94, 0x43, 0x51, 0x05, 0x74
    ]);

    // Mock authentication for alice
    env.mock_all_auths();
    client.deposit(&alice, &commitment);
    
    // Check commitments
    let commitments = client.get_commitments();
    assert_eq!(commitments.len(), 1);
    assert_eq!(commitments.get(0).unwrap(), commitment);

    // Set association root to match the erroneous pub signals
    let association_root = BytesN::from_array(&env, &[
        0x0c, 0x62, 0x9c, 0xe5, 0x84, 0xbe, 0xbb, 0xc0,
        0xd7, 0x6e, 0x7a, 0x23, 0xbc, 0x66, 0x7c, 0x57,
        0xc7, 0xe9, 0xf2, 0xcb, 0x6f, 0x6d, 0xc9, 0x3f,
        0xbd, 0xe9, 0x00, 0x68, 0xb8, 0x2f, 0x74, 0xf6
    ]);
    env.mock_all_auths();
    client.set_association_root(&admin, &association_root);

    // Test withdraw with wrong proof (different state root)
    let proof = init_proof(&env);
    let pub_signals = init_erronous_pub_signals(&env);
    
    let result = client.withdraw(&bob, &proof, &pub_signals);
    assert_eq!(
        result,
        vec![
            &env,
            String::from_str(&env, ERROR_COIN_OWNERSHIP_PROOF)
        ]
    );
    
    // Check that balances are unchanged (withdrawal failed)
    assert_eq!(token_client.balance(&bob), 0); // Bob should still have 0
    assert_eq!(token_client.balance(&contract_id), 1000000000); // Contract should still have tokens
    
    let nullifiers = client.get_nullifiers();
    assert_eq!(nullifiers.len(), 0); // No nullifiers should be stored
}

#[test]
fn test_withdraw_insufficient_balance() {
    let env = Env::default();
    let (_token_id, contract_id, admin) = setup_test_environment(&env);
    let client = PrivacyPoolsContractClient::new(&env, &contract_id);

    // Set association root to match the proof
    let association_root = BytesN::from_array(&env, &[
        0x0c, 0x62, 0x9c, 0xe5, 0x84, 0xbe, 0xbb, 0xc0,
        0xd7, 0x6e, 0x7a, 0x23, 0xbc, 0x66, 0x7c, 0x57,
        0xc7, 0xe9, 0xf2, 0xcb, 0x6f, 0x6d, 0xc9, 0x3f,
        0xbd, 0xe9, 0x00, 0x68, 0xb8, 0x2f, 0x74, 0xf6
    ]);
    env.mock_all_auths();
    client.set_association_root(&admin, &association_root);

    let bob = Address::generate(&env);
    let proof = init_proof(&env);
    let pub_signals = init_pub_signals(&env);
    
    // Attempt to withdraw with zero balance
    env.mock_all_auths();
    let result = client.withdraw(&bob, &proof, &pub_signals);
    assert_eq!(
        result,
        vec![
            &env,
            String::from_str(&env, ERROR_INSUFFICIENT_BALANCE)
        ]
    );

    // Ensure nullifier was not stored when withdrawal failed
    assert_eq!(client.get_nullifiers().len(), 0);
}

#[test]
fn test_reuse_nullifier() {
    let env = Env::default();
    let (token_id, contract_id, admin) = setup_test_environment(&env);
    let client = PrivacyPoolsContractClient::new(&env, &contract_id);
    let token_client = MockTokenClient::new(&env, &token_id);

    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    // Mint tokens to alice
    env.mock_all_auths();
    token_client.mint(&alice, &2000000000); // Mint enough for two deposits

    // First deposit
    let commitment1 = BytesN::from_array(&env, &[
        0x5c, 0xd2, 0x32, 0x9b, 0x7b, 0xcc, 0x98, 0x1a,
        0xea, 0xbb, 0xe6, 0x7f, 0xd2, 0xd1, 0xca, 0x42,
        0x5d, 0x35, 0x1f, 0xab, 0x7b, 0x17, 0x66, 0x7e,
        0xef, 0x82, 0x93, 0x94, 0x43, 0x51, 0x05, 0x74
    ]);
    env.mock_all_auths();
    client.deposit(&alice, &commitment1);

    // Set association root to match the proof
    let association_root = BytesN::from_array(&env, &[
        0x0c, 0x62, 0x9c, 0xe5, 0x84, 0xbe, 0xbb, 0xc0,
        0xd7, 0x6e, 0x7a, 0x23, 0xbc, 0x66, 0x7c, 0x57,
        0xc7, 0xe9, 0xf2, 0xcb, 0x6f, 0x6d, 0xc9, 0x3f,
        0xbd, 0xe9, 0x00, 0x68, 0xb8, 0x2f, 0x74, 0xf6
    ]);
    env.mock_all_auths();
    client.set_association_root(&admin, &association_root);

    // First withdraw
    let proof = init_proof(&env);
    let pub_signals = init_pub_signals(&env);
    env.mock_all_auths();
    client.withdraw(&bob, &proof, &pub_signals);

    // Second deposit
    let commitment2 = BytesN::from_array(&env, &[6u8; 32]);
    env.mock_all_auths();
    client.deposit(&alice, &commitment2);
    
    // Attempt to reuse nullifier
    env.mock_all_auths();
    let result = client.withdraw(&bob, &proof, &pub_signals);
    assert_eq!(
        result,
        vec![
            &env,
            String::from_str(&env, ERROR_NULLIFIER_USED)
        ]
    );
}

#[test]
fn test_contract_initialization() {
    let env = Env::default();
    let (_token_id, contract_id, _admin) = setup_test_environment(&env);
    let client = PrivacyPoolsContractClient::new(&env, &contract_id);
    
    // Test that contract initializes correctly
    let merkle_root = client.get_merkle_root();
    let merkle_depth = client.get_merkle_depth();
    let commitment_count = client.get_commitment_count();
    let commitments = client.get_commitments();
    let nullifiers = client.get_nullifiers();
    
    // Verify initial state
    assert_eq!(merkle_depth, 2);
    assert_eq!(commitment_count, 0);
    assert_eq!(commitments.len(), 0);
    assert_eq!(nullifiers.len(), 0);
    
    // Merkle root should be initialized (not all zeros)
    assert_ne!(merkle_root, BytesN::from_array(&env, &[0u8; 32]));
}

#[test]
#[should_panic(expected = "Association root must be set before withdrawal")]
fn test_withdraw_without_association_set() {
    let env = Env::default();
    let (token_id, contract_id, _admin) = setup_test_environment(&env);
    
    // Create test addresses
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    
    let client = PrivacyPoolsContractClient::new(&env, &contract_id);
    let token_client = MockTokenClient::new(&env, &token_id);
    
    // Mint tokens to alice
    env.mock_all_auths();
    token_client.mint(&alice, &1000000000);

    // Test initial balance
    assert_eq!(client.get_balance(), 0);
    assert_eq!(token_client.balance(&alice), 1000000000);

    // Test deposit - use the same commitment as in our proof
    let commitment = BytesN::from_array(&env, &[
        0x5c, 0xd2, 0x32, 0x9b, 0x7b, 0xcc, 0x98, 0x1a,
        0xea, 0xbb, 0xe6, 0x7f, 0xd2, 0xd1, 0xca, 0x42,
        0x5d, 0x35, 0x1f, 0xab, 0x7b, 0x17, 0x66, 0x7e,
        0xef, 0x82, 0x93, 0x94, 0x43, 0x51, 0x05, 0x74
    ]);
    
    // Mock authentication for alice
    env.mock_all_auths();
    client.deposit(&alice, &commitment);
    
    // Check commitments
    let commitments = client.get_commitments();
    assert_eq!(commitments.len(), 1);
    assert_eq!(commitments.get(0).unwrap(), commitment);

    // Check balances after deposit
    assert_eq!(token_client.balance(&alice), 0); // Alice's balance should be 0
    assert_eq!(token_client.balance(&contract_id), 1000000000); // Contract should have the tokens

    // Verify no association set is configured
    assert_eq!(client.has_association_set(), false);

    // Verify state before withdrawal attempt
    assert_eq!(token_client.balance(&bob), 0); // Bob should have 0
    assert_eq!(token_client.balance(&contract_id), 1000000000); // Contract should have tokens
    assert_eq!(client.get_nullifiers().len(), 0); // No nullifiers should be stored

    // Test withdraw with no association set configured
    // Since association root is now required, withdrawal should panic
    let proof = init_proof(&env);
    let pub_signals = init_pub_signals(&env);

    env.mock_all_auths();
    client.withdraw(&bob, &proof, &pub_signals);
}

#[test]
fn test_withdraw_association_root_mismatch() {
    let env = Env::default();
    let (token_id, contract_id, admin) = setup_test_environment(&env);
    
    // Create test addresses
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    
    let client = PrivacyPoolsContractClient::new(&env, &contract_id);
    let token_client = MockTokenClient::new(&env, &token_id);
    
    // Mint tokens to alice
    env.mock_all_auths();
    token_client.mint(&alice, &1000000000);

    // Test initial balance
    assert_eq!(client.get_balance(), 0);
    assert_eq!(token_client.balance(&alice), 1000000000);

    // Test deposit - use the same commitment as in our proof
    let commitment = BytesN::from_array(&env, &[
        0x5c, 0xd2, 0x32, 0x9b, 0x7b, 0xcc, 0x98, 0x1a,
        0xea, 0xbb, 0xe6, 0x7f, 0xd2, 0xd1, 0xca, 0x42,
        0x5d, 0x35, 0x1f, 0xab, 0x7b, 0x17, 0x66, 0x7e,
        0xef, 0x82, 0x93, 0x94, 0x43, 0x51, 0x05, 0x74
    ]);
    
    // Mock authentication for alice
    env.mock_all_auths();
    client.deposit(&alice, &commitment);
    
    // Check commitments
    let commitments = client.get_commitments();
    assert_eq!(commitments.len(), 1);
    assert_eq!(commitments.get(0).unwrap(), commitment);

    // Check balances after deposit
    assert_eq!(token_client.balance(&alice), 0); // Alice's balance should be 0
    assert_eq!(token_client.balance(&contract_id), 1000000000); // Contract should have the tokens

    // Set an incorrect association root (different from the one in the proof)
    let incorrect_association_root = BytesN::from_array(&env, &[
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
    ]);
    env.mock_all_auths();
    let set_result = client.set_association_root(&admin, &incorrect_association_root);
    assert_eq!(
        set_result,
        vec![
            &env,
            String::from_str(&env, SUCCESS_ASSOCIATION_ROOT_SET)
        ]
    );

    // Verify association set is configured
    assert_eq!(client.has_association_set(), true);

    // Test withdraw with proof that has a different association root
    let proof = init_proof(&env);
    let pub_signals = init_pub_signals(&env); // This has the correct association root for the proof

    let result = client.withdraw(&bob, &proof, &pub_signals);
    assert_eq!(
        result,
        vec![
            &env,
            String::from_str(&env, "Association set root mismatch")
        ]
    );

    // Check that balances are unchanged (withdrawal failed)
    assert_eq!(token_client.balance(&bob), 0); // Bob should still have 0
    assert_eq!(token_client.balance(&contract_id), 1000000000); // Contract should still have tokens
    
    // Check that no nullifier was stored when withdrawal failed
    let nullifiers = client.get_nullifiers();
    assert_eq!(nullifiers.len(), 0);
}

#[test]
fn test_set_association_root_non_admin() {
    let env = Env::default();
    let (_token_id, contract_id, _admin) = setup_test_environment(&env);
    let client = PrivacyPoolsContractClient::new(&env, &contract_id);
    
    // Create a non-admin user
    let non_admin = Address::generate(&env);
    
    // Create a test association root
    let association_root = BytesN::from_array(&env, &[
        0x0c, 0x62, 0x9c, 0xe5, 0x84, 0xbe, 0xbb, 0xc0,
        0xd7, 0x6e, 0x7a, 0x23, 0xbc, 0x66, 0x7c, 0x57,
        0xc7, 0xe9, 0xf2, 0xcb, 0x6f, 0x6d, 0xc9, 0x3f,
        0xbd, 0xe9, 0x00, 0x68, 0xb8, 0x2f, 0x74, 0xf6
    ]);
    
    // Mock authentication for the non-admin user
    env.mock_all_auths();
    
    // Attempt to call set_association_root with non-admin should return error
    let result = client.set_association_root(&non_admin, &association_root);
    
    // Verify that the call returned an error message
    assert_eq!(
        result,
        vec![
            &env,
            String::from_str(&env, ERROR_ONLY_ADMIN)
        ]
    );
    
    // Verify that no association root was set (should still be zero)
    let stored_root = client.get_association_root();
    let zero_root = BytesN::from_array(&env, &[0u8; 32]);
    assert_eq!(stored_root, zero_root, "Association root should not be set by non-admin");
    
    // Verify that has_association_set returns false
    assert_eq!(client.has_association_set(), false, "Should not have association set after failed non-admin call");
}

#[test]
#[should_panic(expected = "Association root must be set before withdrawal")]
fn test_withdraw_requires_association_root() {
    let env = Env::default();
    let (token_id, contract_id, _admin) = setup_test_environment(&env);
    
    // Create test addresses
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    
    let client = PrivacyPoolsContractClient::new(&env, &contract_id);
    let token_client = MockTokenClient::new(&env, &token_id);
    
    // Mint tokens to alice
    env.mock_all_auths();
    token_client.mint(&alice, &1000000000);

    // Test deposit
    let commitment = BytesN::from_array(&env, &[
        0x5c, 0xd2, 0x32, 0x9b, 0x7b, 0xcc, 0x98, 0x1a,
        0xea, 0xbb, 0xe6, 0x7f, 0xd2, 0xd1, 0xca, 0x42,
        0x5d, 0x35, 0x1f, 0xab, 0x7b, 0x17, 0x66, 0x7e,
        0xef, 0x82, 0x93, 0x94, 0x43, 0x51, 0x05, 0x74
    ]);
    
    // Mock authentication for alice
    env.mock_all_auths();
    client.deposit(&alice, &commitment);
    
    // Check balances after deposit
    assert_eq!(token_client.balance(&alice), 0); // Alice's balance should be 0
    assert_eq!(token_client.balance(&contract_id), 1000000000); // Contract should have the tokens

    // Verify no association set is configured
    assert_eq!(client.has_association_set(), false);

    // Verify state before withdrawal attempt
    assert_eq!(token_client.balance(&bob), 0); // Bob should have 0
    assert_eq!(token_client.balance(&contract_id), 1000000000); // Contract should have tokens
    assert_eq!(client.get_nullifiers().len(), 0); // No nullifiers should be stored

    // Attempt to withdraw without setting association root - this should panic
    let proof = init_proof(&env);
    let pub_signals = init_pub_signals(&env);

    env.mock_all_auths();
    client.withdraw(&bob, &proof, &pub_signals);
}

#[cfg(feature = "test_hash")]
#[test]
fn test_hash_method() {
    let env = Env::default();
    let token_address = Address::generate(&env);
    let contract_id = env.register(PrivacyPoolsContract, (init_vk(&env), token_address));
    let client = PrivacyPoolsContractClient::new(&env, &contract_id);
    
    // Should execute without panicking
    client.test_hash();
}
