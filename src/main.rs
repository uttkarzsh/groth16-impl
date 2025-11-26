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
use qap::{U_X, V_X, W_X, H_X, T_TAU, SRS};

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

    let ptau3: [Fr; 3] = [Fr::from(0u64), SRS.ptau[0], SRS.ptau[1]];    
    let u_tau: Fr = arr_sum2(&hadamard_product(&U_X, &SRS.ptau));
    let v_tau: Fr = arr_sum2(&hadamard_product(&V_X, &SRS.ptau));
    let w_tau: Fr = arr_sum2(&hadamard_product(&W_X, &SRS.ptau));
    let h_tau: Fr = arr_sum3(&hadamard_product3(&H_X, &ptau3));

    let verification_successful: bool = u_tau * v_tau == w_tau + h_tau * *T_TAU;

    if verification_successful {
        println!("witness correct yay");
    } else {
        println!("wrong witness lol");
    }
}
