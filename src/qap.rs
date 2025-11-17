use ark_ff::Field;
use ark_poly::{Polynomial}

use crate::r1cs::*;

pub struct QAP<F: Field>{
    pub a: Vec<Polynomial<F>>,
    pub b: Vec<Polynomial<F>>,
    pub c: Vec<Polynomial<F>>
}

pub fun r1cs_to_qap<F: Field>(r1cs: &R1CS<F>) -> QAP<F> {
    let n = r1cs.constraints.len();
}