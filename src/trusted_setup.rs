use ark_bn254::{Fr, G1Projective, G2Projective};
use ark_ff::{UniformRand, Field};
use ark_ec::{CurveGroup, PrimeGroup};
use rand::thread_rng;
use std::sync::LazyLock;
use crate::utils::*;
use crate::curve_ops::{G1, G2};
use crate::r1cs::{N, M, D};

pub struct SRS {
    pub ptau_g1: [G1Projective; D],
    pub ptau_g2: [G2Projective; D],
    pub srs_hx_ttau: [G1Projective; D]
}

impl SRS {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let tau: Fr = Fr::rand(&mut rng);

        let mut ptau: [Fr; D] = [Fr::from(1u64); D];
        let mut ptau_g1: [G1Projective; D] = [*G1; D];
        let mut ptau_g2: [G2Projective; D] = [*G2; D];
        let mut srs_hx_ttau: [G1Projective; D] = [*G1; D];
        
        for i in 0..D {
            ptau[i] = tau.pow([i as u64]);
            ptau_g1[i] = *G1 * ptau[i];
            ptau_g2[i] = *G2 * ptau[i];
        }
        let t_x: [Fr; D] = calculate_tx::<D>();
        let t_tau: Fr = arr_sum(&hadamard_product(&ptau, &t_x));

        for i in 0..D {
            let num: Fr = t_tau * ptau[i];
            srs_hx_ttau[i] = *G1 * num;
        }

        Self { ptau_g1, ptau_g2, srs_hx_ttau }
    }
}

pub static srs: LazyLock<SRS> = LazyLock::new(|| SRS::new());