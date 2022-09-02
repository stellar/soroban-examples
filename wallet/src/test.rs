#![cfg(test)]

use super::*;
use ed25519_dalek::Keypair;
use rand::{thread_rng, RngCore};
use soroban_auth::{Ed25519Signature, Identifier, SignaturePayload, SignaturePayloadV0};
use soroban_sdk::testutils::ed25519::Sign;
use soroban_sdk::{vec, Env, RawVal, Symbol, Vec};
use soroban_token_contract::testutils::{
    register_test_contract as register_token, to_ed25519, Token,
};

fn generate_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}

fn generate_id() -> [u8; 32] {
    let mut id: [u8; 32] = Default::default();
    thread_rng().fill_bytes(&mut id);
    id
}

fn create_token_contract(e: &Env, admin: &Keypair) -> (BytesN<32>, Token) {
    let id = generate_id();
    register_token(&e, &id);
    let token = Token::new(e, &id);
    // decimals, name, symbol don't matter in tests
    token.initialize(&to_ed25519(&e, admin), 7, "name", "symbol");

    (BytesN::from_array(&e, &id), token)
}

fn create_wallet_contract(e: &Env) -> WalletContractClient {
    let contract_id = BytesN::from_array(e, &generate_id());
    e.register_contract(&contract_id, WalletContract {});

    WalletContractClient::new(e, contract_id)
}

fn sign_args(
    env: &Env,
    signer: &Keypair,
    fn_name: &str,
    contract_id: &BytesN<32>,
    args: Vec<RawVal>,
) -> Signature {
    let msg = SignaturePayload::V0(SignaturePayloadV0 {
        function: Symbol::from_str(fn_name),
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

struct WalletTest {
    env: Env,
    wallet_admins: [Keypair; 3],
    payment_receiver: Identifier,
    token: Token,
    token_id: BytesN<32>,
    token_2: Token,
    token_id_2: BytesN<32>,
    token_admin: Keypair,
    contract: WalletContractClient,
    contract_id: Identifier,
}

impl WalletTest {
    fn setup() -> Self {
        let env: Env = Default::default();

        let wallet_admins = [generate_keypair(), generate_keypair(), generate_keypair()];

        let token_admin = generate_keypair();
        let (token_id, token) = create_token_contract(&env, &token_admin);
        let (token_id_2, token_2) = create_token_contract(&env, &token_admin);

        let contract = create_wallet_contract(&env);
        let contract_id = Identifier::Contract(contract.contract_id.clone());
        let payment_receiver = Identifier::Ed25519(BytesN::from_array(&env, &generate_id()));
        WalletTest {
            env,
            wallet_admins,
            payment_receiver,
            token,
            token_id,
            token_2,
            token_id_2,
            token_admin,
            contract,
            contract_id,
        }
    }

    fn initialize(&self, admin_weights: [u32; 3], threshold: u32) {
        let mut admins = vec![&self.env];
        for i in 0..self.wallet_admins.len() {
            admins.push_back(Admin {
                id: self.signer_to_id(&self.wallet_admins[i]),
                weight: admin_weights[i],
            });
        }

        self.contract.initialize(&admins, &threshold);
    }

    fn signer_to_id(&self, signer: &Keypair) -> Identifier {
        to_ed25519(&self.env, &signer)
    }

    fn add_wallet_balance(&self, token: &Token, amount: u32) {
        token.mint(
            &self.token_admin,
            &self.contract_id,
            &BigInt::from_u32(&self.env, amount),
        );
    }

    fn pay(&self, signers: &[&Keypair], payment_id: i64, payment: Payment) -> bool {
        let mut signatures_with_nonce = vec![&self.env];
        for signer in signers {
            let signer_id = self.signer_to_id(signer);
            let nonce = self.contract.get_nonce(&signer_id);
            signatures_with_nonce
                .push_back((self.sign_pay(&signer, &nonce, payment_id, &payment), nonce));
        }

        self.contract
            .pay(&signatures_with_nonce, &payment_id, &payment)
    }

    fn sign_pay(
        &self,
        signer: &Keypair,
        nonce: &BigInt,
        payment_id: i64,
        payment: &Payment,
    ) -> Signature {
        let signer_id = self.signer_to_id(signer);
        let args: Vec<RawVal> = (&signer_id, nonce, payment_id, payment).into_val(&self.env);

        sign_args(&self.env, signer, "pay", &self.contract.contract_id, args)
    }
}

#[test]
fn test_immediate_payment() {
    let test = WalletTest::setup();
    test.initialize([50, 50, 100], 100);

    test.add_wallet_balance(&test.token, 1000);
    test.add_wallet_balance(&test.token_2, 2000);

    // Multiple signers with enough combined weight.
    assert_eq!(
        test.pay(
            &[&test.wallet_admins[0], &test.wallet_admins[1]],
            123,
            Payment {
                receiver: test.payment_receiver.clone(),
                token: test.token_id.clone(),
                amount: BigInt::from_u32(&test.env, 300),
            },
        ),
        true
    );

    assert_eq!(
        test.token.balance(&test.payment_receiver),
        BigInt::from_u32(&test.env, 300)
    );

    // Single signer with high enough weight.
    assert_eq!(
        test.pay(
            &[&test.wallet_admins[2]],
            456,
            Payment {
                receiver: test.payment_receiver.clone(),
                token: test.token_id_2.clone(),
                amount: BigInt::from_u32(&test.env, 1500),
            },
        ),
        true
    );

    assert_eq!(
        test.token_2.balance(&test.payment_receiver),
        BigInt::from_u32(&test.env, 1500)
    );
}

#[test]
fn test_delayed_payment() {
    let test = WalletTest::setup();
    test.initialize([30, 30, 30], 90);

    let payment = Payment {
        receiver: test.payment_receiver.clone(),
        token: test.token_id.clone(),
        amount: BigInt::from_u32(&test.env, 300),
    };
    // Initialize payment - contract is not required to have the token balance yet.
    assert_eq!(
        test.pay(
            &[&test.wallet_admins[0], &test.wallet_admins[1]],
            123,
            payment.clone()
        ),
        false
    );

    assert_eq!(
        test.token.balance(&test.payment_receiver),
        BigInt::from_u32(&test.env, 0)
    );
    // Add balance and authorize the payment by the remaining signer,
    // now the payment can be cleared.
    test.add_wallet_balance(&test.token, 1000);

    assert_eq!(
        test.pay(&[&test.wallet_admins[2]], 123, payment.clone()),
        true
    );
    assert_eq!(
        test.token.balance(&test.payment_receiver),
        BigInt::from_u32(&test.env, 300)
    );
}

#[test]
fn test_mixed_payments() {
    let test = WalletTest::setup();
    test.initialize([30, 30, 30], 90);

    let delayed_payment_1 = Payment {
        receiver: test.payment_receiver.clone(),
        token: test.token_id.clone(),
        amount: BigInt::from_u32(&test.env, 500),
    };
    assert_eq!(
        test.pay(&[&test.wallet_admins[0]], 111, delayed_payment_1.clone(),),
        false
    );

    test.add_wallet_balance(&test.token, 1000);
    assert_eq!(
        test.pay(
            &[
                &test.wallet_admins[0],
                &test.wallet_admins[1],
                &test.wallet_admins[2]
            ],
            333,
            Payment {
                receiver: test.payment_receiver.clone(),
                token: test.token_id.clone(),
                amount: BigInt::from_u32(&test.env, 1000),
            },
        ),
        true
    );
    assert_eq!(
        test.token.balance(&test.payment_receiver),
        BigInt::from_u32(&test.env, 1000)
    );

    let delayed_payment_2 = Payment {
        receiver: test.payment_receiver.clone(),
        token: test.token_id_2.clone(),
        amount: BigInt::from_u32(&test.env, 2000),
    };
    assert_eq!(
        test.pay(&[&test.wallet_admins[1]], 222, delayed_payment_2.clone()),
        false
    );

    assert_eq!(
        test.pay(&[&test.wallet_admins[2]], 111, delayed_payment_1.clone()),
        false
    );
    test.add_wallet_balance(&test.token_2, 2000);
    assert_eq!(
        test.pay(
            &[&test.wallet_admins[0], &test.wallet_admins[2]],
            222,
            delayed_payment_2.clone()
        ),
        true
    );
    assert_eq!(
        test.token_2.balance(&test.payment_receiver),
        BigInt::from_u32(&test.env, 2000)
    );

    test.add_wallet_balance(&test.token, 500);
    assert_eq!(
        test.pay(&[&test.wallet_admins[1]], 111, delayed_payment_1.clone()),
        true
    );

    assert_eq!(
        test.token.balance(&test.payment_receiver),
        BigInt::from_u32(&test.env, 1500)
    );
}

#[test]
#[should_panic(expected = "contract has already been initialized")]
fn test_double_initialization() {
    let test = WalletTest::setup();
    test.initialize([30, 30, 30], 50);
    test.initialize([30, 30, 30], 50);
}

#[test]
#[should_panic(expected = "threshold has to be non-zero")]
fn test_non_zero_threshold() {
    let test = WalletTest::setup();
    test.initialize([30, 30, 30], 0);
}

#[test]
#[should_panic(expected = "admin weight is lower than threshold")]
fn test_too_high_threshold() {
    let test = WalletTest::setup();
    test.initialize([1, 2, 3], 7);
}

#[test]
#[should_panic(expected = "weight should be non-zero")]
fn test_zero_weight() {
    let test = WalletTest::setup();
    test.initialize([1, 0, 3], 1);
}

#[test]
#[should_panic(expected = "HostStorageError")]
fn test_unauthorized_signer() {
    let test = WalletTest::setup();
    test.initialize([2, 2, 2], 2);

    test.pay(
        &[&test.wallet_admins[1], &test.token_admin],
        222,
        Payment {
            receiver: test.payment_receiver.clone(),
            token: test.token_id.clone(),
            amount: BigInt::from_u32(&test.env, 300),
        },
    );
}

#[test]
#[should_panic(expected = "stored payment doesn't match new payment with same id")]
fn test_divergent_delayed_payment() {
    let test = WalletTest::setup();
    test.initialize([2, 2, 2], 4);
    test.add_wallet_balance(&test.token, 1000);

    assert_eq!(
        test.pay(
            &[&test.wallet_admins[1]],
            222,
            Payment {
                receiver: test.payment_receiver.clone(),
                token: test.token_id.clone(),
                amount: BigInt::from_u32(&test.env, 300),
            },
        ),
        false
    );

    test.pay(
        &[&test.wallet_admins[0]],
        222,
        Payment {
            receiver: test.payment_receiver.clone(),
            token: test.token_id.clone(),
            amount: BigInt::from_u32(&test.env, 299),
        },
    );
}

#[test]
#[should_panic(expected = "HostStorageError")]
fn test_payment_reexecution() {
    let test = WalletTest::setup();
    test.initialize([2, 2, 2], 2);
    let payment = Payment {
        receiver: test.payment_receiver.clone(),
        token: test.token_id.clone(),
        amount: BigInt::from_u32(&test.env, 300),
    };
    test.add_wallet_balance(&test.token, 1000);

    assert_eq!(
        test.pay(&[&test.wallet_admins[1]], 222, payment.clone()),
        true
    );

    test.pay(&[&test.wallet_admins[0]], 222, payment.clone());
}

#[test]
#[should_panic(expected = "one of the signers has already signed this payment")]
fn test_duplicate_signers() {
    let test = WalletTest::setup();
    test.initialize([2, 2, 2], 6);
    let payment = Payment {
        receiver: test.payment_receiver.clone(),
        token: test.token_id.clone(),
        amount: BigInt::from_u32(&test.env, 300),
    };
    test.add_wallet_balance(&test.token, 1000);
    assert_eq!(
        test.pay(
            &[&test.wallet_admins[0], &test.wallet_admins[1]],
            222,
            payment.clone()
        ),
        false
    );

    test.pay(
        &[&test.wallet_admins[2], &test.wallet_admins[0]],
        222,
        payment.clone(),
    );
}
