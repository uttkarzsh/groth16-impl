mod r1cs;
mod utils;
mod witness;
mod trusted_setup;

use utils::{interpolate_matrix, scalar_mul, add_2};
use r1cs::{LEFT_MATRIX, RIGHT_MATRIX, RESULT_MATRIX};
use witness::WITNESS;

pub fn qap_representation(matrix: &[[Fr; 3]; 2], witness: &[Fr; 3]) -> [Fr; 2] {
    let matn_x: [[Fr; 2]; 3] = interpolate_matrix(matrix);
    let mut a_matn_x: [Fr; 2] = scalar_mul(matn_x[0], witness[0]);

    for i in 1..3 {
        a_matn_x = add_2(a_matn_x, scalar_mul(matn_x[i], witness[i]));
    }

    a_matn_x
}

pub static u_x = qap_representation(&LEFT_MATRIX, &WITNESS);
pub static v_x = qap_representation(&RIGHT_MATRIX, &WITNESS);
pub static w_x = qap_representation(&RESULT_MATRIX, &WITNESS);



