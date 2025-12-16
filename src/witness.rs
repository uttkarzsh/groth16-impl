use ark_bn254:: Fr;
use crate::r1cs::{M};
use std::sync::LazyLock;

pub static WITNESS: LazyLock<[Fr; M]> = LazyLock::new(||[Fr::from(1u64), Fr::from(2u64), Fr::from(4u64)]);