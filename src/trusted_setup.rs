use ark_bn254::Fr;
use ark_ff::UniformRand;
use rand::thread_rng;

pub struct SRS {
    pub tau: Fr,
    pub ptau: [Fr; 2],
}

impl SRS {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let tau: Fr = Fr::rand(&mut rng);
        let ptau: [Fr; 2] = [tau, Fr::from(0u64)];
        Self { tau, ptau }
    }
}

pub fn generate_srs() -> SRS {
    let mut srs = SRS::new();
    srs
}