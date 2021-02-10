use crate::bls::Engine;
use crate::groth16::aggregate::commit;
use serde::{Deserialize, Serialize};
/// AggregateProof contains all elements to verify n aggregated Groth16 proofs
/// using inner pairing product arguments. This proof can be created by any
/// party in possession of valid Groth16 proofs.
#[derive(Serialize, Deserialize)]
pub struct AggregateProof<E: Engine> {
    /// commitment to A and B using the pair commitment scheme needed to verify
    /// TIPP relation.
    #[serde(bound(
        serialize = "E::Fqk: Serialize, E::Fqk: Serialize",
        deserialize = "E::Fqk: Deserialize<'de>, E::Fqk: Deserialize<'de>",
    ))]
    pub com_ab: commit::Output<E>,
    /// commit to C separate since we use it only in MIPP
    #[serde(bound(
        serialize = "E::Fqk: Serialize, E::Fqk: Serialize",
        deserialize = "E::Fqk: Deserialize<'de>, E::Fqk: Deserialize<'de>",
    ))]
    pub com_c: commit::Output<E>,
    /// $A^r * B = Z$ is the left value on the aggregated Groth16 equation
    pub ip_ab: E::Fqk,
    /// $C^r$ is used on the right side of the aggregated Groth16 equation
    pub agg_c: E::G1,
    /// tipp proof for proving correct aggregation of A and B
    #[serde(bound(
        serialize = "TIPPProof<E>: Serialize",
        deserialize = "TIPPProof<E>: Deserialize<'de>",
    ))]
    pub proof_ab: TIPPProof<E>,
    /// mipp proof for proving correct scaling of C
    #[serde(bound(
        serialize = "MIPPProof<E>: Serialize",
        deserialize = "MIPPProof<E>: Deserialize<'de>",
    ))]
    pub proof_c: MIPPProof<E>,
}

/// GipaTIPP proof contains all the information necessary to verify the Gipa
/// TIPP statement. All GIPA related elements are given in order of generation
/// by the prover.
#[derive(Serialize, Deserialize)]
pub struct GipaTIPP<E: Engine> {
    /// ((T_L, U_L),(T_R,U_R)) values accross all steps
    #[serde(bound(
        serialize = "E::Fqk: Serialize, E::Fqk: Serialize",
        deserialize = "E::Fqk: Deserialize<'de>, E::Fqk: Deserialize<'de>",
    ))]
    pub comms: Vec<(commit::Output<E>, commit::Output<E>)>,
    /// Z values left and right
    #[serde(bound(
        serialize = "E::Fqk: Serialize, E::Fqk: Serialize",
        deserialize = "E::Fqk: Deserialize<'de>, E::Fqk: Deserialize<'de>",
    ))]
    pub z_vec: Vec<(E::Fqk, E::Fqk)>,
    /// final values of A and B at the end of the recursion
    pub final_a: E::G1Affine,
    pub final_b: E::G2Affine,
    /// final commitment keys $v$ and $w$ - there is only one element at the
    /// end for v1 and v2 hence it's a tuple.
    #[serde(bound(
        serialize = "E::G2Affine: Serialize, E::G2Affine: Serialize",
        deserialize = "E::G2Affine: Deserialize<'de>, E::G1Affine: Deserialize<'de>",
    ))]
    pub final_vkey: (E::G2Affine, E::G2Affine),
    #[serde(bound(
        serialize = "E::G1Affine: Serialize, E::G1Affine: Serialize",
        deserialize = "E::G1Affine: Deserialize<'de>, E::G1Affine: Deserialize<'de>",
    ))]
    pub final_wkey: (E::G1Affine, E::G1Affine),
}

/// TIPPProof contains a GIPA proof as well as the opening of the rescaled
/// commitment keys - to let the verifier prove sucinctly the rpvoer has
/// correctly performed the recursion on the commitment keys as well.
#[derive(Serialize, Deserialize)]
pub struct TIPPProof<E: Engine> {
    #[serde(bound(
        serialize = "GipaTIPP<E>: Serialize",
        deserialize = "GipaTIPP<E>: Deserialize<'de>",
    ))]
    pub gipa: GipaTIPP<E>,
    #[serde(bound(
        serialize = "E::G2Affine: Serialize",
        deserialize = "E::G2Affine: Deserialize<'de>",
    ))]
    pub vkey_opening: KZGOpening<E::G2Affine>,
    #[serde(bound(
        serialize = "E::G1Affine: Serialize",
        deserialize = "E::G1Affine: Deserialize<'de>",
    ))]
    pub wkey_opening: KZGOpening<E::G1Affine>,
}

/// KZGOpening represents the KZG opening of a commitment key (which is a tuple
/// given commitment keys are a tuple).
pub type KZGOpening<G> = (G, G);

/// GipaMIPP is similar to GipaTIPP: it contains information to verify the
/// GIPA recursion using the commitment of MIPP. Section 4 of the paper.
#[derive(Serialize, Deserialize)]
pub struct GipaMIPP<E: Engine> {
    /// ((T_L, U_L),(T_R,U_R)) values accross all steps
    #[serde(bound(
        serialize = "E::Fqk: Serialize, E::Fqk: Serialize",
        deserialize = "E::Fqk: Deserialize<'de>, E::Fqk: Deserialize<'de>",
    ))]
    pub comms: Vec<(commit::Output<E>, commit::Output<E>)>,
    /// Z values left and right
    #[serde(bound(
        serialize = "E::G1: Serialize, E::G1: Serialize",
        deserialize = "E::G1: Deserialize<'de>, E::G1: Deserialize<'de>",
    ))]
    pub z_vec: Vec<(E::G1, E::G1)>,
    /// final values of C at the end of the recursion
    pub final_c: E::G1Affine,
    pub final_r: E::Fr,
    /// final commitment keys $v$ - there is only one element at the
    /// end for v1 and v2 hence it's a tuple.
    #[serde(bound(
        serialize = "E::G2Affine: Serialize, E::G2Affine: Serialize",
        deserialize = "E::G2Affine: Deserialize<'de>, E::G2Affine: Deserialize<'de>",
    ))]
    pub final_vkey: (E::G2Affine, E::G2Affine),
}

/// MIPPProof contains the GIPA proof as well as the opening information to be
/// able to verify the correctness of the commitment keys.
#[derive(Serialize, Deserialize)]
pub struct MIPPProof<E: Engine> {
    #[serde(bound(
        serialize = "GipaMIPP<E>: Serialize",
        deserialize = "GipaMIPP<E>: Deserialize<'de>",
    ))]
    pub gipa: GipaMIPP<E>,
    #[serde(bound(
        serialize = "E::G2Affine: Serialize",
        deserialize = "E::G2Affine: Deserialize<'de>",
    ))]
    pub vkey_opening: KZGOpening<E::G2Affine>,
}
