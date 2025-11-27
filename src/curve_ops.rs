use ark_bn254::{Fr, G1Projective, G2Projective, Bn254};
use ark_ec::{CurveGroup, PrimeGroup, pairing::Pairing};
use std::sync::LazyLock;

static G1: LazyLock<G1Projective> = LazyLock::new(|| G1Projective::generator());
static G2: LazyLock<G2Projective> = LazyLock::new(|| G2Projective::generator());

pub fn mul_over_curve() -> bool {
    let doubleG1_mul: G1Projective = *G1 * Fr::from(2u64);
    let doubleG1_add: G1Projective = *G1 + *G1;

    doubleG1_add == doubleG1_mul
}

pub fn pairing_check() -> bool {
    let a: Fr = Fr::from(4u64);
    let b: Fr = Fr::from(6u64);
    let c: Fr = Fr::from(24u64);

    let first_pairing = Bn254::pairing(*G1 * a, *G2 * b);
    let second_pairing = Bn254::pairing(*G1 * c, *G2);

    first_pairing == second_pairing

}
