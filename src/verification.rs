use ark_ff::{Zero};
use ark_ec:: {pairing::Pairing};
use crate::proof::{Proof};
use crate::trusted_setup::{GENERATED_SRS};
use crate::curve_ops::*;
use crate::r1cs::{L};
use crate::types::*;

pub fn verify_proof(proof: &Proof, pub_inputs: &[Field; L]) -> bool {
    let lhs = Curve::pairing(proof.a_1, proof.b_2);

    //x is Ψ evaluations * corresponding public elements of the witness
    let mut x: G1Point = *G1 * Field::zero();
    for i in 0..L {
        x += GENERATED_SRS.psi_pub[i] * pub_inputs[i];
    }

    //let rhs = E::pairing(GENERATED_SRS.alpha_1, GENERATED_SRS.beta_2) * E::pairing(x, GENERATED_SRS.gamma_2) * E::pairing(proof.C, GENERATED_SRS.delta_2);

    let rhs = Curve::multi_pairing([GENERATED_SRS.alpha_1, x, proof.c_1], [GENERATED_SRS.beta_2, GENERATED_SRS.gamma_2, GENERATED_SRS.delta_2]);

    // e(A,B) = e(α,β).e(χ,γ).e(C,δ)
    lhs == rhs
}