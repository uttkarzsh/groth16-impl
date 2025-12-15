use ark_bn254:: Fr;
use std::sync::LazyLock;

pub static WITNESS: LazyLock<[Fr; 3]> = LazyLock::new(||[Fr::from(1u64), Fr::from(2u64), Fr::from(4u64)]);