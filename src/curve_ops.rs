use ark_ec::{CurveGroup, PrimeGroup};
use crate::types::*;
use std::sync::LazyLock;

pub static G1: LazyLock<G1Point> = LazyLock::new(|| G1Point::generator());
pub static G2: LazyLock<G2Point> = LazyLock::new(|| G2Point::generator());

pub fn multiply<G: CurveGroup, const N: usize, const M: usize>(
    a: &[G; N], 
    b: &[G::ScalarField; M]
) -> [G; M] 
where
    G: Copy,
{
    assert!(M <= N, "M must be less than or equal to N");
    let mut arr: [G; M] = [a[0]; M];
    for i in 0..M {
        arr[i] = a[i] * b[i];
    }
    arr
}

pub fn sum_group_elements<G: CurveGroup, const N: usize>(arr: &[G; N])->G where G: Copy, {
    let mut sum: G = G::zero();
    for i in 0..N {
        sum += arr[i];
    }
    sum
}

// pub fn mul_over_curve() -> bool {
//     let double_g1_mul: G1Projective = *G1 * Fr::from(2u64);
//     let double_g1_add: G1Projective = *G1 + *G1;

//     double_g1_add == double_g1_mul
// }

// pub fn pairing_check() -> bool {
//     let a: Fr = Fr::from(4u64);
//     let b: Fr = Fr::from(6u64);
//     let c: Fr = Fr::from(24u64);

//     let first_pairing = Bn254::pairing(*G1 * a, *G2 * b);
//     let second_pairing = Bn254::pairing(*G1 * c, *G2);

//     first_pairing == second_pairing

// }
