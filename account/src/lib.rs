// A multi-sig account that allows spending only up to provided amount of token
// unless all the signers have signed the invocation.
#![no_std]

use soroban_account::AuthorizationContext;
use soroban_sdk::{
    contracterror, contractimpl, contracttype, symbol, Account, BytesN, Env, IntoVal, Map, Symbol,
    TryIntoVal, Vec,
};

struct AccountContract;

#[contracttype]
#[derive(Clone)]
pub struct Signature {
    pub public_key: BytesN<32>,
    pub signature: BytesN<64>,
}

#[contracttype]
#[derive(Clone)]
enum DataKey {
    SignerCnt,
    Signer(BytesN<32>),
    SpendLimit(BytesN<32>),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AccError {
    NotEnoughSigners = 1,
    NegativeAmount = 2,
}

const XFER_FN: Symbol = symbol!("xfer");
const APPROVE_FN: Symbol = symbol!("approve");

#[contractimpl]
impl AccountContract {
    pub fn init(env: Env, signers: Vec<BytesN<32>>) {
        // In reality this would need some validation on signers.
        for signer in signers.iter() {
            env.data().set(DataKey::Signer(signer.unwrap()), ());
        }
        env.data().set(DataKey::SignerCnt, signers.len());
    }

    pub fn add_limit(env: Env, account: Account, token: BytesN<32>, limit: i128) {
        if account.address() != env.current_contract_account().address() {
            panic!("incorrect account");
        }
        account.authorize((token.clone(), limit).into_val(&env));
        env.data().set(DataKey::SpendLimit(token), limit);
    }

    pub fn check_auth(
        env: Env,
        signature_payload: BytesN<32>,
        signatures: Vec<Signature>,
        auth_context: Vec<AuthorizationContext>,
    ) -> Result<(), AccError> {
        // Perform authentication
        for i in 0..signatures.len() {
            let signature = signatures.get_unchecked(i).unwrap();
            if i > 0 {
                let prev_signature = signatures.get_unchecked(i - 1).unwrap();
                if prev_signature.public_key >= signature.public_key {
                    panic!("bad signature order");
                }
            }
            if !env
                .data()
                .has(DataKey::Signer(signature.public_key.clone()))
            {
                panic!("not a signer");
            }
            env.verify_sig_ed25519(
                &signature.public_key,
                &signature_payload.clone().into(),
                &signature.signature,
            );
        }

        let tot_signers: u32 = env.data().get(DataKey::SignerCnt).unwrap().unwrap();
        let all_signed = tot_signers == signatures.len();

        let curr_contract_id = env.current_contract_id();

        let mut spend_left_per_token = Map::<BytesN<32>, i128>::new(&env);
        // Apply authorization policy
        for context in auth_context.iter() {
            let context = context.unwrap();
            // For the account control every signer must sign the invocation.
            if context.contract == curr_contract_id {
                if !all_signed {
                    return Err(AccError::NotEnoughSigners);
                }
            }

            // Otherwise, we're only interested in functions that spend tokens.
            if context.fn_name != XFER_FN && context.fn_name != APPROVE_FN {
                continue;
            }

            let spend_left: Option<i128> =
                if let Some(spend_left) = spend_left_per_token.get(context.contract.clone()) {
                    Some(spend_left.unwrap())
                } else if let Some(limit_left) = env
                    .data()
                    .get(DataKey::SpendLimit(context.contract.clone()))
                {
                    Some(limit_left.unwrap())
                } else {
                    None
                };
            // 'None' means that the contract is outside of the policy.
            if let Some(spend_left) = spend_left {
                // 'amount' is the third argument in both `approve` and `xfer`.
                // If the contract has a different signature, it's safer to panic
                // here, as it's expected to have the standard interface.
                // let spent = i128::try_from_val(&env, context.args.get(2).unwrap());
                let spent: i128 = context
                    .args
                    .get(2)
                    .unwrap()
                    .unwrap()
                    .try_into_val(&env)
                    .unwrap();
                if spent < 0 {
                    return Err(AccError::NegativeAmount);
                }
                if !all_signed && spent > spend_left {
                    return Err(AccError::NotEnoughSigners);
                }
                spend_left_per_token.set(context.contract.clone(), spend_left - spent);
            }
        }
        Ok(())
    }
}

mod test;
