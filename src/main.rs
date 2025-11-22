#![allow(unused)]

mod utils;
mod r1cs;
mod witness;

use ark_bn254:: Fr;
use r1cs::{LEFT_MATRIX, RIGHT_MATRIX, RESULT_MATRIX};
use witness::{WITNESS};
use utils::{matrix_mul, hadamard_product};

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

    let l_w: [Fr; 2] = matrix_mul(&LEFT_MATRIX, &WITNESS);
    let r_w: [Fr; 2] = matrix_mul(&RIGHT_MATRIX, &WITNESS);
    let o_w: [Fr; 2] = matrix_mul(&RESULT_MATRIX, &WITNESS);

    if check_matrix_eq(hadamard_product(&l_w, &r_w), o_w) {
        println!("witness correct yay");
    } else {
        println!("wrong witness lol");
    }
}
