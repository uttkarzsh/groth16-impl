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

pub fn add_2(a: &[Fr; 2], b: &[Fr; 2]) -> [Fr; 2]{
    [a[0] + b[0], a[1] + b[1]]
}

pub fn hadamard_product(a: &[Fr; 2], b: &[Fr; 2]) -> Fr {
    a[0] * b[0] + a[1] * b[1]
}

pub fn interpolate(matrix: &[[Fr; 3]; 2], column: usize) -> [Fr; 2] {
    let xs: [Fr; 2] = [1u64.into(), 2u64.into()];
    let ys: [Fr; 2] = [matrix[0][column], matrix[1][column]];
    
    [ys[1] - ys[0], Fr::from(2u64) * ys[0] - ys[1]]
}