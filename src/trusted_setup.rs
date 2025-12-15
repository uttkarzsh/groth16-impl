use ark_bn254::{Fr, G1Projective, G2Projective};
use ark_ff::{UniformRand, Field};
use ark_ec::{CurveGroup, PrimeGroup};
use rand::thread_rng;
use std::sync::LazyLock;
use crate::utils::*;
use crate::curve_ops::{G1, G2};

pub struct SRS <const N: usize> {
    pub ptau_g1: [G1Projective; N],
    pub ptau_g2: [G2Projective; N],
    pub srs_hx_ttau: [G1Projective; N]
}

impl <const N: usize> SRS <N> {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let tau: Fr = Fr::rand(&mut rng);

        let mut ptau: [Fr; N] = [Fr::from(1u64); N];
        let mut ptau_g1: [G1Projective; N] = [*G1; N];
        let mut ptau_g2: [G2Projective; N] = [*G2; N];
        let mut srs_hx_ttau: [G1Projective; N] = [*G1; N];
        
        for i in 0..N {
            ptau[i] = tau.pow([i as u64]);
        }
        let t_x: [Fr; N] = calculate_tx::<N>();
        let t_tau: Fr = arr_sum(&hadamard_product(&ptau, &t_x));

        for i in 0..N {
            let num: Fr = t_tau * ptau[i];
            srs_hx_ttau[i] = *G1 * num;
        }

        for i in 0..N-1 {
            ptau_g1[i] = *G1 * ptau[i];
            ptau_g2[i] = *G2 * ptau[i];
        }

        Self { ptau_g1, ptau_g2, srs_hx_ttau }
    }
}

pub static srs: LazyLock<SRS<3>> = LazyLock::new(|| SRS::<3>::new());