use ark_bn254:: {Fr, G1Projective, G2Projective, Bn254};
use ark_ec:: {CurveGroup, PrimeGroup, pairing::Pairing};
use crate::proof::{Proof};
use crate::trusted_setup::{srs};
use crate::curve_ops::*;
use crate::r1cs::{L};

pub fn verify_proof(proof: &Proof, pub_inputs: &[Fr; L]) -> bool {
    let lhs = Bn254::pairing(proof.A, proof.B);
    let mut x: G1Projective = *G1 * Fr::from(0u64);

    for i in 0..L {
        x += srs.psi_pub[i] * pub_inputs[i];
    }

    //let rhs = Bn254::pairing(srs.alpha_1, srs.beta_2) * Bn254::pairing(x, srs.gamma_2) * Bn254::pairing(proof.C, srs.delta_2);

    let rhs = Bn254::multi_pairing([srs.alpha_1, x, proof.C], [srs.beta_2, srs.gamma_2, srs.delta_2]);

    lhs == rhs
}