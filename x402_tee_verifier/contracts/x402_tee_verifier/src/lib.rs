#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short,
    token, Address, BytesN, Env,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    ReceiptAlreadySpent = 3,
    InvalidEnclaveSignature = 4,
    InvalidAmount = 5,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Spent(BytesN<32>),
}

#[contract]
pub struct X402TeeVerifierContract;

#[contractimpl]
impl X402TeeVerifierContract {
    /// Initializes the verifier with the accepted settlement token.
    pub fn initialize(env: Env, admin: Address, token: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        Ok(())
    }

    /// Settles an x402 hardware-attested execution receipt.
    pub fn settle_execution(
        env: Env,
        evidence_ref: BytesN<32>,
        fingerprint: BytesN<32>,
        payer: Address,
        recipient: Address,
        amount: i128,
        enclave_pubkey: BytesN<32>,
        signature: BytesN<64>,
    ) -> Result<bool, Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        // 1. Payer must authorize the payment invocation
        payer.require_auth();

        // 2. Prevent double-spending of the receipt fingerprint
        let spent_key = DataKey::Spent(fingerprint.clone());
        if env.storage().persistent().has(&spent_key) {
            return Err(Error::ReceiptAlreadySpent);
        }

        // 3. Verify hardware enclave Ed25519 signature over the receipt fingerprint
        env.crypto().ed25519_verify(
            &enclave_pubkey,
            &fingerprint.clone().into(),
            &signature,
        );

        // 4. Mark receipt as permanently spent and extend storage TTL
        env.storage().persistent().set(&spent_key, &true);
        env.storage().persistent().extend_ttl(&spent_key, 100_000, 500_000);

        // 5. Transfer settlement token (Circle USDC or XLM)
        let token_addr: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(Error::NotInitialized)?;
        
        let client = token::Client::new(&env, &token_addr);
        client.transfer(&payer, &recipient, &amount);

        // 6. Emit verifiable on-chain event
        env.events().publish(
            (symbol_short!("x402_pay"), evidence_ref),
            (payer, recipient, amount),
        );

        Ok(true)
    }

    /// Checks if a receipt fingerprint has already been settled.
    pub fn is_spent(env: Env, fingerprint: BytesN<32>) -> bool {
        env.storage().persistent().has(&DataKey::Spent(fingerprint))
    }
}
