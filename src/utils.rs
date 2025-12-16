use ark_bn254:: Fr;
use ark_ff::Field;

fn check_matrix_eq<const N: usize>(a: [Fr; N], b: [Fr; N]) -> bool {
    let mut is_equal: bool = true;

    for i in 0..N {
        if a[i] != b[i] {
            is_equal = false;
        }
    }

    is_equal
}

pub fn matrix_mul<const N: usize, const M: usize>(matrix: &[[Fr; M]; N], witness: &[Fr; M]) -> [Fr; N]{
    let mut out: [Fr; N] = [Fr::from(0u64); N];
    for i in 0..N {
        for j in 0..M {
            out[i] += matrix[i][j] * witness[j];
        }
    }
    out
}

pub fn add<const N: usize>(a: &[Fr; N], b: &[Fr; N]) -> [Fr; N]{
    let mut arr: [Fr; N] = [Fr::from(0u64); N];
    for i in 0..N{
        arr[i] = a[i] + b[i];
    }
    arr
}

pub fn sub<const N: usize>(a: &[Fr; N], b: &[Fr; N]) -> [Fr; N]{
    let mut arr: [Fr; N] = [Fr::from(0u64); N];
    for i in 0..N{
        arr[i] = a[i] - b[i];
    }
    arr
}

pub fn scalar_mul<const N: usize>(matrix: &[Fr; N], scalar: Fr) -> [Fr; N] {
    let mut arr: [Fr; N] = [Fr::from(0u64); N];
    for i in 0..N{
        arr[i] = scalar * matrix[i];
    }
    arr
}

pub fn hadamard_product<const N: usize>(a: &[Fr; N], b: &[Fr; N]) -> [Fr; N]{
    let mut arr: [Fr; N] = [Fr::from(0u64); N];
    for i in 0..N{
        arr[i] = a[i] * b[i];
    }
    arr
}

pub fn arr_sum<const N: usize>(arr: &[Fr; N]) -> Fr{
    let mut sum: Fr = Fr::from(0u64);
    for i in 0..N{
        sum += arr[i];
    }
    sum
}

fn polynomial_multiplication<const N: usize, const M: usize>(a: &[Fr; N], b: &[Fr; N]) -> [Fr; M]{
    let mut product: [Fr; M] = [Fr::from(0u64); M];
    for i in 0..N {
        for j in 0..N{
            product[j + i] += a[i] * b[j];
        }
    }
    product
}

fn lagrange_interpolation<const N: usize>(xs: &[Fr; N], ys: &[Fr; N]) -> [Fr; N] {
    let mut result = [Fr::from(0u64); N];
    
    for i in 0..N {
        let mut basis = [Fr::from(0u64); N];
        basis[0] = Fr::from(1u64);
        
        let mut denom = Fr::from(1u64);
        for j in 0..N {
            if i != j {
                denom *= xs[i] - xs[j];
            }
        }
        let denom_inv = denom.inverse().unwrap();
        
        for j in 0..N {
            if i != j {
                let mut new_basis = [Fr::from(0u64); N];
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

fn interpolate<const N: usize, const M:usize>(matrix: &[[Fr; M]; N], column: usize) -> [Fr; N] {
    let mut xs: [Fr; N] = [Fr::from(0u32); N];
    let mut ys: [Fr; N] = [Fr::from(0u32); N];
    
    for i in 1..=N{
        xs[i-1] = Fr::from(i as u64);
        ys[i-1] = matrix[i-1][column];
    }

    lagrange_interpolation(&xs, &ys)
}

pub fn interpolate_matrix<const N: usize, const M:usize>(matrix: &[[Fr; M]; N]) -> [[Fr; N]; M] {
    let mut interpolated_matrix: [[Fr; N]; M] = [[Fr::from(0u64); N]; M];
    for i in 0..M{
        interpolated_matrix[i] = interpolate(matrix, i);
    }

    interpolated_matrix
}

pub fn polynomial_division<const N: usize>(px: &[Fr; N], qx: &[Fr; N], deg_p: usize, deg_q: usize) -> [Fr; N] {
    let mut res: [Fr; N] = [Fr::from(0u64); N];
    let mut zx: [Fr; N] = *px;
    let mut deg_z: usize = deg_p;
    let iterations: usize = deg_p - deg_q + 1;

    for i in 0..iterations{
        let factor: Fr = zx[deg_z] / qx[deg_q];
        res[iterations - 1 - i] = factor;

        for j in 0..=deg_q{
            zx[deg_z - j] -= factor * qx[deg_q - j];
        }
        deg_z -= 1;
    }

    res
}

pub fn calculate_tx<const N: usize>() -> [Fr; N] {
    let mut t_x: [Fr; N] = [Fr::from(0u64); N];
    t_x[0] = Fr::from(1u64);

    for i in 1..N {
        let mut q: [Fr; N] = [Fr::from(0u64); N];

        for k in 0..i {
            q[k] += Fr::from(-1i64) * Fr::from(i as u64) * t_x[k];
            q[k+1] += t_x[k];
        }

        t_x = q;
    }

    t_x
}

pub fn calculate_hx(u_x: &[Fr; 2], v_x: &[Fr; 2], w_x: &[Fr; 2], t_x: &[Fr; 3]) -> [Fr; 3]{
    let uv_x2: [Fr; 3] = polynomial_multiplication(&u_x, &v_x);
    let w_x2: [Fr;3] = [w_x[0], w_x[1],Fr::from(0u64)];

    let uv_minus_w: [Fr; 3] = sub(&uv_x2, &w_x2);
    polynomial_division(&uv_minus_w, &t_x, 2, 2)
} 