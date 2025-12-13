use ark_bn254::Fr;
use crate::utils::*;
use crate::r1cs::{LEFT_MATRIX, RIGHT_MATRIX, RESULT_MATRIX};
use crate::witness::WITNESS;
use crate::trusted_setup::{SRS, generate_srs};
use std::sync::LazyLock;
use ark_ff:: Field;

fn qap_representation(matrix: &[[Fr; 3]; 2], witness: &[Fr; 3]) -> [Fr; 2] {
    let matn_x: [[Fr; 2]; 3] = interpolate_matrix(&matrix);
    let mut a_matn_x: [Fr; 2] = scalar_mul(&matn_x[0], witness[0]);

    for i in 1..3 {
        a_matn_x = add(&a_matn_x, &scalar_mul(&matn_x[i], witness[i]));
    }

    a_matn_x
}

pub static U_X: LazyLock<[Fr; 2]> = LazyLock::new(|| {qap_representation(&LEFT_MATRIX, &WITNESS)});
pub static V_X: LazyLock<[Fr; 2]> = LazyLock::new(|| {qap_representation(&RIGHT_MATRIX, &WITNESS)});
pub static W_X: LazyLock<[Fr; 2]> = LazyLock::new(|| {qap_representation(&RESULT_MATRIX, &WITNESS)});
pub static T_X: LazyLock<[Fr; 3]> = LazyLock::new(|| calculate_tx::<3>());
pub static H_X: LazyLock<[Fr; 3]> = LazyLock::new(|| {calculate_hx(&U_X, &V_X, &W_X, &T_X)});


