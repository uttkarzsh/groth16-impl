use ark_bn254::{Fr, G1Projective, G2Projective};
use ark_ff::{UniformRand};
use rand::thread_rng;
use crate::trusted_setup::{GENERATED_SRS, SRS};
use crate::qap::{QAP, QAP_FOR_PROOF};
use crate::curve_ops::*;
use crate::r1cs::{N, M, D};

pub struct Proof {
    pub a_1: G1Projective,
    pub b_2: G2Projective,
    pub c_1: G1Projective
}

impl Proof{

    pub fn new(_qap: &QAP) -> Self {
        let u_tau_g1: G1Projective = sum_g1_array(&hadamard_g1(&GENERATED_SRS.ptau_g1, &_qap.u_x));
        let v_tau_g1: G1Projective = sum_g1_array(&hadamard_g1(&GENERATED_SRS.ptau_g1, &_qap.v_x));
        let v_tau_g2: G2Projective = sum_g2_array(&hadamard_g2(&GENERATED_SRS.ptau_g2, &_qap.v_x));
        let a_psi_pvt_tau: G1Projective = sum_g1_array(&_qap.psi_pvt_w);
        let h_tau_t_tau: G1Projective = sum_g1_array(&hadamard_g1(&GENERATED_SRS.srs_hx_ttau, &_qap.h_x));


        let mut rng = thread_rng();

        let r: Fr = Fr::rand(&mut rng);
        let s: Fr = Fr::rand(&mut rng);

        let a_1: G1Projective = u_tau_g1 + GENERATED_SRS.alpha_1 + (GENERATED_SRS.delta_1 * r);
        let b_1: G1Projective = v_tau_g1 + GENERATED_SRS.beta_1 + (GENERATED_SRS.delta_1 * s);
        let b_2: G2Projective = v_tau_g2 + GENERATED_SRS.beta_2 + (GENERATED_SRS.delta_2 * s);
        let c_1: G1Projective = a_psi_pvt_tau + h_tau_t_tau + (a_1 * s) + (b_1 * r) - (GENERATED_SRS.delta_1 * (r*s));

        Self { a_1, b_2, c_1 }
    }
}
