#![allow(unused)]

mod utils;
mod r1cs;
mod witness;
mod trusted_setup;
mod qap;
mod curve_ops;
mod proof;
mod verification;
mod types;

use r1cs::{L};
use qap::{QAP_FOR_PROOF};
use proof::{Proof};
use verification::{verify_proof};
use types::*;

fn main() {

    let pub_inputs: [Field; L] = [Field::from(1u32)];
    let proof: Proof = Proof::generate_proof(&*QAP_FOR_PROOF);

    let verification_successful: bool = verify_proof(&proof, &pub_inputs);

    if verification_successful {
        println!("witness correct yay");
    } else {
        println!("wrong witness lol");
    }

}
