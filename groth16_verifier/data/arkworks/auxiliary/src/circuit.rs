use crate::common::*;
use crate::{MerkleConfig, Root, SimplePath};

use ark_crypto_primitives::crh::constraints::{CRHSchemeGadget, TwoToOneCRHSchemeGadget};
use ark_crypto_primitives::crh::{CRHScheme, TwoToOneCRHScheme};
use ark_crypto_primitives::merkle_tree::constraints::{ConfigGadget, PathVar};

use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

/// Merkle tree gadget config: binds hash gadgets + digest types to the MerkleConfig.
pub struct MerkleConfigGadget;

impl ConfigGadget<MerkleConfig, ConstraintF> for MerkleConfigGadget {
    /// Leaf in-circuit is a byte-slice (`&[u8]` off-circuit).
    type Leaf = [UInt8<ConstraintF>];

    /// Digest variables produced by the leaf hash gadget and the two-to-one hash gadget.
    type LeafDigest = <LeafHashGadget as CRHSchemeGadget<LeafHash, ConstraintF>>::OutputVar;
    type InnerDigest =
        <TwoToOneHashGadget as TwoToOneCRHSchemeGadget<TwoToOneHash, ConstraintF>>::OutputVar;

    /// Hash gadgets used by the Merkle path gadget.
    type LeafHash = LeafHashGadget;
    type TwoToOneHash = TwoToOneHashGadget;

    /// Converts leaf digest var into bytes for feeding into the two-to-one hash gadget.
    /// (Required for many arkworks versions when leaf and inner digests differ in representation.)
    type LeafInnerConverter =
        ark_crypto_primitives::merkle_tree::constraints::BytesVarDigestConverter<
            Self::LeafDigest,
            ConstraintF,
        >;
}

/// The R1CS equivalent of the Merkle tree root.
pub type RootVar =
    <TwoToOneHashGadget as TwoToOneCRHSchemeGadget<TwoToOneHash, ConstraintF>>::OutputVar;

/// The R1CS equivalent of the Merkle tree authentication path.
pub type SimplePathVar = PathVar<MerkleConfig, ConstraintF, MerkleConfigGadget>;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct MerkleTreeVerification {
    /// Public inputs
    pub root: Root,
    pub leaf: u8,

    /// Private witness
    pub authentication_path: Option<SimplePath>,
    /// Constants embedded into the circuit
    pub leaf_crh_params: Option<<LeafHash as CRHScheme>::Parameters>,
    pub two_to_one_crh_params: Option<<TwoToOneHash as TwoToOneCRHScheme>::Parameters>,
}

impl ConstraintSynthesizer<ConstraintF> for MerkleTreeVerification {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // --- Public inputs ---
        let root = RootVar::new_input(ark_relations::ns!(cs, "root_var"), || Ok(&self.root))?;
        let leaf = UInt8::new_input(ark_relations::ns!(cs, "leaf_var"), || Ok(&self.leaf))?;

        // --- Constant params ---
        let leaf_crh_params = LeafHashParamsVar::new_constant(
            cs.clone(),
            self.leaf_crh_params
                .as_ref()
                .expect("leaf_crh_params must be Some"),
        )?;
        let two_to_one_crh_params = TwoToOneHashParamsVar::new_constant(
            cs.clone(),
            self.two_to_one_crh_params
                .as_ref()
                .expect("two_to_one_crh_params must be Some"),
        )?;

        // --- Private witness (path) ---
        let path = SimplePathVar::new_witness(ark_relations::ns!(cs, "path_var"), || {
            self.authentication_path
                .as_ref()
                .ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Leaf as bytes (your leaf is a single u8)
        let leaf_bytes = vec![leaf.clone()];

        // Enforce membership == true
        path.verify_membership(&leaf_crh_params, &two_to_one_crh_params, &root, &leaf_bytes)?
            .enforce_equal(&Boolean::TRUE)?;

        Ok(())
    }
}
