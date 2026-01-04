use ark_bn254::{Bn254};
use ark_bls12_381::{Bls12_381};
use ark_bls12_377::{Bls12_377};
use ark_bw6_761::{BW6_761};
use ark_ec::{pairing::Pairing};

pub type Curve = Bn254;
pub type Field = <Curve as Pairing>::ScalarField;
pub type G1Point = <Curve as Pairing>::G1;
pub type G2Point = <Curve as Pairing>::G2;
