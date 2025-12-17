#![allow(unused)]

mod utils;
mod r1cs;
mod witness;
mod trusted_setup;
mod qap;
mod curve_ops;
mod proof;
mod verification;

use ark_bn254:: {Fr, G1Projective, G2Projective, Bn254};
use ark_ec:: {CurveGroup, PrimeGroup, pairing::Pairing};
use r1cs::{LEFT_MATRIX, RIGHT_MATRIX, RESULT_MATRIX, N, M, D, L};
use witness::{WITNESS};
use utils::*;
use qap::{QAP, QAP_FOR_PROOF};
use trusted_setup:: {SRS, srs};
use proof::{Proof};
use curve_ops::*;
use verification::{verify_proof};


fn main() {

    /* 
    let l_w: [Fr; 2] = matrix_mul(&LEFT_MATRIX, &WITNESS);
    let r_w: [Fr; 2] = matrix_mul(&RIGHT_MATRIX, &WITNESS);
    let o_w: [Fr; 2] = matrix_mul(&RESULT_MATRIX, &WITNESS);

    let verification_successful: bool = check_matrix_eq(hadamard_product(&l_w, &r_w), o_w);
    */

    
    ///false proof generation
    
    // let a: G1Projective = *G1 * Fr::from(2u64);
    // let b: G2Projective = *G2 * Fr::from(5u32);
    // let c: G1Projective = *G1 * Fr::from(10u32);

    // let proof: Proof = Proof{
    //     A: a,
    //     B: b,
    //     C: c
    // };

    let pub_inputs: [Fr; L] = [Fr::from(1u32)];
    let proof: Proof = Proof::new(&*QAP_FOR_PROOF);

    let verification_successful: bool = verify_proof(&proof, &pub_inputs);

    if verification_successful {
        println!("witness correct yay");
    } else {
        println!("wrong witness lol");
    }

    // check polynomials
    // println!("{} and {}", U_X[0], U_X[1]);
    // println!("{} and {}", V_X[0], V_X[1]);
    // println!("{} and {}", W_X[0], W_X[1]);
    // println!("{} and {} and {}", T_X[0], T_X[1], T_X[2]);
    // println!("{} and {} and {}", H_X[0], H_X[1], H_X[2]);
    

    // let mul_working: bool = mul_over_curve();
    // println!("{}", mul_working);

    // let pair_check: bool = pairing_check();
    // println!("{}", pair_check);
}
