use ark_bn254::Fr;
use crate::utils::{interpolate_matrix, scalar_mul,hadamard_product, add_2, sub_2};
use crate::r1cs::{LEFT_MATRIX, RIGHT_MATRIX, RESULT_MATRIX};
use crate::witness::WITNESS;
use crate::trusted_setup::{SRS, generate_srs};
use std::sync::LazyLock;

pub fn qap_representation(matrix: &[[Fr; 3]; 2], witness: &[Fr; 3]) -> [Fr; 2] {
    let matn_x: [[Fr; 2]; 3] = interpolate_matrix(&matrix);
    let mut a_matn_x: [Fr; 2] = scalar_mul(&matn_x[0], witness[0]);

    for i in 1..3 {
        a_matn_x = add_2(&a_matn_x, &scalar_mul(&matn_x[i], witness[i]));
    }

    a_matn_x
}

pub static SRS: LazyLock<SRS> = LazyLock::new(|| { generate_srs()});


pub static U_X: LazyLock<[Fr; 2]> = LazyLock::new(|| {qap_representation(&LEFT_MATRIX, &WITNESS)});
pub static V_X: LazyLock<[Fr; 2]> = LazyLock::new(|| {qap_representation(&RIGHT_MATRIX, &WITNESS)});
pub static W_X: LazyLock<[Fr; 2]> = LazyLock::new(|| {qap_representation(&RESULT_MATRIX, &WITNESS)});

pub static H_X: LazyLock<[Fr; 2]> = LazyLock::new(|| {sub_2(&hadamard_product(&U_X, &V_X), &W_X)});

pub fn calculate_t_tau(ptau: [Fr; 1]) -> Fr {
    SRS.ptau[0] - Fr::from(1u64)
}
