//! A Merkle Airdrop contract for distributing tokens based on a Merkle tree.
//!
//! This contract verifies Merkle proofs submitted by users to claim their airdrop.
//! Merkle proofs must be generated off-chain and provided to users claiming tokens.
#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, token, xdr::ToXdr, Address, BytesN, Env,
    Vec,
};

#[contracttype]
#[derive(Clone, Debug)]
enum DataKey {
    RootHash,
    TokenAddress,
    Claimed(u32),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyClaimed = 1,
    InvalidProof = 2,
}

#[contracttype]
#[derive(Clone, Debug)]
struct Receiver {
    pub index: u32,
    pub address: Address,
    pub amount: i128,
}

#[contract]
pub struct MerkleAirdropContract;

#[contractimpl]
impl MerkleAirdropContract {
    /// Constructor to initialize the Merkle Airdrop contract.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment.
    /// * `root_hash` - The root hash of the Merkle tree.
    /// * `token` - The address of the token to be airdropped.
    /// * `funding_amount` - The total amount of tokens to fund the contract with.
    /// * `funding_source` - The address that funds the contract.
    pub fn __constructor(
        env: Env,
        root_hash: BytesN<32>,
        token: Address,
        funding_amount: i128,
        funding_source: Address,
    ) {
        env.storage().instance().set(&DataKey::RootHash, &root_hash);
        env.storage().instance().set(&DataKey::TokenAddress, &token);

        // Transfer the funding amount from the funding source to this contract.
        token::TokenClient::new(&env, &token).transfer(
            &funding_source,
            env.current_contract_address(),
            &funding_amount,
        );
    }

    /// Claim tokens if the receiver is part of the Merkle tree defined by the root hash.
    ///
    /// # Arguments
    /// * `env` - The Soroban environment.
    /// * `index` - The index of the receiver in the original list.
    /// * `receiver` - The address of the receiver claiming the tokens.
    /// * `amount` - The amount of tokens the receiver is claiming.
    /// * `proof` - The Merkle proof (a list of sibling hashes)
    pub fn claim(
        env: Env,
        index: u32,
        receiver: Address,
        amount: i128,
        proof: Vec<BytesN<32>>,
    ) -> Result<(), Error> {
        // Check if this index has already been claimed.
        let key = DataKey::Claimed(index);
        if env.storage().instance().has(&key) {
            return Err(Error::AlreadyClaimed);
        }

        // Recompute the Merkle root from the leaf node and the provided proof.
        let node = Receiver {
            index,
            address: receiver.clone(),
            amount,
        };
        let mut hash = env.crypto().sha256(&node.to_xdr(&env));
        for p in proof {
            let a = hash.to_array();
            let b = p.to_array();

            let (left, right) = if a < b { (a, b) } else { (b, a) };
            let mut combined = [0u8; 64];
            combined[..32].copy_from_slice(&left);
            combined[32..].copy_from_slice(&right);
            hash = env
                .crypto()
                .sha256(&BytesN::from_array(&env, &combined).into());
        }

        // Verify that the recomputed root matches the stored root.
        let root_hash: BytesN<32> = env.storage().instance().get(&DataKey::RootHash).unwrap();
        if !root_hash.eq(&hash.to_bytes()) {
            return Err(Error::InvalidProof);
        }

        // Transfer the tokens to the receiver.
        let token: Address = env
            .storage()
            .instance()
            .get(&DataKey::TokenAddress)
            .unwrap();
        token::TokenClient::new(&env, &token).transfer(
            &env.current_contract_address(),
            &receiver,
            &amount,
        );

        // Mark this index as claimed.
        env.storage().instance().set(&key, &());

        Ok(())
    }
}

mod test;
