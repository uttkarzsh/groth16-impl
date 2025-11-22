use ark_bn254::Fr;
use std::sync::LazyLock;

/* 

x^2 - 6x + 8 = 0

constraint 1: x * x = v
constraint 2: (v - 6x + 8) * 1 = 0

*/

macro_rules! fr_matrix {
    ($([$( $x: expr ),*]),* $(,)?) => {
        [
            $([$( Fr::from($x as i64) ),*]),*
        ]
    };
}

pub static LEFT_MATRIX: LazyLock<[[Fr; 3]; 2]> = LazyLock::new(|| fr_matrix![
    [0, 1, 0],
    [8, -6, 1]
]);

pub static RIGHT_MATRIX: LazyLock<[[Fr; 3]; 2]> = LazyLock::new(|| fr_matrix![
    [0, 1, 0],
    [1, 0, 0]
]);

pub static RESULT_MATRIX: LazyLock<[[Fr; 3]; 2]> = LazyLock::new(|| fr_matrix![
    [0, 0, 1],
    [0, 0, 0]
]);