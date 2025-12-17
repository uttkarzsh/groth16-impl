use ark_bn254:: {Fr, G1Projective, G2Projective, Bn254};
use ark_ec:: {CurveGroup, PrimeGroup, pairing::Pairing};
use crate::proof::{Proof};
use crate::trusted_setup::{GENERATED_SRS};
use crate::curve_ops::*;
use crate::r1cs::{L};

pub fn verify_proof(proof: &Proof, pub_inputs: &[Fr; L]) -> bool {
    let lhs = Bn254::pairing(proof.a_1, proof.b_2);
    let mut x: G1Projective = *G1 * Fr::from(0u64);

    for i in 0..L {
        x += GENERATED_SRS.psi_pub[i] * pub_inputs[i];
    }

    //let rhs = Bn254::pairing(GENERATED_SRS.alpha_1, GENERATED_SRS.beta_2) * Bn254::pairing(x, GENERATED_SRS.gamma_2) * Bn254::pairing(proof.C, GENERATED_SRS.delta_2);

    let rhs = Bn254::multi_pairing([GENERATED_SRS.alpha_1, x, proof.c_1], [GENERATED_SRS.beta_2, GENERATED_SRS.gamma_2, GENERATED_SRS.delta_2]);

    lhs == rhs
}