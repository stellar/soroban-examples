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
    
    
    let deltax1 = "2750191744467054372912942146482544263484467550244832445881626112777617723646810063952263428512022936903253267127350";
    let deltax2 = "2413234737575312815700598631122026291319065432043412800839944397857332202830802685415923770088689063622756702939375";
    let deltay1 = "1076967202486993406108941342102174843689250913208763125383730107292668137282535239225119066564005251774661400843821";
    let deltay2 = "784091089348445241891924627629031628871298938526420228496183038286414003726447208549611976928427786617444752683904";


    let ic0x = "1931769351244036379618100283994844046485312882458040431401676712058257124546097756332532237907637132315648906217636";
    let ic0y = "2219462221684288788247757134332962645470083865115055927456187574960992952094314940257753501443104606354496083113203";


    let ic1x = "2726325242623221693388802248110816107554759305800882344642286106642968529507795071709947858512355148550879270019178";
    let ic1y = "2690452834591447292232392438454117662004701691035040250634864436657178120453111433393322306334324558619029220405511";


    let ic2x = "2276753520377413052133204619264853734926027674320220733263964937413806530791610300908525130874383991218501161443629";
    let ic2y = "2216565042994647061456742959690979278824752277479734731836503122505090074006677407948960110633236603228440758211011";


    let ic3x = "2054702829658916052030239062784122350883101497414801284378548048954817335805733517964277882891682327579038641542963";
    let ic3y = "1861299377849520465661244108949779781960526739720579329803172490216038156998919390163110860296739149427635782605232";


    let ic4x = "2856004998221708121377069305149495649378668245327503671752831152976814973551962498318427356938380464598719642329610";
    let ic4y = "3445052445376607662168014620609501339582857414982758608624858423598446194176241135586201569345644453045853894315946";

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
    let pi_ax = "212441980386531387965179969427761278516713032280181090947921812756826140060407715059887577334725859148245723641819";
    let pi_ay = "1043777624107376029707768486456740992720703652869770475160608327115557016215114376228813623265706726948422658129414";


    let pi_bx1 = "3418594862087761466119736619773903243566736312162156790303713071542564953050210637510854068742634808271915139642232";
    let pi_bx2 = "2964213444574507673113570038406470453416167035798707123655994180726336395671895988634516761061394366777882421458970";
    let pi_by1 = "3223650775040459204413178680640740880187469067260410489432422220219327812050544783645893434878446219624098341623090";
    let pi_by2 = "2459594096752687436760263121473341283140767398180854423130454432176129735618526991834231434439933241851791233993087";


    let pi_cx = "443107262259769407693822320108000100156551631812684950355991461785927550068576958619659870180972229362608557133163";
    let pi_cy = "2104019945288105000027262551270879368920664217362082679701219406216434095932547212599338209378889102523865669374434";

    // Construct the proof from the pre-computed components
    let proof = Proof {
        a: g1_from_coords(env, &pi_ax, &pi_ay),
        b: g2_from_coords(env, &pi_bx1, &pi_bx2, &pi_by1, &pi_by2),
        c: g1_from_coords(env, &pi_cx, &pi_cy),
    };

    return proof.to_bytes(env);
}

fn init_pub_signals(env: &Env) -> Bytes {
    let public_0 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[0x4b, 0xb7, 0x52, 0xd5, 0x98, 0x01, 0xe5, 0x86, 0xfa, 0x43, 0xaa, 0x95, 0x2a, 0xb3, 0xc2, 0x31, 0xf8, 0xca, 0x8c, 0x9b, 0x86, 0x3b, 0x82, 0xca, 0x9a, 0xbd, 0x32, 0x00, 0xa7, 0xe5, 0xa2, 0x2d])); // nullifier
    let public_1 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x9a, 0xca, 0x00])); // withdrawn value
    let public_2 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[0x4a, 0x4f, 0x11, 0x8a, 0x44, 0xf7, 0xd0, 0x73, 0xe8, 0x8b, 0xae, 0x54, 0xe6, 0x20, 0x6d, 0xd2, 0x48, 0x97, 0xa5, 0x43, 0x48, 0xb9, 0xf2, 0xc8, 0xeb, 0x70, 0x7d, 0x26, 0xf4, 0x4e, 0x32, 0xbc])); // state root
    let public_3 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[0x5d, 0x58, 0x26, 0xf9, 0xc9, 0x18, 0x7b, 0xdb, 0x21, 0x3f, 0x01, 0xde, 0xd6, 0xd2, 0x30, 0xe9, 0xf1, 0xab, 0x65, 0x3b, 0x5b, 0xee, 0x60, 0x36, 0x50, 0x4e, 0x82, 0xbc, 0x07, 0x16, 0xba, 0xa2])); // Association root
    
    // Create output vector for verification: [nullifierHash, withdrawnValue, stateRoot, associationRoot]
    let output = Vec::from_array(&env, [Fr::from_u256(public_0), Fr::from_u256(public_1), Fr::from_u256(public_2), Fr::from_u256(public_3)]);
    
    let pub_signals = PublicSignals {
        pub_signals: output
    };

    return pub_signals.to_bytes(env);
}

fn init_erronous_pub_signals(env: &Env) -> Bytes {
    let public_0 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[0x65, 0x18, 0x92, 0xef, 0x37, 0x4f, 0x78, 0x93, 0x82, 0x36, 0xd4, 0x83, 0x2b, 0x62, 0xd3, 0x5f, 0xb7, 0x9c, 0x54, 0xf8, 0x72, 0xe3, 0x0f, 0x5a, 0xa9, 0xab, 0xf9, 0xe6, 0xab, 0x15, 0xcb, 0x41])); // wrong nullifier
    let public_1 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x9a, 0xca, 0x00])); // withdrawn value
    let public_2 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[0x43, 0xc7, 0x5b, 0x13, 0x4d, 0x38, 0x9a, 0x5f, 0x97, 0x8c, 0xec, 0x2a, 0x75, 0x91, 0x10, 0xe9, 0x9d, 0x1b, 0x9b, 0x7b, 0xe0, 0x34, 0x45, 0xbd, 0xb9, 0x64, 0xd3, 0x43, 0x92, 0xc5, 0x79, 0x63])); // wrong state root
    let public_3 = U256::from_be_bytes(&env, &Bytes::from_array(&env, &[0x5d, 0x58, 0x26, 0xf9, 0xc9, 0x18, 0x7b, 0xdb, 0x21, 0x3f, 0x01, 0xde, 0xd6, 0xd2, 0x30, 0xe9, 0xf1, 0xab, 0x65, 0x3b, 0x5b, 0xee, 0x60, 0x36, 0x50, 0x4e, 0x82, 0xbc, 0x07, 0x16, 0xba, 0xa2])); // Same association root as correct proof
    
    // Create output vector for verification: [nullifierHash, withdrawnValue, stateRoot, associationRoot]
    let output = Vec::from_array(&env, [Fr::from_u256(public_0), Fr::from_u256(public_1), Fr::from_u256(public_2), Fr::from_u256(public_3)]);
    
    let pub_signals = PublicSignals {
        pub_signals: output
    };

    return pub_signals.to_bytes(env);
}

fn setup_test_environment(env: &Env) -> (Address, Address, Address) {
    // Deploy groth16_verifier contract
    let groth16_verifier_id = env.register(groth16_verifier_wasm::WASM, ());

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
    let privacy_pools_id = env.register(
        PrivacyPoolsContract,
        (
            init_vk(env),
            token_id.clone(),
            admin.clone(),
            groth16_verifier_id,
        ),
    );

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
        0x10, 0xcb, 0x63, 0x1d, 0x17, 0x4a, 0x98, 0xb2,
        0x44, 0x0b, 0x68, 0xd2, 0xe5, 0x7d, 0xa2, 0xae,
        0x9a, 0x13, 0xf7, 0xd1, 0xcc, 0xcb, 0x1f, 0x41,
        0xa1, 0xdd, 0x3d, 0x69, 0xa2, 0x2f, 0xaa, 0xe9
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
        0x5d, 0x58, 0x26, 0xf9, 0xc9, 0x18, 0x7b, 0xdb,
        0x21, 0x3f, 0x01, 0xde, 0xd6, 0xd2, 0x30, 0xe9,
        0xf1, 0xab, 0x65, 0x3b, 0x5b, 0xee, 0x60, 0x36,
        0x50, 0x4e, 0x82, 0xbc, 0x07, 0x16, 0xba, 0xa2
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
        0x10, 0xcb, 0x63, 0x1d, 0x17, 0x4a, 0x98, 0xb2,
        0x44, 0x0b, 0x68, 0xd2, 0xe5, 0x7d, 0xa2, 0xae,
        0x9a, 0x13, 0xf7, 0xd1, 0xcc, 0xcb, 0x1f, 0x41,
        0xa1, 0xdd, 0x3d, 0x69, 0xa2, 0x2f, 0xaa, 0xe9
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
        0x5d, 0x58, 0x26, 0xf9, 0xc9, 0x18, 0x7b, 0xdb,
        0x21, 0x3f, 0x01, 0xde, 0xd6, 0xd2, 0x30, 0xe9,
        0xf1, 0xab, 0x65, 0x3b, 0x5b, 0xee, 0x60, 0x36,
        0x50, 0x4e, 0x82, 0xbc, 0x07, 0x16, 0xba, 0xa2
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
        0x5d, 0x58, 0x26, 0xf9, 0xc9, 0x18, 0x7b, 0xdb,
        0x21, 0x3f, 0x01, 0xde, 0xd6, 0xd2, 0x30, 0xe9,
        0xf1, 0xab, 0x65, 0x3b, 0x5b, 0xee, 0x60, 0x36,
        0x50, 0x4e, 0x82, 0xbc, 0x07, 0x16, 0xba, 0xa2
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

    // Mint tokens to alice for the deposit
    env.mock_all_auths();
    token_client.mint(&alice, &1000000000);

    // Deposit
    let commitment = BytesN::from_array(&env, &[
        0x10, 0xcb, 0x63, 0x1d, 0x17, 0x4a, 0x98, 0xb2,
        0x44, 0x0b, 0x68, 0xd2, 0xe5, 0x7d, 0xa2, 0xae,
        0x9a, 0x13, 0xf7, 0xd1, 0xcc, 0xcb, 0x1f, 0x41,
        0xa1, 0xdd, 0x3d, 0x69, 0xa2, 0x2f, 0xaa, 0xe9
    ]);
    env.mock_all_auths();
    client.deposit(&alice, &commitment);

    // Set association root to match the proof
    let association_root = BytesN::from_array(&env, &[
        0x5d, 0x58, 0x26, 0xf9, 0xc9, 0x18, 0x7b, 0xdb,
        0x21, 0x3f, 0x01, 0xde, 0xd6, 0xd2, 0x30, 0xe9,
        0xf1, 0xab, 0x65, 0x3b, 0x5b, 0xee, 0x60, 0x36,
        0x50, 0x4e, 0x82, 0xbc, 0x07, 0x16, 0xba, 0xa2
    ]);
    env.mock_all_auths();
    client.set_association_root(&admin, &association_root);

    // First withdraw - should succeed
    let proof = init_proof(&env);
    let pub_signals = init_pub_signals(&env);
    env.mock_all_auths();
    let result = client.withdraw(&bob, &proof, &pub_signals);
    assert_eq!(result, vec![&env]); // Should succeed

    // Verify the nullifier was stored
    let nullifiers = client.get_nullifiers();
    assert_eq!(nullifiers.len(), 1);

    // Attempt to reuse nullifier - should fail even though contract has no balance
    // The balance check comes first, so we need to add balance to reach the nullifier check
    env.mock_all_auths();
    token_client.mint(&contract_id, &1000000000); // Add balance directly to contract

    // Now try to withdraw again with the same proof
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
    assert_eq!(merkle_depth, 20);
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
        0x10, 0xcb, 0x63, 0x1d, 0x17, 0x4a, 0x98, 0xb2,
        0x44, 0x0b, 0x68, 0xd2, 0xe5, 0x7d, 0xa2, 0xae,
        0x9a, 0x13, 0xf7, 0xd1, 0xcc, 0xcb, 0x1f, 0x41,
        0xa1, 0xdd, 0x3d, 0x69, 0xa2, 0x2f, 0xaa, 0xe9
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
        0x10, 0xcb, 0x63, 0x1d, 0x17, 0x4a, 0x98, 0xb2,
        0x44, 0x0b, 0x68, 0xd2, 0xe5, 0x7d, 0xa2, 0xae,
        0x9a, 0x13, 0xf7, 0xd1, 0xcc, 0xcb, 0x1f, 0x41,
        0xa1, 0xdd, 0x3d, 0x69, 0xa2, 0x2f, 0xaa, 0xe9
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
        0x5d, 0x58, 0x26, 0xf9, 0xc9, 0x18, 0x7b, 0xdb,
        0x21, 0x3f, 0x01, 0xde, 0xd6, 0xd2, 0x30, 0xe9,
        0xf1, 0xab, 0x65, 0x3b, 0x5b, 0xee, 0x60, 0x36,
        0x50, 0x4e, 0x82, 0xbc, 0x07, 0x16, 0xba, 0xa2
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
        0x10, 0xcb, 0x63, 0x1d, 0x17, 0x4a, 0x98, 0xb2,
        0x44, 0x0b, 0x68, 0xd2, 0xe5, 0x7d, 0xa2, 0xae,
        0x9a, 0x13, 0xf7, 0xd1, 0xcc, 0xcb, 0x1f, 0x41,
        0xa1, 0xdd, 0x3d, 0x69, 0xa2, 0x2f, 0xaa, 0xe9
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
