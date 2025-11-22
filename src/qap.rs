mod r1cs;
mod utils;
mod witness;

use utils::{interpolate_matrix, scalar_mul, add_2};
use r1cs::{LEFT_MATRIX, RIGHT_MATRIX, RESULT_MATRIX};
use witness::WITNESS;

pub fn qap_representation(matrix: &[[Fr; 3]; 2], witness: &[Fr; 3]) -> [Fr; 2] {
    let un_x: [[Fr; 2]; 3] = interpolate_matrix(matrix);
    let mut a_un_x: [Fr; 2] = scalar_mul(un_x[0], witness[0]);

    for i in 1..3 {
        a_un_x = add_2(a_un_x, scalar_mul(un_x[i], witness[i]));
    }

    a_un_x
}