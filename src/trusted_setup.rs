use ark_ff::{UniformRand, Field as IField, One};
use rand::thread_rng;
use std::sync::LazyLock;
use crate::utils::*;
use crate::types::*;
use crate::curve_ops::{G1, G2};
use crate::r1cs::{LEFT_MATRIX, RIGHT_MATRIX, RESULT_MATRIX, N, M, D, L};

pub struct SRS {
    //toxic waste curve points
    pub alpha_1: G1Point,
    pub beta_1: G1Point,
    pub beta_2: G2Point,
    pub gamma_2: G2Point,
    pub delta_1: G1Point,
    pub delta_2: G2Point,

    //powers of tau
    pub ptau_g1: [G1Point; D],     //powers of τ (G1)
    pub ptau_g2: [G2Point; D],     //powers of τ (G2)
    pub srs_hx_ttau: [G1Point; D], //powers of τ * t(τ)

    //psi = alpha * vi(x) + beta * ui(x) + wi(x)
    pub psi_pub: [G1Point; L],     //srs for pub inputs
    pub psi_pvt: [G1Point; M - L]  //srs for private inputs
}

impl SRS {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let tau: Field = Field::rand(&mut rng);

        let mut ptau: [Field; D] = [Field::one(); D];    // τ
        let mut ptau_g1: [G1Point; D] = [*G1; D];
        let mut ptau_g2: [G2Point; D] = [*G2; D];
        let mut srs_hx_ttau: [G1Point; D] = [*G1; D];

        let alpha: Field = Field::rand(&mut rng);
        let beta: Field = Field::rand(&mut rng);
        let gamma: Field = Field::rand(&mut rng);
        let delta: Field = Field::rand(&mut rng); 

        let gamma_inv: Field = gamma.inverse().unwrap();
        let delta_inv: Field = delta.inverse().unwrap();

        let alpha_1: G1Point = *G1 * alpha;
        let beta_1: G1Point = *G1 * beta;
        let beta_2: G2Point = *G2 * beta;
        let gamma_2: G2Point = *G2 * gamma;
        let delta_1: G1Point = *G1 * delta;
        let delta_2: G2Point = *G2 * delta;
        
        for i in 0..D {
            ptau[i] = tau.pow([i as u64]);
            ptau_g1[i] = *G1 * ptau[i];
            ptau_g2[i] = *G2 * ptau[i];
        }
        let t_x: [Field; D] = calculate_tx::<D>();
        let t_tau: Field = arr_sum(&hadamard_product(&ptau, &t_x));

        for i in 0..D {
            let num: Field = t_tau * ptau[i] * delta_inv;
            srs_hx_ttau[i] = *G1 * num;
        }

        let mut psi_pub: [G1Point; L] = [*G1; L];
        let mut psi_pvt: [G1Point; M - L] = [*G1; M - L];

        let ui_x: [[Field; N]; M] = interpolate_matrix(&LEFT_MATRIX);
        let vi_x: [[Field; N]; M] = interpolate_matrix(&RIGHT_MATRIX);
        let wi_x: [[Field; N]; M] = interpolate_matrix(&RESULT_MATRIX);

        for i in 0..L {
            let arr: [Field; N] = scalar_mul(&add_three_arr(&scalar_mul(&ui_x[i], beta), &scalar_mul(&vi_x[i], alpha), &wi_x[i]), gamma_inv);

            let arr_tau: Field = arr_sum(&hadamard_product(&ptau, &arr));

            psi_pub[i] = *G1 * arr_tau;
        }

        for i in 0..M - L {
            let arr: [Field; N] = scalar_mul(&add_three_arr(&scalar_mul(&ui_x[i + L], beta), &scalar_mul(&vi_x[i + L], alpha), &wi_x[i + L]), delta_inv);

            let arr_tau: Field = arr_sum(&hadamard_product(&ptau, &arr));

            psi_pvt[i] = *G1 * arr_tau;
        }

        Self { alpha_1, beta_1, beta_2, gamma_2, delta_1, delta_2, ptau_g1, ptau_g2, srs_hx_ttau, psi_pub, psi_pvt }
    }
}

pub static GENERATED_SRS: LazyLock<SRS> = LazyLock::new(|| SRS::new());