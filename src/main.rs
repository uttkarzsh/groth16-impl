#![allow(unused)]

mod utils;
mod r1cs;

use ark_bn254:: Fr;
use r1cs::{LEFT_MATRIX, RIGHT_MATRIX, RESULT_MATRIX};
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
    let witness_vector: [Fr; 3] = [Fr::from(1u64), Fr::from(4u64), Fr::from(16u64)];

    let l_w: [Fr; 2] = matrix_mul(&LEFT_MATRIX, &witness_vector);
    let r_w: [Fr; 2] = matrix_mul(&RIGHT_MATRIX, &witness_vector);
    let o_w: [Fr; 2] = matrix_mul(&RESULT_MATRIX, &witness_vector);

    if check_matrix_eq(hadamard_product(&l_w, &r_w), o_w) {
        println!("witness correct yay");
    } else {
        println!("wrong witness lol");
    }
}
