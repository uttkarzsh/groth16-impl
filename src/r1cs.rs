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

pub static N: usize = 2;        //number of rows
pub static M: usize = 3;        //number of columns
pub static D: usize = 2 * N - 1;  //polynomial size limit (deg(n) * deg(n) = deg(2n))
pub static L: usize = 1;        //number of public inputs


pub static LEFT_MATRIX: LazyLock<[[Fr; M]; N]> = LazyLock::new(|| fr_matrix![
    [0, 1, 0],
    [8, -6, 1]
]);

pub static RIGHT_MATRIX: LazyLock<[[Fr; M]; N]> = LazyLock::new(|| fr_matrix![
    [0, 1, 0],
    [1, 0, 0]
]);

pub static RESULT_MATRIX: LazyLock<[[Fr; M]; N]> = LazyLock::new(|| fr_matrix![
    [0, 0, 1],
    [0, 0, 0]
]);


//(Lw).(Rw) = (Ow)     
