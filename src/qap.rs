use ark_bn254::Fr;
use crate::utils::*;
use crate::r1cs::{LEFT_MATRIX, RIGHT_MATRIX, RESULT_MATRIX};
use crate::witness::WITNESS;
use std::sync::LazyLock;
use ark_ff:: Field;

fn qap_representation<const N: usize, const M:usize>(matrix: &[[Fr; M]; N], witness: &[Fr; M]) -> [Fr; N] {
    let matn_x: [[Fr; N]; M] = interpolate_matrix(&matrix);
    let mut a_matn_x: [Fr; N] = scalar_mul(&matn_x[0], witness[0]);

    for i in 1..M {
        a_matn_x = add(&a_matn_x, &scalar_mul(&matn_x[i], witness[i]));
    }

    a_matn_x
}

#[derive(Clone)]
pub struct QAP <const N: usize, const M: usize>{
    pub u_x: [Fr; N],
    pub v_x: [Fr; N],
    pub w_x: [Fr; N],
    pub t_x: [Fr; M],
    pub h_x: [Fr; M]
}

impl <const N: usize, const M: usize> QAP <N,M>{
    pub fn new(u_x: [Fr; N], v_x: [Fr; N], w_x: [Fr; N], t_x: [Fr; M], h_x: [Fr; M]) -> Self {
        Self { u_x, v_x, w_x, t_x, h_x }
    }
}

static U_X: LazyLock<[Fr; 2]> = LazyLock::new(|| {qap_representation(&LEFT_MATRIX, &WITNESS)});
static V_X: LazyLock<[Fr; 2]> = LazyLock::new(|| {qap_representation(&RIGHT_MATRIX, &WITNESS)});
static W_X: LazyLock<[Fr; 2]> = LazyLock::new(|| {qap_representation(&RESULT_MATRIX, &WITNESS)});
static T_X: LazyLock<[Fr; 3]> = LazyLock::new(|| calculate_tx::<3>());
static H_X: LazyLock<[Fr; 3]> = LazyLock::new(|| {calculate_hx(&U_X, &V_X, &W_X, &T_X)});

pub static QAP_FOR_PROOF: LazyLock<QAP<2,3>> = LazyLock::new(|| QAP::<2,3>::new((*U_X).clone(), (*V_X).clone(), (*W_X).clone(), (*T_X).clone(), (*H_X).clone()));
