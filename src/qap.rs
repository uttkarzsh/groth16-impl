use ark_bn254::Fr;
use crate::utils::*;
use crate::r1cs::{LEFT_MATRIX, RIGHT_MATRIX, RESULT_MATRIX, N, M, D};
use crate::witness::WITNESS;
use std::sync::LazyLock;
use ark_ff:: Field;

fn qap_representation(matrix: &[[Fr; M]; N], witness: &[Fr; M]) -> [Fr; N] {
    let matn_x: [[Fr; N]; M] = interpolate_matrix(&matrix);
    let mut a_matn_x: [Fr; N] = scalar_mul(&matn_x[0], witness[0]);

    for i in 1..M {
        a_matn_x = add(&a_matn_x, &scalar_mul(&matn_x[i], witness[i]));
    }

    a_matn_x
}

pub struct QAP {
    pub u_x: [Fr; N],
    pub v_x: [Fr; N],
    pub w_x: [Fr; N],
    pub t_x: [Fr; D],
    pub h_x: [Fr; D]
}

impl QAP {
    pub fn new(u_x: [Fr; N], v_x: [Fr; N], w_x: [Fr; N], t_x: [Fr; D], h_x: [Fr; D]) -> Self {
        Self { u_x, v_x, w_x, t_x, h_x }
    }
}

static U_X: LazyLock<[Fr; N]> = LazyLock::new(|| {qap_representation(&LEFT_MATRIX, &WITNESS)});
static V_X: LazyLock<[Fr; N]> = LazyLock::new(|| {qap_representation(&RIGHT_MATRIX, &WITNESS)});
static W_X: LazyLock<[Fr; N]> = LazyLock::new(|| {qap_representation(&RESULT_MATRIX, &WITNESS)});
static T_X: LazyLock<[Fr; D]> = LazyLock::new(|| calculate_tx::<D>());
static H_X: LazyLock<[Fr; D]> = LazyLock::new(|| {calculate_hx(&U_X, &V_X, &W_X, &T_X)});

pub static QAP_FOR_PROOF: LazyLock<QAP> = LazyLock::new(|| QAP::new(*U_X, *V_X, *W_X, *T_X, *H_X));
