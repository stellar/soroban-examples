#![cfg(test)]

use super::*;
use ed25519_dalek::Keypair;
use rand::{thread_rng, RngCore};
use soroban_auth::{Ed25519Signature, Identifier, SignaturePayload, SignaturePayloadV0};
use soroban_sdk::testutils::ed25519::Sign;
use soroban_sdk::testutils::LedgerInfo;
use soroban_sdk::{vec, Env, IntoVal, RawVal};
use soroban_token_contract::testutils::{
    register_test_contract as register_token, to_ed25519, Token,
};

fn generate_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}

fn generate_contract_id() -> [u8; 32] {
    let mut id: [u8; 32] = Default::default();
    thread_rng().fill_bytes(&mut id);
    id
}

fn create_token_contract(e: &Env, admin: &Keypair) -> (BytesN<32>, Token) {
    let id = generate_contract_id();
    register_token(&e, &id);
    let token = Token::new(e, &id);
    // decimals, name, symbol don't matter in tests
    token.initialize(&to_ed25519(&e, admin), 7, "name", "symbol");

    (BytesN::from_array(&e, &id), token)
}

fn create_claimable_balance_contract(e: &Env) -> ClaimableBalanceContractClient {
    let contract_id = BytesN::from_array(e, &generate_contract_id());
    e.register_contract(&contract_id, ClaimableBalanceContract {});

    ClaimableBalanceContractClient::new(e, contract_id)
}

fn sign_args(
    env: &Env,
    signer: &Keypair,
    fn_name: &str,
    contract_id: &BytesN<32>,
    args: Vec<RawVal>,
) -> Signature {
    let msg = SignaturePayload::V0(SignaturePayloadV0 {
        name: Symbol::from_str(fn_name),
        contract: contract_id.clone(),
        network: env.ledger().network_passphrase(),
        args,
    });
    sign_payload(env, signer, msg)
}

fn sign_payload(env: &Env, signer: &Keypair, payload: SignaturePayload) -> Signature {
    Signature::Ed25519(Ed25519Signature {
        public_key: signer.public.to_bytes().into_val(env),
        signature: signer.sign(payload).unwrap().into_val(env),
    })
}

struct ClaimableBalanceTest {
    env: Env,
    deposit_user: Keypair,
    claim_users: [Keypair; 3],
    token: Token,
    token_id: BytesN<32>,
    contract: ClaimableBalanceContractClient,
    contract_id: Identifier,
}

impl ClaimableBalanceTest {
    fn setup() -> Self {
        let env: Env = Default::default();
        env.set_ledger(LedgerInfo {
            timestamp: 12345,
            protocol_version: 1,
            sequence_number: 10,
            network_passphrase: Default::default(),
            base_reserve: 10,
        });

        let deposit_user = generate_keypair();

        let claim_users = [generate_keypair(), generate_keypair(), generate_keypair()];

        let token_admin = generate_keypair();
        let (token_id, token) = create_token_contract(&env, &token_admin);
        token.mint(
            &token_admin,
            &to_ed25519(&env, &deposit_user),
            &BigInt::from_u32(&env, 1000),
        );

        let contract = create_claimable_balance_contract(&env);
        let contract_id = Identifier::Contract(contract.contract_id.clone());
        ClaimableBalanceTest {
            env,
            deposit_user,
            claim_users,
            token,
            token_id,
            contract,
            contract_id: contract_id,
        }
    }

    fn approve_deposit(&self, amount: u32) {
        self.token.approve(
            &self.deposit_user,
            &Identifier::Contract(self.contract.contract_id.clone()),
            &BigInt::from_u32(&self.env, amount),
        );
    }

    fn deposit(&self, amount: u32, claimants: &Vec<Identifier>, time_bound: TimeBound) {
        self.call_deposit(
            &self.deposit_user,
            &self.token_id,
            &BigInt::from_u32(&self.env, amount),
            claimants,
            &time_bound,
        );
    }

    fn claim(&self, claim_user: &Keypair) {
        self.call_claim(claim_user);
    }

    fn signer_to_id(&self, signer: &Keypair) -> Identifier {
        to_ed25519(&self.env, &signer)
    }

    fn call_deposit(
        &self,
        signer: &Keypair,
        token: &BytesN<32>,
        amount: &BigInt,
        claimants: &Vec<Identifier>,
        time_bound: &TimeBound,
    ) {
        let signer_id = self.signer_to_id(signer);
        let args: Vec<RawVal> =
            (&signer_id, token, amount, claimants, time_bound).into_val(&self.env);
        let signature = sign_args(
            &self.env,
            signer,
            "deposit",
            &self.contract.contract_id,
            args,
        );
        self.contract
            .deposit(&signature, token, amount, claimants, time_bound);
    }

    fn call_claim(&self, signer: &Keypair) {
        let signer_id = self.signer_to_id(signer);
        let args: Vec<RawVal> = (&signer_id,).into_val(&self.env);
        let signature = sign_args(&self.env, signer, "claim", &self.contract.contract_id, args);
        self.contract.claim(&signature);
    }
}

#[test]
fn test_deposit_and_claim() {
    let test = ClaimableBalanceTest::setup();
    test.approve_deposit(800);
    test.deposit(
        800,
        &vec![
            &test.env,
            test.signer_to_id(&test.claim_users[0]),
            test.signer_to_id(&test.claim_users[1]),
        ],
        TimeBound {
            kind: TimeBoundKind::Before,
            timestamp: 12346,
        },
    );

    assert_eq!(
        test.token.balance(&test.signer_to_id(&test.deposit_user)),
        BigInt::from_u32(&test.env, 200)
    );
    assert_eq!(
        test.token.balance(&test.contract_id),
        BigInt::from_u32(&test.env, 800)
    );
    assert_eq!(
        test.token.balance(&test.signer_to_id(&test.claim_users[1])),
        BigInt::from_u32(&test.env, 0)
    );

    test.claim(&test.claim_users[1]);
    assert_eq!(
        test.token.balance(&test.signer_to_id(&test.deposit_user)),
        BigInt::from_u32(&test.env, 200)
    );
    assert_eq!(
        test.token.balance(&test.contract_id),
        BigInt::from_u32(&test.env, 0)
    );
    assert_eq!(
        test.token.balance(&test.signer_to_id(&test.claim_users[1])),
        BigInt::from_u32(&test.env, 800)
    );
}

#[test]
#[should_panic(expected = "insufficient allowance")]
fn test_deposit_above_allowance_not_possible() {
    let test = ClaimableBalanceTest::setup();
    test.approve_deposit(800);
    test.deposit(
        801,
        &vec![&test.env, test.signer_to_id(&test.claim_users[0])],
        TimeBound {
            kind: TimeBoundKind::Before,
            timestamp: 12346,
        },
    );
}

#[test]
#[should_panic(expected = "contract has been already initialized")]
fn test_double_deposit_not_possible() {
    let test = ClaimableBalanceTest::setup();
    test.approve_deposit(800);
    test.deposit(
        1,
        &vec![&test.env, test.signer_to_id(&test.claim_users[0])],
        TimeBound {
            kind: TimeBoundKind::Before,
            timestamp: 12346,
        },
    );
    test.deposit(
        1,
        &vec![&test.env, test.signer_to_id(&test.claim_users[0])],
        TimeBound {
            kind: TimeBoundKind::Before,
            timestamp: 12346,
        },
    );
}

#[test]
#[should_panic(expected = "claimant is not allowed to claim this balance")]
fn test_unauthorized_claim_not_possible() {
    let test = ClaimableBalanceTest::setup();
    test.approve_deposit(800);
    test.deposit(
        800,
        &vec![
            &test.env,
            test.signer_to_id(&test.claim_users[0]),
            test.signer_to_id(&test.claim_users[1]),
        ],
        TimeBound {
            kind: TimeBoundKind::Before,
            timestamp: 12346,
        },
    );

    test.claim(&test.claim_users[2]);
}

#[test]
#[should_panic(expected = "time predicate is not fulfilled")]
fn test_out_of_time_bound_claim_not_possible() {
    let test = ClaimableBalanceTest::setup();
    test.approve_deposit(800);
    test.deposit(
        800,
        &vec![&test.env, test.signer_to_id(&test.claim_users[0])],
        TimeBound {
            kind: TimeBoundKind::After,
            timestamp: 12346,
        },
    );

    test.claim(&test.claim_users[0]);
}

#[test]
#[should_panic(expected = "HostStorageError")]
fn test_double_claim_not_possible() {
    let test = ClaimableBalanceTest::setup();
    test.approve_deposit(800);
    test.deposit(
        800,
        &vec![&test.env, test.signer_to_id(&test.claim_users[0])],
        TimeBound {
            kind: TimeBoundKind::Before,
            timestamp: 12346,
        },
    );

    test.claim(&test.claim_users[0]);
    assert_eq!(
        test.token.balance(&test.signer_to_id(&test.claim_users[0])),
        BigInt::from_u32(&test.env, 800)
    );
    test.claim(&test.claim_users[0]);
}

#[test]
#[should_panic(expected = "contract has been already initialized")]
fn test_deposit_after_claim_not_possible() {
    let test = ClaimableBalanceTest::setup();
    test.approve_deposit(1000);
    test.deposit(
        800,
        &vec![&test.env, test.signer_to_id(&test.claim_users[0])],
        TimeBound {
            kind: TimeBoundKind::After,
            timestamp: 12344,
        },
    );

    test.claim(&test.claim_users[0]);
    assert_eq!(
        test.token.balance(&test.signer_to_id(&test.claim_users[0])),
        BigInt::from_u32(&test.env, 800)
    );
    test.deposit(
        200,
        &vec![&test.env, test.signer_to_id(&test.claim_users[0])],
        TimeBound {
            kind: TimeBoundKind::After,
            timestamp: 12344,
        },
    );
}
