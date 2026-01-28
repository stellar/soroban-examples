#![no_std]

use soroban_poseidon::{poseidon_hash, PoseidonSponge};

use soroban_sdk::{
    crypto::bls12_381::Fr as BlsScalar, symbol_short, vec, BytesN, Env, Map, Symbol, Vec, U256,
};

/// Storage keys for the LeanIMT
pub const TREE_ROOT_KEY: Symbol = symbol_short!("root");
pub const TREE_DEPTH_KEY: Symbol = symbol_short!("depth");
pub const TREE_LEAVES_KEY: Symbol = symbol_short!("leaves");

/// Converts u64 to BlsScalar for test compatibility
pub fn u64_to_bls_scalar(env: &Env, value: u64) -> BlsScalar {
    BlsScalar::from_u256(U256::from_u32(env, value as u32))
}

/// Converts BlsScalar to BytesN<32> for Soroban storage
pub fn bls_scalar_to_bytes(scalar: BlsScalar) -> BytesN<32> {
    scalar.to_bytes()
}

/// Converts BytesN<32> to BlsScalar for computation
pub fn bytes_to_bls_scalar(bytes_n: &BytesN<32>) -> BlsScalar {
    BlsScalar::from_bytes(bytes_n.clone())
}

/// Lean Incremental Merkle Tree implementation with hybrid approach:
/// - Internal computation uses BlsScalar for perfect Circom compatibility
/// - Storage and API uses BytesN<32> for Soroban compatibility
pub struct LeanIMT {
    env: Env,
    leaves: Vec<BytesN<32>>,
    depth: u32,
    capacity: u32, // Pre-computed capacity (2^depth), cached for efficiency
    root: BytesN<32>,
    // Hybrid cache system:
    // 1. subtree_cache: Dynamic programming cache for empty tree levels
    //    Key: level -> Value: hash of subtrees at that level (all identical for empty trees)
    // 2. sparse_cache: Sparse storage for nodes updated due to leaf insertions
    //    Key: (level, node_index) -> Value: computed hash for specific nodes
    subtree_cache: Map<u32, BlsScalar>,
    sparse_cache: Map<(u32, u32), BlsScalar>,
}

impl LeanIMT {
    /// Creates a new LeanIMT with a fixed depth. Missing leaves are assumed zero.
    pub fn new(env: &Env, depth: u32) -> Self {
        let capacity = 1u32.checked_shl(depth).unwrap_or(u32::MAX);
        let env_clone = env.clone();
        let mut tree = Self {
            env: env_clone.clone(),
            leaves: vec![&env_clone],
            depth,
            capacity,
            root: BytesN::from_array(&env_clone, &[0u8; 32]),
            subtree_cache: Map::new(&env_clone),
            sparse_cache: Map::new(&env_clone),
        };
        tree.recompute_tree();
        tree
    }

    /// Inserts a new leaf into the tree (appends; missing leaves remain zero)
    /// Uses incremental path recomputation for efficiency (Clever shortcut 2)
    /// Returns Err if the tree is at capacity (2^depth leaves)
    pub fn insert(&mut self, leaf: BytesN<32>) -> Result<(), &'static str> {
        let current_count = self.leaves.len() as u32;

        if current_count >= self.capacity {
            return Err("Tree is at capacity: cannot insert more leaves");
        }

        self.leaves.push_back(leaf);
        self.incremental_update();
        Ok(())
    }

    /// Inserts a u64 leaf (converts to BlsScalar internally)
    pub fn insert_u64(&mut self, leaf_value: u64) -> Result<(), &'static str> {
        let leaf_scalar = u64_to_bls_scalar(&self.env, leaf_value);
        let leaf_bytes = bls_scalar_to_bytes(leaf_scalar);
        self.insert(leaf_bytes)
    }

    /// Gets the current root of the tree
    pub fn get_root(&self) -> BytesN<32> {
        self.root.clone()
    }

    /// Gets the current root as BlsScalar (for computation)
    pub fn get_root_scalar(&self) -> BlsScalar {
        bytes_to_bls_scalar(&self.root)
    }

    /// Gets the current depth of the tree
    pub fn get_depth(&self) -> u32 {
        self.depth
    }

    /// Gets the number of leaves that have been explicitly inserted
    pub fn get_leaf_count(&self) -> u32 {
        self.leaves.len() as u32
    }

    /// Gets the maximum capacity of the tree (2^depth)
    pub fn get_capacity(&self) -> u32 {
        self.capacity
    }

    /// Checks if the tree is at capacity
    pub fn is_full(&self) -> bool {
        self.get_leaf_count() >= self.get_capacity()
    }

    /// Generates a merkle proof for a given leaf index
    pub fn generate_proof(&self, leaf_index: u32) -> Option<(Vec<BlsScalar>, u32)> {
        if leaf_index >= self.leaves.len() as u32 {
            return None;
        }

        let mut siblings = vec![&self.env];

        // Handle the simple 2-leaf case correctly
        if self.depth == 1 && self.leaves.len() == 2 {
            if leaf_index == 0 {
                let sibling_bytes = self.leaves.get(1).unwrap();
                siblings.push_back(bytes_to_bls_scalar(&sibling_bytes));
            } else {
                let sibling_bytes = self.leaves.get(0).unwrap();
                siblings.push_back(bytes_to_bls_scalar(&sibling_bytes));
            }
        } else {
            // General approach
            let mut current_index = leaf_index;
            let mut current_depth = 0;

            while current_depth < self.depth {
                let sibling_index = if current_index % 2 == 0 {
                    current_index + 1
                } else {
                    current_index - 1
                };

                let sibling_scalar = if current_depth == 0 {
                    // At leaf level, use actual leaves or zero if missing
                    if sibling_index < self.leaves.len() as u32 {
                        let sibling_bytes = self.leaves.get(sibling_index).unwrap();
                        bytes_to_bls_scalar(&sibling_bytes)
                    } else {
                        BlsScalar::from_u256(U256::from_u32(&self.env, 0))
                    }
                } else {
                    // At internal levels, compute the actual node value
                    self.compute_node_at_level_scalar(sibling_index, current_depth)
                };

                siblings.push_back(sibling_scalar);
                current_index = current_index / 2;
                current_depth += 1;
            }
        }

        Some((siblings, self.depth))
    }

    /// Computes the value of an internal node at a specific level
    fn compute_node_at_level(&self, node_index: u32, target_level: u32) -> BytesN<32> {
        let result_scalar = self.compute_node_at_level_scalar(node_index, target_level);
        bls_scalar_to_bytes(result_scalar)
    }

    /// Computes the value of an internal node at a specific level in BlsScalar space
    /// Now uses memoization cache for efficiency
    fn compute_node_at_level_scalar(&self, node_index: u32, target_level: u32) -> BlsScalar {
        if target_level > self.depth {
            return BlsScalar::from_u256(U256::from_u32(&self.env, 0));
        }

        // Check if we have this node cached using hybrid cache system
        if let Some(cached_value) = self.get_cached_node(target_level, node_index) {
            return cached_value;
        }

        // If not cached, compute it
        if target_level == 0 {
            if node_index < self.leaves.len() as u32 {
                let leaf_bytes = self.leaves.get(node_index).unwrap();
                bytes_to_bls_scalar(&leaf_bytes)
            } else {
                BlsScalar::from_u256(U256::from_u32(&self.env, 0))
            }
        } else {
            // For levels > 0, compute by hashing the two children from the level below
            let left_child_index = node_index * 2;
            let right_child_index = left_child_index + 1;

            let left_scalar = self.compute_node_at_level_scalar(left_child_index, target_level - 1);
            let right_scalar =
                self.compute_node_at_level_scalar(right_child_index, target_level - 1);

            self.hash_pair(left_scalar, right_scalar)
        }
    }

    /// Incremental update using path recomputation (Clever shortcut 2)
    /// Only recomputes the path from the new leaf to the root
    ///
    /// This implements the optimization described in Tornado Cash:
    /// "all subtrees to the left of the newest member consist of subtrees
    /// whose roots can be cached rather than recalculated"
    ///
    /// Now with full memoization - we only recompute the specific path from the new leaf to root,
    /// and update the cache as we go.
    fn incremental_update(&mut self) {
        let leaf_index = (self.leaves.len() - 1) as u32;

        // Update the leaf in the sparse cache
        let leaf_bytes = self.leaves.get(leaf_index).unwrap();
        let leaf_scalar = bytes_to_bls_scalar(&leaf_bytes);
        self.cache_sparse_node(0, leaf_index, leaf_scalar);

        // Recompute the path to root and update cache
        self.root = self.recompute_path_to_root_with_cache_update(leaf_index);
    }

    /// Recomputes only the path from a specific leaf to the root with cache updates
    /// This is the optimized version that updates the cache as it goes
    fn recompute_path_to_root_with_cache_update(&mut self, leaf_index: u32) -> BytesN<32> {
        let leaf_bytes = self.leaves.get(leaf_index).unwrap();
        let leaf_scalar = bytes_to_bls_scalar(&leaf_bytes);

        // Create sponge once for efficient repeated hashing
        let mut sponge = PoseidonSponge::<3, BlsScalar>::new(&self.env);

        // Start from the leaf and work our way up to the root
        let mut current_index = leaf_index;
        let mut current_level = 0;
        let mut current_scalar = leaf_scalar;

        while current_level < self.depth {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            // Get the sibling value (either from cache or compute if missing)
            let sibling_scalar = if current_level == 0 {
                // At leaf level, use actual leaves or zero if missing
                if sibling_index < self.leaves.len() as u32 {
                    let sibling_bytes = self.leaves.get(sibling_index).unwrap();
                    bytes_to_bls_scalar(&sibling_bytes)
                } else {
                    BlsScalar::from_u256(U256::from_u32(&self.env, 0))
                }
            } else {
                // At internal levels, use hybrid cache system
                if let Some(cached_value) = self.get_cached_node(current_level, sibling_index) {
                    cached_value
                } else {
                    self.compute_node_at_level_scalar(sibling_index, current_level)
                }
            };

            // Compute the parent hash (reuse sponge for efficiency)
            let parent_scalar = if current_index % 2 == 0 {
                self.hash_pair_with_sponge(&mut sponge, current_scalar, sibling_scalar)
            } else {
                self.hash_pair_with_sponge(&mut sponge, sibling_scalar, current_scalar)
            };

            // Cache the parent hash in sparse cache (specific node update)
            let parent_index = current_index / 2;
            let parent_level = current_level + 1;
            self.cache_sparse_node(parent_level, parent_index, parent_scalar.clone());

            // Move up to the parent level
            current_index = current_index / 2;
            current_level = parent_level;
            current_scalar = parent_scalar;
        }

        // Return the root
        bls_scalar_to_bytes(current_scalar)
    }

    /// Gets a cached subtree hash for a level if it exists
    fn get_cached_subtree_level(&self, level: u32) -> Option<BlsScalar> {
        self.subtree_cache.get(level)
    }

    /// Caches a computed subtree hash for a level
    fn cache_subtree_level(&mut self, level: u32, hash: BlsScalar) {
        self.subtree_cache.set(level, hash);
    }

    /// Gets a cached node value using hybrid cache system:
    /// 1. First check sparse_cache for specific node updates
    /// 2. If not found, fall back to subtree_cache for level-based cache
    fn get_cached_node(&self, level: u32, node_index: u32) -> Option<BlsScalar> {
        // First check sparse cache for specific node updates
        if let Some(cached_value) = self.sparse_cache.get((level, node_index)) {
            return Some(cached_value);
        }

        // Fall back to subtree cache for level-based cache (empty tree optimization)
        self.get_cached_subtree_level(level)
    }

    /// Caches a specific node in the sparse cache (for incremental updates)
    fn cache_sparse_node(&mut self, level: u32, node_index: u32, hash: BlsScalar) {
        self.sparse_cache.set((level, node_index), hash);
    }

    /// Rebuilds the cache from the current leaves
    /// This is used when deserializing from storage
    fn rebuild_cache_from_leaves(&mut self) {
        if self.leaves.is_empty() {
            // For empty trees, use the optimized empty tree construction
            self.recompute_tree();
            return;
        }

        // For trees with leaves, clear both caches and let them rebuild on-demand
        // The hybrid cache system will handle the rest
        self.subtree_cache = Map::new(&self.env);
        self.sparse_cache = Map::new(&self.env);
    }

    /// Recomputes the entire tree after insertion using fixed depth and zero padding
    /// Optimized for empty trees: O(depth) instead of O(2^depth) using dynamic programming
    fn recompute_tree(&mut self) {
        if self.depth == 0 {
            // Special case: depth 0 tree with no leaves
            self.root = BytesN::from_array(&self.env, &[0u8; 32]);
            return;
        }

        // Create sponge once for efficient repeated hashing
        let mut sponge = PoseidonSponge::<3, BlsScalar>::new(&self.env);

        // For empty trees, all subtrees at the same level are identical
        // We only need to compute one hash per level: hash(level_n, level_n) = level_n+1
        let zero_scalar = BlsScalar::from_u256(U256::from_u32(&self.env, 0));
        let mut current_level_hash = zero_scalar.clone();

        // Compute hashes level by level, reusing the same hash for all nodes at each level
        for level in 0..=self.depth {
            if level == 0 {
                // Level 0: all leaves are zero
                current_level_hash = zero_scalar.clone();
            } else {
                // Level 1+: hash the previous level with itself (reuse sponge for efficiency)
                current_level_hash = self.hash_pair_with_sponge(
                    &mut sponge,
                    current_level_hash.clone(),
                    current_level_hash,
                );
            }

            // Cache this hash for the level (all nodes at this level are identical)
            self.cache_subtree_level(level, current_level_hash.clone());
        }

        // Set the root
        self.root = bls_scalar_to_bytes(current_level_hash);
    }

    /// Hashes two BlsScalar values using Poseidon hash function
    fn hash_pair(&self, left: BlsScalar, right: BlsScalar) -> BlsScalar {
        let left_u256 = BlsScalar::to_u256(&left);
        let right_u256 = BlsScalar::to_u256(&right);
        let inputs = Vec::from_array(&self.env, [left_u256, right_u256]);
        // Use poseidon_hash (not poseidon2_hash) to match circom circuit
        let result_u256 = poseidon_hash::<3, BlsScalar>(&self.env, &inputs);
        BlsScalar::from_u256(result_u256)
    }

    /// Hashes two BlsScalar values using a pre-initialized sponge for efficiency
    /// Use this in loops where many hashes are computed
    fn hash_pair_with_sponge(
        &self,
        sponge: &mut PoseidonSponge<3, BlsScalar>,
        left: BlsScalar,
        right: BlsScalar,
    ) -> BlsScalar {
        let left_u256 = BlsScalar::to_u256(&left);
        let right_u256 = BlsScalar::to_u256(&right);
        let inputs = Vec::from_array(&self.env, [left_u256, right_u256]);
        let result_u256 = sponge.compute_hash(&inputs);
        BlsScalar::from_u256(result_u256)
    }

    /// Serializes the tree state for storage
    pub fn to_storage(&self) -> (Vec<BytesN<32>>, u32, BytesN<32>) {
        (self.leaves.clone(), self.depth, self.root.clone())
    }

    /// Deserializes the tree state from storage
    pub fn from_storage(env: &Env, leaves: Vec<BytesN<32>>, depth: u32, root: BytesN<32>) -> Self {
        let capacity = 1u32.checked_shl(depth).unwrap_or(u32::MAX);
        let env_clone = env.clone();
        let mut tree = Self {
            env: env_clone.clone(),
            leaves,
            depth,
            capacity,
            root,
            subtree_cache: Map::new(&env_clone),
            sparse_cache: Map::new(&env_clone),
        };

        // Rebuild the cache for the deserialized tree
        tree.rebuild_cache_from_leaves();
        tree
    }

    /// Gets all leaves in the tree
    pub fn get_leaves(&self) -> &Vec<BytesN<32>> {
        &self.leaves
    }

    /// Checks if the tree is empty
    pub fn is_empty(&self) -> bool {
        self.leaves.is_empty()
    }

    /// Gets a leaf at a specific index
    pub fn get_leaf(&self, index: usize) -> Option<BytesN<32>> {
        match self.leaves.get(index.try_into().unwrap()) {
            Some(leaf) => Some(leaf.clone()),
            None => None,
        }
    }

    /// Gets a leaf as BlsScalar at a specific index
    pub fn get_leaf_scalar(&self, index: usize) -> Option<BlsScalar> {
        self.get_leaf(index)
            .map(|leaf_bytes| bytes_to_bls_scalar(&leaf_bytes))
    }

    /// Gets the value of a node at a specific level and index
    pub fn get_node(&self, level: u32, index: u32) -> Option<BytesN<32>> {
        if level == 0 {
            if index < self.leaves.len() as u32 {
                Some(self.leaves.get(index).unwrap())
            } else {
                None
            }
        } else if level > self.depth {
            None
        } else {
            Some(self.compute_node_at_level(index, level))
        }
    }

    /// Gets the sibling of a node at a specific level and index
    pub fn get_sibling(&self, level: u32, index: u32) -> Option<BytesN<32>> {
        if level > self.depth {
            return None;
        }

        if level == self.depth {
            return None;
        }

        let sibling_index = if index % 2 == 0 { index + 1 } else { index - 1 };

        self.get_node(level, sibling_index)
    }

    /// Demonstrates the "Clever shortcut 2" optimization concept
    /// Shows which subtrees would be reused vs recomputed for a new leaf
    ///
    /// This method analyzes the path from a new leaf to the root and identifies
    /// which sibling subtrees could be cached (left of current position) vs
    /// which need to be computed (right of current position).
    pub fn analyze_optimization_path(&self, new_leaf_index: u32) -> Vec<(u32, u32, bool)> {
        let mut path_analysis = vec![&self.env];
        let mut current_index = new_leaf_index;
        let mut current_level = 0;

        while current_level < self.depth {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            // Determine if this sibling subtree would be cached (left of current position)
            // In the true "Clever shortcut 2", subtrees to the left are cached
            let is_cached = sibling_index < current_index;

            path_analysis.push_back((current_level, sibling_index, is_cached));

            current_index = current_index / 2;
            current_level += 1;
        }

        path_analysis
    }
}

#[cfg(test)]
mod tests;
