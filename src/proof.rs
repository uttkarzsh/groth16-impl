use ark_ff::{UniformRand};
use rand::thread_rng;
use crate::trusted_setup::{GENERATED_SRS};
use crate::qap::{QAP};
use crate::curve_ops::*;
use crate::types::*;

pub struct Proof {  //α,β
    pub a_1: G1Point, // [A]1 = [α]1 + Σai*ui(x) + r[δ]1
    pub b_2: G2Point, // [B]2 = [β]2 + Σai*vi(x) + s[δ]2
    pub c_1: G1Point  // [C]1 = Σai*[Ψi]1 + s*[A]1 + r*[B]1 - rs[δ]1
}

impl Proof{

    pub fn generate_proof(_qap: &QAP) -> Self {
        let u_tau_g1: G1Point = sum_group_elements(&multiply(&GENERATED_SRS.ptau_g1, &_qap.u_x));
        let v_tau_g1: G1Point = sum_group_elements(&multiply(&GENERATED_SRS.ptau_g1, &_qap.v_x));
        let v_tau_g2: G2Point = sum_group_elements(&multiply(&GENERATED_SRS.ptau_g2, &_qap.v_x));
        let a_psi_pvt_tau: G1Point = sum_group_elements(&_qap.psi_pvt_w);
        let h_tau_t_tau: G1Point = sum_group_elements(&multiply(&GENERATED_SRS.srs_hx_ttau, &_qap.h_x));


        //salting proof for true zero knowledge
        let mut rng = thread_rng();
        let r: Field = Field::rand(&mut rng);
        let s: Field = Field::rand(&mut rng);

        let a_1: G1Point = u_tau_g1 + GENERATED_SRS.alpha_1 + (GENERATED_SRS.delta_1 * r);
        let b_1: G1Point = v_tau_g1 + GENERATED_SRS.beta_1 + (GENERATED_SRS.delta_1 * s);
        let b_2: G2Point = v_tau_g2 + GENERATED_SRS.beta_2 + (GENERATED_SRS.delta_2 * s);
        let c_1: G1Point = a_psi_pvt_tau + h_tau_t_tau + (a_1 * s) + (b_1 * r) - (GENERATED_SRS.delta_1 * (r*s));

        Self { a_1, b_2, c_1 }
    }
}
