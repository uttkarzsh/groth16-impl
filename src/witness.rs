use crate::r1cs::{M};
use crate::types::*;
use std::sync::LazyLock;

pub static WITNESS: LazyLock<[Field; M]> = LazyLock::new(||[Field::from(1u64), Field::from(2u64), Field::from(4u64)]);