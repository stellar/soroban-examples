#![no_std]

mod auth;
mod pool_contract;
mod token_contract;

pub use crate::token_contract::{SaltedSignaturePayload, SaltedSignaturePayloadV0};
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{
    contractimpl, contracttype, symbol, BigInt, Bytes, BytesN, Env, IntoVal, Map, RawVal, Symbol,
    TryIntoVal, Vec,
};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Pool(BytesN<32>),
}

#[derive(Clone)]
#[contracttype]
pub struct Deposit {
    pub token: BytesN<32>,
    pub nonce: BigInt,
    pub desired: BigInt,
    pub min: BigInt,
}

fn pool_salt(e: &Env, token_a: &BytesN<32>, token_b: &BytesN<32>) -> BytesN<32> {
    if token_a >= token_b {
        panic!("token_a must be less t&han token_b");
    }

    let mut salt_bin = Bytes::new(&e);
    salt_bin.append(&token_a.clone().into());
    salt_bin.append(&token_b.clone().into());
    e.compute_hash_sha256(salt_bin)
}

fn get_pool_id(e: &Env, salt: &BytesN<32>) -> BytesN<32> {
    e.contract_data()
        .get_unchecked(DataKey::Pool(salt.clone()))
        .unwrap()
}

fn put_pool(e: &Env, salt: &BytesN<32>, pool: &BytesN<32>) {
    e.contract_data()
        .set(DataKey::Pool(salt.clone()), pool.clone())
}

fn has_pool(e: &Env, salt: &BytesN<32>) -> bool {
    e.contract_data().has(DataKey::Pool(salt.clone()))
}

fn get_deposit_amounts(
    desired_a: BigInt,
    min_a: BigInt,
    desired_b: BigInt,
    min_b: BigInt,
    reserves: (BigInt, BigInt),
) -> (BigInt, BigInt) {
    if reserves.0 == 0 && reserves.1 == 0 {
        return (desired_a, desired_b);
    }

    let amount_b = desired_a.clone() * reserves.1.clone() / reserves.0.clone();
    if amount_b <= desired_b {
        if amount_b < min_b {
            panic!("amount_b less than min")
        }
        return (desired_a, amount_b);
    } else {
        let amount_a = desired_b.clone() * reserves.0 / reserves.1;
        if amount_a > desired_a || desired_a < min_a {
            panic!("amount_a invalid")
        }
        return (amount_a, desired_b);
    }
}

pub trait LiquidityPoolRouterTrait {
    fn deposit(
        e: Env,
        to: Identifier,
        deposit_a: Deposit,
        deposit_b: Deposit,
        sigs: Map<Symbol, Signature>,
    );
}

pub struct LiquidityPoolRouter;

impl LiquidityPoolRouterTrait for LiquidityPoolRouter {
    fn deposit(
        e: Env,
        to: Identifier,
        deposit_a: Deposit,
        deposit_b: Deposit,
        sigs: Map<Symbol, Signature>,
    ) {
        crate::auth::verify(
            &e,
            sigs.get_unchecked(symbol!("deposit")).unwrap(),
            symbol!("deposit"),
            (&to, &deposit_a, &deposit_b),
            e.get_current_call_stack().into(),
        );

        let salt = pool_salt(&e, &deposit_a.token, &deposit_b.token);
        if !has_pool(&e, &salt) {
            let pool_id = pool_contract::create_contract(&e, &salt);
            put_pool(&e, &salt, &pool_id);
            pool_contract::Client::new(&e, &pool_id).initialize(&deposit_a.token, &deposit_b.token);
        }
        let pool_id = get_pool_id(&e, &salt);
        let pool_client = pool_contract::Client::new(&e, &pool_id);

        let reserves = pool_client.get_rsrvs();
        let amounts = get_deposit_amounts(
            deposit_a.desired,
            deposit_a.min,
            deposit_b.desired,
            deposit_b.min,
            reserves,
        );

        let mut sigs_a = Map::new(&e);
        sigs_a.set(
            symbol!("sig"),
            sigs.get_unchecked(symbol!("xfer_a")).unwrap(),
        );
        let mut sigs_b = Map::new(&e);
        sigs_b.set(
            symbol!("sig"),
            sigs.get_unchecked(symbol!("xfer_b")).unwrap(),
        );

        token_contract::Client::new(&e, deposit_a.token).xfer(
            &to,
            &deposit_a.nonce,
            &Identifier::Contract(e.get_current_contract()),
            &amounts.0,
            &sigs_a,
        );
        token_contract::Client::new(&e, deposit_b.token).xfer(
            &to,
            &deposit_b.nonce,
            &Identifier::Contract(e.get_current_contract()),
            &amounts.1,
            &sigs_b,
        );

        pool_client.deposit(&to);
    }
}

pub trait PayloadTrait {
    fn has_sig(e: Env, function: Symbol) -> bool;

    fn payload(
        e: Env,
        function: Symbol,
        args: Vec<RawVal>,
        callstack: Vec<(BytesN<32>, Symbol)>,
    ) -> Map<Symbol, (Identifier, SaltedSignaturePayload)>;
}

pub struct LiquidityPoolRouterPayload;

#[contractimpl]
impl PayloadTrait for LiquidityPoolRouterPayload {
    fn has_sig(_e: Env, function: Symbol) -> bool {
        const DEPOSIT_RAW: u64 = symbol!("deposit").to_raw().get_payload();
        match function.to_raw().get_payload() {
            DEPOSIT_RAW => true,
            _ => false,
        }
    }

    fn payload(
        e: Env,
        function: Symbol,
        args: Vec<RawVal>,
        callstack: Vec<(BytesN<32>, Symbol)>,
    ) -> Map<Symbol, (Identifier, SaltedSignaturePayload)> {
        const DEPOSIT_RAW: u64 = symbol!("deposit").to_raw().get_payload();
        match function.to_raw().get_payload() {
            DEPOSIT_RAW => {
                let to: Identifier = args.get_unchecked(0).unwrap().try_into_val(&e).unwrap();
                let deposit_a: Deposit = args.get_unchecked(1).unwrap().try_into_val(&e).unwrap();
                let deposit_b: Deposit = args.get_unchecked(2).unwrap().try_into_val(&e).unwrap();

                let pool_id = get_pool_id(&e, &pool_salt(&e, &deposit_a.token, &deposit_b.token));
                let reserves = pool_contract::Client::new(&e, &pool_id).get_rsrvs();
                let amounts = get_deposit_amounts(
                    deposit_a.desired,
                    deposit_a.min,
                    deposit_b.desired,
                    deposit_b.min,
                    reserves,
                );

                let mut callstack_a = callstack.clone();
                callstack_a.push_back((deposit_a.token.clone(), symbol!("xfer")));
                let to_forward_a = token_contract::Client::new(&e, &deposit_a.token)
                    .payload(
                        &symbol!("xfer"),
                        &(&to, deposit_a.nonce, e.get_current_contract(), amounts.0).into_val(&e),
                        &callstack_a,
                    )
                    .get_unchecked(symbol!("sig"))
                    .unwrap();

                let mut callstack_b = callstack.clone();
                callstack_b.push_back((deposit_b.token.clone(), symbol!("xfer")));
                let to_forward_b = token_contract::Client::new(&e, &deposit_b.token)
                    .payload(
                        &symbol!("xfer"),
                        &(&to, deposit_b.nonce, e.get_current_contract(), amounts.1).into_val(&e),
                        &callstack_b,
                    )
                    .get_unchecked(symbol!("sig"))
                    .unwrap();

                let to_verify = SaltedSignaturePayload::V0(SaltedSignaturePayloadV0 {
                    function,
                    contract: e.get_current_contract(),
                    network: e.ledger().network_passphrase(),
                    args,
                    salt: callstack.into(),
                });

                let mut res = Map::new(&e);
                res.set(symbol!("xfer_a"), to_forward_a);
                res.set(symbol!("xfer_b"), to_forward_b);
                res.set(symbol!("deposit"), (to, to_verify));
            }
            _ => panic!(),
        }
        todo!()
    }
}
