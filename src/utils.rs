use ark_bn254:: Fr;

fn check_matrix_eq(a: [Fr; 2], b: [Fr; 2]) -> bool {
    let mut is_equal: bool = true;

    for i in 0..2 {
        if a[i] != b[i] {
            is_equal = false;
        }
    }

    is_equal
}

pub fn matrix_mul(matrix: &[[Fr; 3]; 2], witness: &[Fr; 3]) -> [Fr; 2]{
    let mut out: [Fr; 2] = [0u64.into(), 0u64.into()];
    for i in 0..2 {
        for j in 0..3 {
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

pub fn polynomial_multiplication(a: &[Fr; 2], b: &[Fr; 2]) -> [Fr; 3] {
    [a[0] * b[0], a[0] * b[1] + a[1] * b[0], a[1] * b[1]]
}

fn interpolate(matrix: &[[Fr; 3]; 2], column: usize) -> [Fr; 2] {
    let xs: [Fr; 2] = [1u64.into(), 2u64.into()];
    let ys: [Fr; 2] = [matrix[0][column], matrix[1][column]];
    
    [Fr::from(2u64) * ys[0] - ys[1], ys[1] - ys[0]]
}

pub fn interpolate_matrix(matrix: &[[Fr; 3]; 2]) -> [[Fr; 2]; 3] {
    [interpolate(matrix, 0), interpolate(matrix, 1), interpolate(matrix, 2)]
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