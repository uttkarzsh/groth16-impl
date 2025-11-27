use ark_bn254::Fr;
use crate::utils::*;
use crate::r1cs::{LEFT_MATRIX, RIGHT_MATRIX, RESULT_MATRIX};
use crate::witness::WITNESS;
use crate::trusted_setup::{SRS, generate_srs};
use std::sync::LazyLock;
use ark_ff:: Field;

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

pub static T_X: LazyLock<[Fr; 3]> = LazyLock::new(|| [Fr::from(1u64), Fr::from(-3i64), Fr::from(2u64)]);

pub fn calculate_hx_tx(u_x: &[Fr; 2], v_x: &[Fr; 2], w_x: &[Fr; 2]) ->[Fr; 3]{
    let uv_x2: [Fr; 3] = polynomial_multiplication(&u_x, &v_x);
    let w_x2: [Fr;3] = [Fr::from(0u64), w_x[0], w_x[1]];

    let hx_tx: [Fr; 3] = sub_3(&uv_x2, &w_x2);
    hx_tx
}  

pub static HX_TX: LazyLock<[Fr; 3]> = LazyLock::new(|| {calculate_hx_tx(&U_X, &V_X, &W_X)});


