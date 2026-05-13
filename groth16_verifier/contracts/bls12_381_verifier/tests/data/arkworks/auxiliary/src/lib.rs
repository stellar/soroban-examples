use ark_crypto_primitives::crh::{CRHScheme, TwoToOneCRHScheme};
use ark_crypto_primitives::merkle_tree::{ByteDigestConverter, Config, MerkleTree, Path};

pub mod circuit;
pub mod common;

use common::*;

/// Merkle tree configuration for byte-slice leaves.
///
/// This Merkle tree uses:
/// - `LeafHash` to hash raw leaves (`&[u8]`)
/// - `TwoToOneHash` to hash pairs of child digests for internal nodes
#[derive(Clone)]
pub struct MerkleConfig;

impl Config for MerkleConfig {
    /// Hash function used for leaf hashing.
    type LeafHash = LeafHash;
    /// Hash function used for hashing pairs of child digests at internal nodes.
    type TwoToOneHash = TwoToOneHash;

    /// A leaf is a raw byte slice. Ark's Merkle tree APIs operate on `&Self::Leaf`,
    /// so this becomes `&[u8]` in practice.
    type Leaf = [u8];

    /// Digest type produced by `LeafHash`.
    type LeafDigest = <LeafHash as CRHScheme>::Output;
    /// Digest type produced by `TwoToOneHash`.
    type InnerDigest = <TwoToOneHash as TwoToOneCRHScheme>::Output;

    /// Converts a leaf digest into bytes so it can be used as input to the two-to-one hash
    /// when building/verifying inner nodes.
    type LeafInnerDigestConverter = ByteDigestConverter<Self::LeafDigest>;
}

/// A simple Merkle tree over raw byte-slice leaves.
pub type SimpleMerkleTree = MerkleTree<MerkleConfig>;
/// The root digest of the Merkle tree (an output of the two-to-one hash).
pub type Root = <TwoToOneHash as TwoToOneCRHScheme>::Output;
/// A Merkle membership proof for a leaf.
pub type SimplePath = Path<MerkleConfig>;
