use ark_bn254::{Fr, G1Projective, G2Projective, Bn254};
use ark_ec::{CurveGroup, PrimeGroup, pairing::Pairing};
use std::sync::LazyLock;

pub static G1: LazyLock<G1Projective> = LazyLock::new(|| G1Projective::generator());
pub static G2: LazyLock<G2Projective> = LazyLock::new(|| G2Projective::generator());

pub fn mul_over_curve() -> bool {
    let double_g1_mul: G1Projective = *G1 * Fr::from(2u64);
    let double_g1_add: G1Projective = *G1 + *G1;

    double_g1_add == double_g1_mul
}

pub fn pairing_check() -> bool {
    let a: Fr = Fr::from(4u64);
    let b: Fr = Fr::from(6u64);
    let c: Fr = Fr::from(24u64);

    let first_pairing = Bn254::pairing(*G1 * a, *G2 * b);
    let second_pairing = Bn254::pairing(*G1 * c, *G2);

    first_pairing == second_pairing

}

pub fn hadamard_g1<const J: usize, const K: usize>(a: &[G1Projective; J], b: &[Fr; K]) -> [G1Projective; K]{
    let mut arr: [G1Projective; K] = [*G1; K];
    for i in 0..K{
        arr[i] = a[i] * b[i];
    }
    arr
}

pub fn hadamard_g2<const J: usize, const K: usize>(a: &[G2Projective; J], b: &[Fr; K]) -> [G2Projective; K]{
    let mut arr: [G2Projective; K] = [*G2; K];
    for i in 0..K{
        arr[i] = a[i] * b[i];
    }
    arr
}

pub fn sum_g1_array<const K: usize>(arr: &[G1Projective; K])->G1Projective{
    let mut sum: G1Projective = *G1 * Fr::from(0u64);
    for i in 0..K {
        sum += arr[i];
    }
    sum
}

pub fn sum_g2_array<const K: usize>(arr: &[G2Projective; K])->G2Projective{
    let mut sum: G2Projective = *G2 * Fr::from(0u64);
    for i in 0..K {
        sum += arr[i];
    }
    sum
}
