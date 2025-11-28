use ark_bn254::Fr;
use ark_ff::{UniformRand, Field};
use rand::thread_rng;

pub struct SRS <const N: usize> {
    pub tau: Fr,
    pub ptau: [Fr; N],
}

impl <const N: usize> SRS <N> {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let tau: Fr = Fr::rand(&mut rng);
        let mut ptau: [Fr; N] = [Fr::from(1u64); N];
        for i in 0..N {
            ptau[i] = tau.pow([i as u64]);
        }
        Self { tau, ptau }
    }
}

pub fn generate_srs<const N: usize>() -> SRS<N> {
    let mut srs = SRS::<N>::new();
    srs
}