use std::sync::LazyLock;

use crate::utils::*;
use crate::r1cs::{LEFT_MATRIX, RIGHT_MATRIX, RESULT_MATRIX, N, M, D, L};
use crate::witness::WITNESS;
use crate::trusted_setup::{GENERATED_SRS};
use crate::curve_ops::*;
use crate::types::*;


pub struct QAP {
    pub u_x: [Field; N],
    pub v_x: [Field; N],
    // pub w_x: [Field; N],
    // pub t_x: [Field; D],
    pub h_x: [Field; D],
    pub psi_pvt_w: [G1Point; M - L]    //Ψ polynomials from trusted setup evaluated at τ (multiplied by corresponding witness element)
}

impl QAP {
    pub fn new(left_matrix: &[[Field; M]; N], right_matrix: &[[Field; M]; N], result_matix: &[[Field; M]; N], witness: &[Field; M]) -> Self {
        let u_x: [Field; N] = qap_representation(&left_matrix, &witness);
        let v_x: [Field; N] = qap_representation(&right_matrix, &witness);
        let w_x: [Field; N] = qap_representation(&result_matix, &witness);
        let t_x: [Field; D] = calculate_tx::<D>();
        let h_x: [Field; D] = calculate_hx(&u_x, &v_x, &w_x, &t_x);

        let mut psi_pvt_w: [G1Point; M - L] = [*G1; M - L];

        for i in 0..M - L {
            psi_pvt_w[i] = GENERATED_SRS.psi_pvt[i] * witness[i + L];
        }
        
        Self { u_x, v_x, h_x, psi_pvt_w }
    }
}

pub static QAP_FOR_PROOF: LazyLock<QAP> = LazyLock::new(|| QAP::new(&LEFT_MATRIX, &RIGHT_MATRIX, &RESULT_MATRIX, &WITNESS));
