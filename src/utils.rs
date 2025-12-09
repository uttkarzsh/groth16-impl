use ark_bn254:: Fr;

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