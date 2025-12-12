#![allow(unused)]

mod utils;
mod r1cs;
mod witness;
mod trusted_setup;
mod qap;
mod curve_ops;

use ark_bn254:: {Fr, G1Projective, G2Projective, Bn254};
use ark_ec:: {CurveGroup, PrimeGroup, pairing::Pairing};
use r1cs::{LEFT_MATRIX, RIGHT_MATRIX, RESULT_MATRIX};
use witness::{WITNESS};
use utils::*;
use qap::{U_X, V_X, W_X, H_X, SRS};
use curve_ops::*;


fn main() {

    /* 
    let l_w: [Fr; 2] = matrix_mul(&LEFT_MATRIX, &WITNESS);
    let r_w: [Fr; 2] = matrix_mul(&RIGHT_MATRIX, &WITNESS);
    let o_w: [Fr; 2] = matrix_mul(&RESULT_MATRIX, &WITNESS);

    let verification_successful: bool = check_matrix_eq(hadamard_product(&l_w, &r_w), o_w);
    */

    let ptaug1_sliced: [G1Projective; 2] = [SRS.ptau_g1[0], SRS.ptau_g1[1]];
    let ptaug2_sliced: [G2Projective; 2] = [SRS.ptau_g2[0], SRS.ptau_g2[1]];
    let u_tau: G1Projective = sum_g1_array(&hadamard_g1(&ptaug1_sliced, &U_X));
    let v_tau: G2Projective = sum_g2_array(&hadamard_g2(&ptaug2_sliced, &V_X));
    let w_tau: G1Projective = sum_g1_array(&hadamard_g1(&ptaug1_sliced, &W_X));
    let h_tau_t_tau: G1Projective = sum_g1_array(&hadamard_g1(&SRS.srs_hx_ttau, &H_X));

    let u_tau_v_tau = Bn254::pairing(u_tau, v_tau);
    let w_plus_ht_tau = Bn254::pairing(w_tau + h_tau_t_tau, *G2);

    let verification_successful: bool = u_tau_v_tau == w_plus_ht_tau;

    if verification_successful {
        println!("witness correct yay");
    } else {
        println!("wrong witness lol");
    }

    // check polynomials
    // println!("{} and {}", U_X[0], U_X[1]);
    // println!("{} and {}", V_X[0], V_X[1]);
    // println!("{} and {}", W_X[0], W_X[1]);
    // println!("{} and {} and {}", HX_TX[0], HX_TX[1], HX_TX[2]);

    // let mul_working: bool = mul_over_curve();
    // println!("{}", mul_working);

    // let pair_check: bool = pairing_check();
    // println!("{}", pair_check);
}
