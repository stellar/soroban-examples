#![no_std]

extern crate alloc;

use soroban_sdk::{
    contract, contractimpl, 
    vec, Env, String, Vec, Address, symbol_short, Symbol, Bytes, BytesN,
    token, log,
};

#[cfg(feature = "test_hash")]
use soroban_sdk::{U256, crypto::bls12_381::Fr as BlsScalar};

use zk::{Groth16Verifier, VerificationKey, Proof, PublicSignals};
use lean_imt::{LeanIMT, TREE_ROOT_KEY, TREE_DEPTH_KEY, TREE_LEAVES_KEY};
#[cfg(feature = "test_hash")]
use poseidon::Poseidon255;

#[cfg(test)]
mod test;

use soroban_sdk::contracterror;

// Contract errors
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NullifierUsed = 1,
    InsufficientBalance = 2,
    CoinOwnershipProofFailed = 3,
    OnlyAdmin = 4,
    TreeAtCapacity = 5,
    AssociationRootMismatch = 6,
}

// Error messages for Vec<String> returns (legacy compatibility)
pub const ERROR_NULLIFIER_USED: &str = "Nullifier already used";
pub const ERROR_INSUFFICIENT_BALANCE: &str = "Insufficient balance";
pub const ERROR_COIN_OWNERSHIP_PROOF: &str = "Couldn't verify coin ownership proof";
pub const ERROR_WITHDRAW_SUCCESS: &str = "Withdrawal successful";
pub const ERROR_ONLY_ADMIN: &str = "Only the admin can set association root";
pub const SUCCESS_ASSOCIATION_ROOT_SET: &str = "Association root set successfully";

const TREE_DEPTH: u32 = 2;

// Storage keys
const NULL_KEY: Symbol = symbol_short!("null");
const VK_KEY: Symbol = symbol_short!("vk");
const TOKEN_KEY: Symbol = symbol_short!("token");
const ASSOCIATION_ROOT_KEY: Symbol = symbol_short!("assoc");
const ADMIN_KEY: Symbol = symbol_short!("admin");

const FIXED_AMOUNT: i128 = 1000000000; // 1 XLM in stroops

#[contract]
pub struct PrivacyPoolsContract;

#[contractimpl]
impl PrivacyPoolsContract {
    pub fn __constructor(env: &Env, vk_bytes: Bytes, token_address: Address, admin: Address) {
        // Store the admin
        env.storage().instance().set(&ADMIN_KEY, &admin);
        
        env.storage().instance().set(&VK_KEY, &vk_bytes);
        env.storage().instance().set(&TOKEN_KEY, &token_address);
        
        // Initialize empty merkle tree with fixed depth
        let tree = LeanIMT::new(env, TREE_DEPTH);
        let (leaves, depth, root) = tree.to_storage();
        env.storage().instance().set(&TREE_LEAVES_KEY, &leaves);
        env.storage().instance().set(&TREE_DEPTH_KEY, &depth);
        env.storage().instance().set(&TREE_ROOT_KEY, &root);
    }

    /// Stores a commitment in the merkle tree and updates the tree state
    /// 
    /// # Arguments
    /// * `env` - The Soroban environment
    /// * `commitment` - The commitment to store
    /// 
    /// # Returns
    /// * A Result containing a tuple of (updated_merkle_root, leaf_index) after insertion
    fn store_commitment(env: &Env, commitment: BytesN<32>) -> Result<(BytesN<32>, u32), Error> {
        // Load current tree state
        let leaves: Vec<BytesN<32>> = env.storage().instance().get(&TREE_LEAVES_KEY)
            .unwrap_or(vec![&env]);
        let depth: u32 = env.storage().instance().get(&TREE_DEPTH_KEY)
            .unwrap_or(0);
        let root: BytesN<32> = env.storage().instance().get(&TREE_ROOT_KEY)
            .unwrap_or(BytesN::from_array(&env, &[0u8; 32]));
        
        // Create tree and insert new commitment
        let mut tree = LeanIMT::from_storage(env, leaves, depth, root);
        tree.insert(commitment).map_err(|_| Error::TreeAtCapacity)?;
        
        // Get the leaf index (it's the last leaf in the tree)
        let leaf_index = tree.get_leaf_count() - 1;
        
        // Store updated tree state
        let (new_leaves, new_depth, new_root) = tree.to_storage();
        env.storage().instance().set(&TREE_LEAVES_KEY, &new_leaves);
        env.storage().instance().set(&TREE_DEPTH_KEY, &new_depth);
        env.storage().instance().set(&TREE_ROOT_KEY, &new_root);

        Ok((new_root, leaf_index))
    }

    /// Deposits funds into the privacy pool and stores a commitment in the merkle tree.
    ///
    /// This function allows a user to deposit a fixed amount (1 XLM) of the configured token into the privacy pool
    /// while providing a cryptographic commitment that will be used for zero-knowledge proof
    /// verification during withdrawal.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `from` - The address of the depositor (must be authenticated)
    /// * `commitment` - A 32-byte cryptographic commitment that will be used to prove
    ///                 ownership during withdrawal without revealing the actual coin details
    ///
    /// # Returns
    ///
    /// * The leaf index where the commitment was stored in the merkle tree
    ///
    /// # Security
    ///
    /// * Requires authentication from the `from` address
    /// * The commitment is stored in a merkle tree for efficient inclusion proofs
    /// * Transfers exactly `FIXED_AMOUNT` of the configured token from the depositor to the contract
    ///
    /// # Storage
    ///
    /// * Updates the merkle tree with the new commitment
    /// * Transfers the asset from the depositor to the contract
    pub fn deposit(env: &Env, from: Address, commitment: BytesN<32>) -> Result<u32, Error> {
        from.require_auth();
        
        // Get the stored token address
        let token_address: Address = env.storage().instance().get(&TOKEN_KEY).unwrap();
        
        // Create token client and transfer from depositor to contract
        let token_client = token::Client::new(env, &token_address);
        token_client.transfer(&from, &env.current_contract_address(), &FIXED_AMOUNT);
        
        // Store the commitment in the merkle tree
        let (_, leaf_index) = Self::store_commitment(env, commitment)?;

        Ok(leaf_index)
    }

    /// Withdraws funds from the privacy pool using a zero-knowledge proof.
    ///
    /// This function allows a user to withdraw a fixed amount (1 XLM) of the configured token from the privacy pool
    /// by providing a cryptographic proof that demonstrates ownership of a previously deposited
    /// commitment without revealing which specific commitment it corresponds to.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `to` - The address of the recipient (must be authenticated)
    /// * `proof_bytes` - The serialized zero-knowledge proof demonstrating ownership of a
    ///                   commitment without revealing the commitment itself
    /// * `pub_signals_bytes` - The serialized public signals associated with the proof
    ///
    /// # Returns
    ///
    /// Returns a vector containing status messages:
    /// * Empty vector `[]` on successful withdrawal (success is logged as a diagnostic event)
    /// * `["Nullifier already used"]` if the nullifier has been used before
    /// * `["Couldn't verify coin ownership proof"]` if the zero-knowledge proof verification fails
    /// * `["Insufficient balance"]` if the contract doesn't have enough funds
    ///
    /// # Security
    ///
    /// * Requires authentication from the `to` address
    /// * Verifies that the nullifier hasn't been used before (prevents double-spending)
    /// * Validates the zero-knowledge proof using Groth16 verification
    /// * Transfers exactly `FIXED_AMOUNT` of the configured token from the contract to the recipient
    ///
    /// # Storage
    ///
    /// * Adds the nullifier to the used nullifiers list to prevent reuse
    /// * Transfers the asset from the contract to the recipient
    ///
    /// # Privacy
    ///
    /// * The withdrawal doesn't reveal which specific commitment is being spent
    /// * The nullifier ensures the same commitment cannot be spent twice
    /// * The zero-knowledge proof proves ownership without revealing the commitment details
    pub fn withdraw(env: &Env, 
            to: Address,
            proof_bytes: Bytes, 
            pub_signals_bytes: Bytes) -> Vec<String> {
        to.require_auth();

        // Require association root to be set before any withdrawal
        if !Self::has_association_set(env) {
            panic!("Association root must be set before withdrawal");
        }

        // Get the stored token address
        let token_address: Address = env.storage().instance().get(&TOKEN_KEY).unwrap();

        // Check contract balance before updating state
        let token_client = token::Client::new(env, &token_address);
        let contract_balance = token_client.balance(&env.current_contract_address());
        if contract_balance < FIXED_AMOUNT {
            return vec![env, String::from_str(env, ERROR_INSUFFICIENT_BALANCE)]
        }

        let vk_bytes: Bytes = env.storage().instance().get(&VK_KEY).unwrap();
        let vk = VerificationKey::from_bytes(env, &vk_bytes).unwrap();
        let proof = Proof::from_bytes(env, &proof_bytes);
        let pub_signals = PublicSignals::from_bytes(env, &pub_signals_bytes);

        // Extract public signals: [nullifierHash, withdrawnValue, stateRoot, associationRoot]
        let nullifier_hash = &pub_signals.pub_signals.get(0).unwrap();
        let _withdrawn_value = &pub_signals.pub_signals.get(1).unwrap();
        let proof_root = &pub_signals.pub_signals.get(2).unwrap();
        let proof_association_root = &pub_signals.pub_signals.get(3).unwrap();

        // Verify association set root matches the proof
        let stored_association_root = Self::get_association_root(env);
        let proof_association_root_bytes = proof_association_root.to_bytes();
        
        if stored_association_root != proof_association_root_bytes {
            return vec![env, String::from_str(env, "Association set root mismatch")]
        }

        // Check if nullifier has been used before
        let mut nullifiers: Vec<BytesN<32>> = env.storage().instance().get(&NULL_KEY)
            .unwrap_or(vec![env]);

        let nullifier = nullifier_hash.to_bytes();
        
        if nullifiers.contains(&nullifier) {
            return vec![env, String::from_str(env, ERROR_NULLIFIER_USED)]
        }
        
        // Verify state root matches
        let state_root: BytesN<32> = env.storage().instance().get(&TREE_ROOT_KEY)
            .unwrap_or(BytesN::from_array(&env, &[0u8; 32]));
        
        let proof_root_bytes = proof_root.to_bytes();
        
        if state_root != proof_root_bytes {
             return vec![env, String::from_str(env, ERROR_COIN_OWNERSHIP_PROOF)]
        }
        
        // Verify the zero-knowledge proof
        let res = Groth16Verifier::verify_proof(env, vk, proof, &pub_signals.pub_signals);
        if res.is_err() || !res.unwrap() {
            return vec![env, String::from_str(env, ERROR_COIN_OWNERSHIP_PROOF)]
        }

        // Add nullifier to used nullifiers only after all checks pass
        nullifiers.push_back(nullifier);
        env.storage().instance().set(&NULL_KEY, &nullifiers);

        // Transfer the asset from the contract to the recipient
        token_client.transfer(&env.current_contract_address(), &to, &FIXED_AMOUNT);

        // Log success message as diagnostic event
        log!(&env, "{}", ERROR_WITHDRAW_SUCCESS);
        
        vec![env]
    }

    /// Gets the current merkle root of the commitment tree
    pub fn get_merkle_root(env: &Env) -> BytesN<32> {
        env.storage().instance().get(&TREE_ROOT_KEY)
            .unwrap_or(BytesN::from_array(&env, &[0u8; 32]))
    }

    /// Gets the current depth of the merkle tree
    pub fn get_merkle_depth(env: &Env) -> u32 {
        env.storage().instance().get(&TREE_DEPTH_KEY)
            .unwrap_or(0)
    }

    /// Gets the number of commitments (leaves) in the merkle tree
    pub fn get_commitment_count(env: &Env) -> u32 {
        let leaves: Vec<BytesN<32>> = env.storage().instance().get(&TREE_LEAVES_KEY)
            .unwrap_or(vec![&env]);
        leaves.len() as u32
    }

    /// Gets all commitments (leaves) in the merkle tree
    pub fn get_commitments(env: &Env) -> Vec<BytesN<32>> {
        env.storage().instance().get(&TREE_LEAVES_KEY)
            .unwrap_or(vec![env])
    }

    pub fn get_nullifiers(env: &Env) -> Vec<BytesN<32>> {
        env.storage().instance().get(&NULL_KEY)
            .unwrap_or(vec![env])
    }

    /// Gets the balance of the configured token held by the contract
    pub fn get_balance(env: &Env) -> i128 {
        let token_address: Address = env.storage().instance().get(&TOKEN_KEY).unwrap();
        let token_client = token::Client::new(env, &token_address);
        token_client.balance(&env.current_contract_address())
    }

    /// Validates that the caller is the admin
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `caller` - The address to validate as admin
    ///
    /// # Returns
    ///
    /// * `true` if the caller is the admin, `false` otherwise
    fn is_admin(env: &Env, caller: &Address) -> bool {
        let admin: Address = env.storage().instance().get(&ADMIN_KEY).unwrap();
        *caller == admin
    }

    /// Sets the association set root for compliance verification
    ///
    /// This function allows the admin to update the association set root,
    /// which is used to verify that withdrawals are associated with approved
    /// subsets of deposits for compliance purposes.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `caller` - The address of the caller (must be authenticated and be the admin)
    /// * `association_root` - The new association set root (32-byte hash)
    ///
    /// # Returns
    ///
    /// Returns a vector containing status messages:
    /// * `["Association root set successfully"]` on successful update
    /// * `["Only the admin can set association root"]` if the caller is not the admin
    ///
    /// # Security
    ///
    /// * Requires authentication from the caller
    /// * Only the contract deployer (admin) can update association sets
    pub fn set_association_root(env: &Env, caller: Address, association_root: BytesN<32>) -> Vec<String> {
        caller.require_auth();
        
        // Verify that the caller is actually the admin
        if !Self::is_admin(env, &caller) {
            return vec![env, String::from_str(env, ERROR_ONLY_ADMIN)];
        }
        
        env.storage().instance().set(&ASSOCIATION_ROOT_KEY, &association_root);
        vec![env, String::from_str(env, SUCCESS_ASSOCIATION_ROOT_SET)]
    }

    /// Gets the current association set root
    ///
    /// # Returns
    ///
    /// * The current association set root, or zero bytes if not set
    pub fn get_association_root(env: &Env) -> BytesN<32> {
        env.storage().instance().get(&ASSOCIATION_ROOT_KEY)
            .unwrap_or(BytesN::from_array(&env, &[0u8; 32]))
    }

    /// Checks if an association set is currently configured
    ///
    /// # Returns
    ///
    /// * `true` if an association set root is configured, `false` otherwise
    pub fn has_association_set(env: &Env) -> bool {
        let association_root = Self::get_association_root(env);
        let zero_root = BytesN::from_array(&env, &[0u8; 32]);
        association_root != zero_root
    }

    /// Gets the admin address (the contract deployer)
    ///
    /// # Returns
    ///
    /// * The address of the admin (contract deployer)
    pub fn get_admin(env: &Env) -> Address {
        env.storage().instance().get(&ADMIN_KEY).unwrap()
    }

}

#[cfg(feature = "test_hash")]
#[contractimpl]
impl PrivacyPoolsContract {
    pub fn test_hash(env: &Env) ->  () {
        let poseidon = Poseidon255::new_with_t(env, 3);
        let zero = BlsScalar::from_u256(U256::from_u32(env, 0));
        poseidon.hash_two(&zero, &zero);
    }
}
