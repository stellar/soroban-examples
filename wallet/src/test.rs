#![cfg(test)]

use core::fmt::Debug;

use super::*;
use ed25519_dalek::Keypair;
use rand::{thread_rng, RngCore};
use soroban_auth::{Ed25519Signature, Identifier, SignaturePayload, SignaturePayloadV0};
use soroban_sdk::testutils::ed25519::Sign;
use soroban_sdk::testutils::Accounts;
use soroban_sdk::{vec, AccountId, Env, IntoVal, RawVal, Status, Symbol, Vec};
use token::{Client as TokenClient, TokenMetadata};

fn generate_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}

fn generate_id() -> [u8; 32] {
    let mut id: [u8; 32] = Default::default();
    thread_rng().fill_bytes(&mut id);
    id
}

// Note: we use `AccountId` here and `Ed25519` signers everywhere else in this
// test only for the sake of the test setup simplicity. There are no limitations
// on types of identifiers used in any contexts here.
fn create_token_contract(e: &Env, admin: &AccountId) -> (BytesN<32>, TokenClient) {
    let id = e.register_contract_token(None);
    let token = TokenClient::new(e, &id);
    // decimals, name, symbol don't matter in tests
    token.init(
        &Identifier::Account(admin.clone()),
        &TokenMetadata {
            name: "name".into_val(e),
            symbol: "symbol".into_val(e),
            decimals: 7,
        },
    );
    (id, token)
}

fn create_wallet_contract(e: &Env) -> WalletContractClient {
    WalletContractClient::new(e, e.register_contract(None, WalletContract {}))
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

struct WalletTest {
    env: Env,
    wallet_admins: [Keypair; 3],
    payment_receiver: Identifier,
    token: TokenClient,
    token_id: BytesN<32>,
    token_2: TokenClient,
    token_id_2: BytesN<32>,
    token_admin: AccountId,
    contract: WalletContractClient,
    contract_id: Identifier,
}

fn extract_error<T, U: Debug>(
    res: Result<Result<T, U>, Result<Error, Status>>,
) -> Result<T, Error> {
    match res {
        Ok(v) => Ok(v.unwrap()),
        Err(e) => Err(e.unwrap()),
    }
}

impl WalletTest {
    fn setup() -> Self {
        let env: Env = Default::default();

        let wallet_admins = [generate_keypair(), generate_keypair(), generate_keypair()];

        let token_admin = env.accounts().generate();
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

    fn initialize(&self, admin_weights: [u32; 3], threshold: u32) -> Result<(), Error> {
        let mut admins = vec![&self.env];
        for i in 0..self.wallet_admins.len() {
            admins.push_back(Admin {
                id: self.signer_to_id(&self.wallet_admins[i]),
                weight: admin_weights[i],
            });
        }

        extract_error(self.contract.try_initialize(&admins, &threshold))
    }

    fn signer_to_id(&self, signer: &Keypair) -> Identifier {
        Identifier::Ed25519(BytesN::<32>::from_array(
            &self.env,
            &signer.public.to_bytes(),
        ))
    }

    fn add_wallet_balance(&self, token: &TokenClient, amount: u32) {
        token.with_source_account(&self.token_admin).mint(
            &Signature::Invoker,
            &0,
            &self.contract_id,
            &(amount as i128),
        );
    }

    fn pay(&self, signers: &[&Keypair], payment_id: i64, payment: Payment) -> Result<bool, Error> {
        let mut signatures = vec![&self.env];
        for signer in signers {
            signatures.push_back(self.sign_pay(signer, payment_id, &payment));
        }
        extract_error(self.contract.try_pay(&signatures, &payment_id, &payment))
    }

    fn sign_pay(&self, signer: &Keypair, payment_id: i64, payment: &Payment) -> Signature {
        sign_args(
            &self.env,
            signer,
            "pay",
            &self.contract.contract_id,
            (&self.signer_to_id(signer), payment_id, payment).into_val(&self.env),
        )
    }
}

#[test]
fn test_immediate_payment() {
    let test = WalletTest::setup();
    test.initialize([50, 50, 100], 100).unwrap();

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
                amount: 300,
            },
        ),
        Ok(true)
    );

    assert_eq!(test.token.balance(&test.payment_receiver), 300);

    // Single signer with high enough weight.
    assert_eq!(
        test.pay(
            &[&test.wallet_admins[2]],
            456,
            Payment {
                receiver: test.payment_receiver.clone(),
                token: test.token_id_2.clone(),
                amount: 1500,
            },
        ),
        Ok(true)
    );

    assert_eq!(test.token_2.balance(&test.payment_receiver), 1500);
}

#[test]
fn test_delayed_payment() {
    let test = WalletTest::setup();
    test.initialize([30, 30, 30], 90).unwrap();

    let payment = Payment {
        receiver: test.payment_receiver.clone(),
        token: test.token_id.clone(),
        amount: 300,
    };
    // Initialize payment - contract is not required to have the token balance yet.
    assert_eq!(
        test.pay(
            &[&test.wallet_admins[0], &test.wallet_admins[1]],
            123,
            payment.clone()
        ),
        Ok(false)
    );

    assert_eq!(test.token.balance(&test.payment_receiver), 0);
    // Add balance and authorize the payment by the remaining signer,
    // now the payment can be cleared.
    test.add_wallet_balance(&test.token, 1000);

    assert_eq!(test.pay(&[&test.wallet_admins[2]], 123, payment), Ok(true));
    assert_eq!(test.token.balance(&test.payment_receiver), 300);
}

#[test]
fn test_mixed_payments() {
    let test = WalletTest::setup();
    test.initialize([30, 30, 30], 90).unwrap();

    let delayed_payment_1 = Payment {
        receiver: test.payment_receiver.clone(),
        token: test.token_id.clone(),
        amount: 500,
    };
    assert_eq!(
        test.pay(&[&test.wallet_admins[0]], 111, delayed_payment_1.clone(),),
        Ok(false)
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
                amount: 1000,
            },
        ),
        Ok(true)
    );
    assert_eq!(test.token.balance(&test.payment_receiver), 1000);

    let delayed_payment_2 = Payment {
        receiver: test.payment_receiver.clone(),
        token: test.token_id_2.clone(),
        amount: 2000,
    };
    assert_eq!(
        test.pay(&[&test.wallet_admins[1]], 222, delayed_payment_2.clone()),
        Ok(false)
    );

    assert_eq!(
        test.pay(&[&test.wallet_admins[2]], 111, delayed_payment_1.clone()),
        Ok(false)
    );
    test.add_wallet_balance(&test.token_2, 2000);
    assert_eq!(
        test.pay(
            &[&test.wallet_admins[0], &test.wallet_admins[2]],
            222,
            delayed_payment_2
        ),
        Ok(true)
    );
    assert_eq!(test.token_2.balance(&test.payment_receiver), 2000);

    test.add_wallet_balance(&test.token, 500);
    assert_eq!(
        test.pay(&[&test.wallet_admins[1]], 111, delayed_payment_1),
        Ok(true)
    );

    assert_eq!(test.token.balance(&test.payment_receiver), 1500);
}

#[test]
fn test_double_initialization_returns_error() {
    let test = WalletTest::setup();
    test.initialize([30, 30, 30], 50).unwrap();
    assert_eq!(
        test.initialize([30, 30, 30], 50),
        Err(Error::AlreadyInitialized)
    );
}

#[test]
fn test_invalid_threshold_values_return_error() {
    let test = WalletTest::setup();
    // 0 threshold
    assert_eq!(
        test.initialize([30, 30, 30], 0),
        Err(Error::InvalidThreshold)
    );
    // Threshold is higher than the sum of admin weights
    assert_eq!(
        test.initialize([1, 2, 3], 7),
        Err(Error::AdminWeightsBelowThreshold)
    );
    // Threshold is too high
    assert_eq!(
        test.initialize([1000, 1000, 1000], 2001),
        Err(Error::InvalidThreshold)
    );
}

#[test]
fn test_invalid_admin_weights_return_error() {
    let test = WalletTest::setup();
    // 0 weight
    assert_eq!(
        test.initialize([1, 0, 3], 1),
        Err(Error::InvalidAdminWeight)
    );
    // Too high weight
    assert_eq!(
        test.initialize([1, 2, 101], 1),
        Err(Error::InvalidAdminWeight)
    );
    // Large values
    assert_eq!(
        test.initialize([u32::MAX, u32::MAX, u32::MAX], 1),
        Err(Error::InvalidAdminWeight)
    );
}

#[test]
fn test_invalid_admin_count_returns_error() {
    let test = WalletTest::setup();
    let mut admins = vec![&test.env];

    assert_eq!(
        extract_error(test.contract.try_initialize(&admins, &10)),
        Err(Error::InvalidAdminCount)
    );

    for _ in 0..21 {
        admins.push_back(Admin {
            id: test.signer_to_id(&generate_keypair()),
            weight: 5,
        });
    }

    assert_eq!(
        extract_error(test.contract.try_initialize(&admins, &10)),
        Err(Error::InvalidAdminCount)
    );
}

#[test]
fn test_unauthorized_signer_returns_error() {
    let test = WalletTest::setup();
    test.initialize([2, 2, 2], 2).unwrap();
    let non_wallet_admin = generate_keypair();
    assert_eq!(
        test.pay(
            &[&test.wallet_admins[1], &non_wallet_admin],
            222,
            Payment {
                receiver: test.payment_receiver.clone(),
                token: test.token_id.clone(),
                amount: 300,
            },
        ),
        Err(Error::UnauthorizedSigner)
    );
}

#[test]
fn test_divergent_delayed_payment_returns_error() {
    let test = WalletTest::setup();
    test.initialize([2, 2, 2], 4).unwrap();
    test.add_wallet_balance(&test.token, 1000);

    assert_eq!(
        test.pay(
            &[&test.wallet_admins[1]],
            222,
            Payment {
                receiver: test.payment_receiver.clone(),
                token: test.token_id.clone(),
                amount: 300,
            },
        ),
        Ok(false)
    );

    assert_eq!(
        test.pay(
            &[&test.wallet_admins[0]],
            222,
            Payment {
                receiver: test.payment_receiver.clone(),
                token: test.token_id.clone(),
                amount: 299,
            },
        ),
        Err(Error::StoredPaymentMismatch)
    );
}

#[test]
fn test_payment_reexecution() {
    let test = WalletTest::setup();
    test.initialize([2, 2, 2], 2).unwrap();
    let payment = Payment {
        receiver: test.payment_receiver.clone(),
        token: test.token_id.clone(),
        amount: 300,
    };
    test.add_wallet_balance(&test.token, 1000);

    assert_eq!(
        test.pay(&[&test.wallet_admins[1]], 222, payment.clone()),
        Ok(true)
    );

    assert_eq!(
        test.pay(&[&test.wallet_admins[0]], 222, payment),
        Err(Error::PaymentAlreadyExecuted)
    );
}

#[test]
fn test_duplicate_signers() {
    let test = WalletTest::setup();
    test.initialize([2, 2, 2], 6).unwrap();
    let payment = Payment {
        receiver: test.payment_receiver.clone(),
        token: test.token_id.clone(),
        amount: 300,
    };
    test.add_wallet_balance(&test.token, 1000);
    assert_eq!(
        test.pay(
            &[&test.wallet_admins[0], &test.wallet_admins[1]],
            222,
            payment.clone()
        ),
        Ok(false)
    );

    assert_eq!(
        test.pay(
            &[&test.wallet_admins[2], &test.wallet_admins[0]],
            222,
            payment,
        ),
        Err(Error::DuplicateSigner)
    );
}
