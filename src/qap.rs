use ark_bn254::{Fr, G1Projective, G2Projective};
use ark_ff:: Field;
use std::sync::LazyLock;

use crate::utils::*;
use crate::r1cs::{LEFT_MATRIX, RIGHT_MATRIX, RESULT_MATRIX, N, M, D, L};
use crate::witness::WITNESS;
use crate::trusted_setup::{srs};
use crate::curve_ops::*;


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
    pub h_x: [Fr; D],
    pub psi_pvt_w: [G1Projective; M - L]
}

impl QAP {
    pub fn new(left_matrix: &[[Fr; M]; N], right_matrix: &[[Fr; M]; N], result_matix: &[[Fr; M]; N], witness: &[Fr; M]) -> Self {
        let u_x: [Fr; N] = qap_representation(&left_matrix, &witness);
        let v_x: [Fr; N] = qap_representation(&right_matrix, &witness);
        let w_x: [Fr; N] = qap_representation(&result_matix, &witness);
        let t_x: [Fr; D] = calculate_tx::<D>();
        let h_x: [Fr; D] = calculate_hx(&u_x, &v_x, &w_x, &t_x);

        let mut psi_pvt_w: [G1Projective; M - L] = [*G1; M - L];

        for i in 0..M - L {
            psi_pvt_w[i] = srs.psi_pvt[i] * witness[i + L];
        }
        
        Self { u_x, v_x, w_x, t_x, h_x, psi_pvt_w }
    }
}

pub static QAP_FOR_PROOF: LazyLock<QAP> = LazyLock::new(|| QAP::new(&LEFT_MATRIX, &RIGHT_MATRIX, &RESULT_MATRIX, &WITNESS));
