use ark_bn254::{G1Projective, G2Projective};
use crate::trusted_setup::{srs, SRS};
use crate::qap::{QAP, QAP_FOR_PROOF};
use crate::curve_ops::*;
use crate::r1cs::{N, M, D};

pub struct Proof {
    pub A: G1Projective,
    pub B: G2Projective,
    pub C: G1Projective
}

impl Proof{

    pub fn new(_qap: &QAP) -> Self {
        let u_tau: G1Projective = sum_g1_array(&hadamard_g1(&srs.ptau_g1, &_qap.u_x));
        let v_tau: G2Projective = sum_g2_array(&hadamard_g2(&srs.ptau_g2, &_qap.v_x));
        let a_psi_pvt_tau: G1Projective = sum_g1_array(&_qap.psi_pvt_w);
        let h_tau_t_tau: G1Projective = sum_g1_array(&hadamard_g1(&srs.srs_hx_ttau, &_qap.h_x));

        let A: G1Projective = u_tau + srs.alpha_1;
        let B: G2Projective = v_tau + srs.beta_2;
        let C: G1Projective = a_psi_pvt_tau + h_tau_t_tau;

        Self { A, B, C }
    }
}
