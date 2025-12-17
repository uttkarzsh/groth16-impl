use ark_bn254::{Fr, G1Projective, G2Projective};
use ark_ff::{UniformRand, Field};
use ark_ec::{CurveGroup, PrimeGroup};
use rand::thread_rng;
use std::sync::LazyLock;
use crate::utils::*;
use crate::curve_ops::{G1, G2};
use crate::r1cs::{LEFT_MATRIX, RIGHT_MATRIX, RESULT_MATRIX, N, M, D, L};

pub struct SRS {
    pub alpha_1: G1Projective,
    pub beta_1: G1Projective,
    pub beta_2: G2Projective,
    pub gamma_2: G2Projective,
    pub delta_1: G1Projective,
    pub delta_2: G2Projective,
    pub ptau_g1: [G1Projective; D],
    pub ptau_g2: [G2Projective; D],
    pub srs_hx_ttau: [G1Projective; D],
    pub psi_pub: [G1Projective; L],
    pub psi_pvt: [G1Projective; M - L]
}

impl SRS {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let tau: Fr = Fr::rand(&mut rng);

        let mut ptau: [Fr; D] = [Fr::from(1u64); D];
        let mut ptau_g1: [G1Projective; D] = [*G1; D];
        let mut ptau_g2: [G2Projective; D] = [*G2; D];
        let mut srs_hx_ttau: [G1Projective; D] = [*G1; D];

        let alpha: Fr = Fr::rand(&mut rng);
        let beta: Fr = Fr::rand(&mut rng);
        let gamma: Fr = Fr::rand(&mut rng);
        let delta: Fr = Fr::rand(&mut rng); 

        let gamma_inv: Fr = gamma.inverse().unwrap();
        let delta_inv: Fr = delta.inverse().unwrap();

        let alpha_1: G1Projective = *G1 * alpha;
        let beta_1: G1Projective = *G1 * beta;
        let beta_2: G2Projective = *G2 * beta;
        let gamma_2: G2Projective = *G2 * gamma;
        let delta_1: G1Projective = *G1 * delta;
        let delta_2: G2Projective = *G2 * delta;
        
        for i in 0..D {
            ptau[i] = tau.pow([i as u64]);
            ptau_g1[i] = *G1 * ptau[i];
            ptau_g2[i] = *G2 * ptau[i];
        }
        let t_x: [Fr; D] = calculate_tx::<D>();
        let t_tau: Fr = arr_sum(&hadamard_product(&ptau, &t_x));

        for i in 0..D {
            let num: Fr = t_tau * ptau[i] * delta_inv;
            srs_hx_ttau[i] = *G1 * num;
        }

        let mut psi_pub: [G1Projective; L] = [*G1; L];
        let mut psi_pvt: [G1Projective; M - L] = [*G1; M - L];

        let ui_x: [[Fr; N]; M] = interpolate_matrix(&LEFT_MATRIX);
        let vi_x: [[Fr; N]; M] = interpolate_matrix(&RIGHT_MATRIX);
        let wi_x: [[Fr; N]; M] = interpolate_matrix(&RESULT_MATRIX);

        for i in 0..L {
            let arr: [Fr; N] = scalar_mul(&add_three_arr(&scalar_mul(&ui_x[i], beta), &scalar_mul(&vi_x[i], alpha), &wi_x[i]), gamma_inv);

            let arr_tau: Fr = arr_sum(&hadamard_product(&ptau, &arr));

            psi_pub[i] = *G1 * arr_tau;
        }

        for i in 0..M - L {
            let arr: [Fr; N] = scalar_mul(&add_three_arr(&scalar_mul(&ui_x[i + L], beta), &scalar_mul(&vi_x[i + L], alpha), &wi_x[i + L]), delta_inv);

            let arr_tau: Fr = arr_sum(&hadamard_product(&ptau, &arr));

            psi_pvt[i] = *G1 * arr_tau;
        }

        Self { alpha_1, beta_1, beta_2, gamma_2, delta_1, delta_2, ptau_g1, ptau_g2, srs_hx_ttau, psi_pub, psi_pvt }
    }
}

pub static srs: LazyLock<SRS> = LazyLock::new(|| SRS::new());