use ark_bn254:: {Fr, G1Projective, G2Projective, Bn254};
use ark_ec:: {CurveGroup, PrimeGroup, pairing::Pairing};
use crate::proof::{Proof};
use crate::curve_ops::*;

pub fn verify_proof<const N:usize, const M: usize>(proof: Proof<N,M>) -> bool {
    let u_tau_v_tau = Bn254::pairing(proof.A, proof.B);
    let w_plus_ht_tau = Bn254::pairing(proof.C, *G2);

    u_tau_v_tau == w_plus_ht_tau
}