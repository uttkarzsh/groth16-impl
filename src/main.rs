#![allow(unused)]

mod utils;
mod r1cs;

use r1cs::{LEFT_MATRIX};
use ark_bn254:: Fr;
use utils::matrix_mul;



fn main() {
    let witness_vector: [Fr; 3] = [Fr::from(1u64), Fr::from(4u64), Fr::from(16u64)];

    let lol : [Fr; 2] = matrix_mul(&LEFT_MATRIX, &witness_vector);
    

    println!("{:?}", lol[0]);
}
