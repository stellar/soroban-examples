use ark_crypto_primitives::crh::constraints::{CRHSchemeGadget, TwoToOneCRHSchemeGadget};
use ark_crypto_primitives::crh::injective_map::constraints::{
    PedersenCRHCompressorGadget, PedersenTwoToOneCRHCompressorGadget, TECompressorGadget,
};
use ark_crypto_primitives::crh::{
    injective_map::{PedersenCRHCompressor, PedersenTwoToOneCRHCompressor, TECompressor},
    pedersen,
};
use ark_ed_on_bls12_381::{EdwardsProjective, constraints::EdwardsVar};

pub type TwoToOneHash =
    PedersenTwoToOneCRHCompressor<EdwardsProjective, TECompressor, TwoToOneWindow>;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwoToOneWindow;

// `WINDOW_SIZE * NUM_WINDOWS` = 2 * 256 bits = enough for hashing two outputs.
impl pedersen::Window for TwoToOneWindow {
    const WINDOW_SIZE: usize = 4;
    const NUM_WINDOWS: usize = 128;
}

pub type LeafHash = PedersenCRHCompressor<EdwardsProjective, TECompressor, LeafWindow>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct LeafWindow;

// `WINDOW_SIZE * NUM_WINDOWS` = 2 * 256 bits = enough for hashing two outputs.
impl pedersen::Window for LeafWindow {
    const WINDOW_SIZE: usize = 4;
    const NUM_WINDOWS: usize = 144;
}

pub type LeafHashGadget = PedersenCRHCompressorGadget<
    EdwardsProjective,
    TECompressor,
    LeafWindow,
    EdwardsVar,
    TECompressorGadget,
>;

pub type TwoToOneHashGadget = PedersenTwoToOneCRHCompressorGadget<
    EdwardsProjective,
    TECompressor,
    TwoToOneWindow,
    EdwardsVar,
    TECompressorGadget,
>;

pub type LeafHashParamsVar =
    <LeafHashGadget as CRHSchemeGadget<LeafHash, ConstraintF>>::ParametersVar;
pub type TwoToOneHashParamsVar =
    <TwoToOneHashGadget as TwoToOneCRHSchemeGadget<TwoToOneHash, ConstraintF>>::ParametersVar;

pub type ConstraintF = ark_ed_on_bls12_381::Fq;
