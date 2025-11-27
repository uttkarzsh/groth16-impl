#![allow(unused)]

mod utils;
mod r1cs;
mod witness;
mod trusted_setup;
mod qap;

use ark_bn254:: Fr;
use r1cs::{LEFT_MATRIX, RIGHT_MATRIX, RESULT_MATRIX};
use witness::{WITNESS};
use utils::*;
use trusted_setup::SRS;
use qap::{U_X, V_X, W_X, HX_TX, T_X, SRS};

fn check_matrix_eq(a: [Fr; 2], b: [Fr; 2]) -> bool {
    let mut is_equal: bool = true;

    for i in 0..2 {
        if a[i] != b[i] {
            is_equal = false;
        }
    }

    is_equal
}

fn main() {

    /* 
    let l_w: [Fr; 2] = matrix_mul(&LEFT_MATRIX, &WITNESS);
    let r_w: [Fr; 2] = matrix_mul(&RIGHT_MATRIX, &WITNESS);
    let o_w: [Fr; 2] = matrix_mul(&RESULT_MATRIX, &WITNESS);

    let verification_successful: bool = check_matrix_eq(hadamard_product(&l_w, &r_w), o_w);
    */

    let tau_sq: Fr = SRS.tau * SRS.tau;
    let ptau3: [Fr; 3] = [tau_sq, SRS.ptau[0], SRS.ptau[1]];    
    let u_tau: Fr = arr_sum2(&hadamard_product(&U_X, &SRS.ptau));
    let v_tau: Fr = arr_sum2(&hadamard_product(&V_X, &SRS.ptau));
    let w_tau: Fr = arr_sum2(&hadamard_product(&W_X, &SRS.ptau));
    let h_tau_t_tau: Fr = arr_sum3(&hadamard_product3(&HX_TX, &ptau3));
    // let t_tau: Fr = arr_sum3(&hadamard_product3(&T_X, &ptau3));

    let u_tau_v_tau: Fr = u_tau * v_tau;
    let w_plus_ht_tau: Fr = w_tau + h_tau_t_tau;

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
}
