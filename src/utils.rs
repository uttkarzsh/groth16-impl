use crate::types::*;
use ark_ff::{Field as IField, Zero, One};

// fn check_matrix_eq<const N: usize>(a: [Field; N], b: [Field; N]) -> bool {
//     let mut is_equal: bool = true;

//     for i in 0..N {
//         if a[i] != b[i] {
//             is_equal = false;
//         }
//     }

//     is_equal
// }

// pub fn matrix_mul<const N: usize, const M: usize>(matrix: &[[Field; M]; N], witness: &[Field; M]) -> [Field; N]{
//     let mut out: [Field; N] = [Field::zero(); N];
//     for i in 0..N {
//         for j in 0..M {
//             out[i] += matrix[i][j] * witness[j];
//         }
//     }
//     out
// }

pub fn add<const N: usize>(a: &[Field; N], b: &[Field; N]) -> [Field; N]{
    let mut arr: [Field; N] = [Field::zero(); N];
    for i in 0..N{
        arr[i] = a[i] + b[i];
    }
    arr
}

pub fn add_three_arr<const N: usize>(a: &[Field; N], b: &[Field; N], c: &[Field; N]) -> [Field; N]{
    let mut arr: [Field; N] = [Field::zero(); N];
    for i in 0..N{
        arr[i] = a[i] + b[i] + c[i];
    }
    arr
}

pub fn sub<const N: usize>(a: &[Field; N], b: &[Field; N]) -> [Field; N]{
    let mut arr: [Field; N] = [Field::zero(); N];
    for i in 0..N{
        arr[i] = a[i] - b[i];
    }
    arr
}

pub fn scalar_mul<const N: usize>(matrix: &[Field; N], scalar: Field) -> [Field; N] {
    let mut arr: [Field; N] = [Field::zero(); N];
    for i in 0..N{
        arr[i] = scalar * matrix[i];
    }
    arr
}

pub fn hadamard_product<const N: usize, const M: usize>(a: &[Field; N], b: &[Field; M]) -> [Field; M]{
    let mut arr: [Field; M] = [Field::zero(); M];
    for i in 0..M{
        arr[i] = a[i] * b[i];
    }
    arr
}

pub fn arr_sum<const N: usize>(arr: &[Field; N]) -> Field{
    let mut sum: Field = Field::zero();
    for i in 0..N{
        sum += arr[i];
    }
    sum
}

fn polynomial_multiplication<const N: usize, const M: usize>(a: &[Field; N], b: &[Field; N]) -> [Field; M]{
    let mut product: [Field; M] = [Field::zero(); M];
    for i in 0..N {
        for j in 0..N{
            product[j + i] += a[i] * b[j];
        }
    }
    product
}

fn lagrange_interpolation<const N: usize>(xs: &[Field; N], ys: &[Field; N]) -> [Field; N] {
    let mut result = [Field::zero(); N];
    
    for i in 0..N {
        let mut basis = [Field::zero(); N];
        basis[0] = Field::one();
        
        let mut denom = Field::one();
        for j in 0..N {
            if i != j {
                denom *= xs[i] - xs[j];
            }
        }
        let denom_inv = denom.inverse().unwrap();
        
        for j in 0..N {
            if i != j {
                let mut new_basis = [Field::zero(); N];
                for k in 0..N {
                    if k > 0 {
                        new_basis[k] += basis[k - 1]; 
                    }
                    new_basis[k] -= basis[k] * xs[j]; 
                }
                basis = new_basis;
            }
        }
        for k in 0..N {
            result[k] += basis[k] * ys[i] * denom_inv;
        }
    }
    
    result
}

fn interpolate<const N: usize, const M:usize>(matrix: &[[Field; M]; N], column: usize) -> [Field; N] {
    let mut xs: [Field; N] = [Field::zero(); N];
    let mut ys: [Field; N] = [Field::zero(); N];
    
    for i in 1..=N{
        xs[i-1] = Field::from(i as u64);
        ys[i-1] = matrix[i-1][column];
    }

    lagrange_interpolation(&xs, &ys)
}

pub fn interpolate_matrix<const N: usize, const M:usize>(matrix: &[[Field; M]; N]) -> [[Field; N]; M] {
    let mut interpolated_matrix: [[Field; N]; M] = [[Field::zero(); N]; M];
    for i in 0..M{
        interpolated_matrix[i] = interpolate(matrix, i);
    }

    interpolated_matrix
}

pub fn polynomial_division<const N: usize>(px: &[Field; N], qx: &[Field; N], deg_p: usize, deg_q: usize) -> [Field; N] {
    let mut res: [Field; N] = [Field::zero(); N];
    let mut zx: [Field; N] = *px;
    let mut deg_z: usize = deg_p;
    let iterations: usize = deg_p - deg_q + 1;

    for i in 0..iterations{
        let factor: Field = zx[deg_z] / qx[deg_q];
        res[iterations - 1 - i] = factor;

        for j in 0..=deg_q{
            zx[deg_z - j] -= factor * qx[deg_q - j];
        }
        deg_z -= 1;
    }

    res
}

pub fn calculate_tx<const N: usize>() -> [Field; N] {
    let mut t_x: [Field; N] = [Field::zero(); N];
    t_x[0] = Field::one();

    for i in 1..N {
        let mut q: [Field; N] = [Field::zero(); N];

        for k in 0..i {
            q[k] += Field::from(-1i64) * Field::from(i as u64) * t_x[k];
            q[k+1] += t_x[k];
        }

        t_x = q;
    }

    t_x
}

pub fn calculate_hx(u_x: &[Field; 2], v_x: &[Field; 2], w_x: &[Field; 2], t_x: &[Field; 3]) -> [Field; 3]{
    let uv_x2: [Field; 3] = polynomial_multiplication(&u_x, &v_x);
    let w_x2: [Field;3] = [w_x[0], w_x[1],Field::zero()];

    let uv_minus_w: [Field; 3] = sub(&uv_x2, &w_x2);
    polynomial_division(&uv_minus_w, &t_x, 2, 2)
} 

pub fn qap_representation<const N: usize, const M: usize>(matrix: &[[Field; M]; N], vector: &[Field; M]) -> [Field; N] {
    let matn_x: [[Field; N]; M] = interpolate_matrix(&matrix);
    let mut a_matn_x: [Field; N] = scalar_mul(&matn_x[0], vector[0]);

    for i in 1..M {
        a_matn_x = add(&a_matn_x, &scalar_mul(&matn_x[i], vector[i]));
    }

    a_matn_x
}