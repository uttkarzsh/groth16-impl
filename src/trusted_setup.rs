use ark_bn254::{Fr, G1Projective, G2Projective};
use ark_ff::{UniformRand, Field};
use ark_ec::{CurveGroup, PrimeGroup};
use rand::thread_rng;
use crate::utils::*;
use crate::curve_ops::{G1, G2};

pub struct SRS <const N: usize> {
    pub ptau_g1: [G1Projective; N],
    pub ptau_g2: [G2Projective; N],
}

impl <const N: usize> SRS <N> {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let tau: Fr = Fr::rand(&mut rng);
        let mut ptau: [Fr; N] = [Fr::from(1u64); N];
        for i in 0..N {
            ptau[i] = tau.pow([i as u64]);
        }
        let mut ptau_g1: [G1Projective; N] = [*G1; N];
        
        for i in 0..N {
            ptau_g1[i] = *G1 * ptau[i];
        }

        let mut ptau_g2: [G2Projective; N] = [*G2; N];
        
        for i in 0..N {
            ptau_g2[i] = *G2 * ptau[i];
        }

        Self { ptau_g1, ptau_g2 }
    }
}

pub fn generate_srs<const N: usize>() -> SRS<N> {
    let mut srs = SRS::<N>::new();
    srs
}