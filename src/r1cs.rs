use ark_bn254: Fr;

// x^2 + 6x + 8 = 0

// x * x = v
// (v + 6x + 8) * 1 = 0

macro_rules! fr_matrix {
    ($([$( $x: expr),*]),* $(,)?) => {
        [$([$(Fr::from($x as u64)),*]),*]
    };
}

let left_matrix: [[Fr; 3]; 2] = fr_matrix![
    [0, 1, 0],
    [8, 6, 1]
];

let right_matrix: [[Fr; 3]; 2] = fr_matrix![
    [0, 1, 0],
    [1, 0, 0]
];

let multiplied_matrix: [[Fr; 3]; 2] = fr_matrix![
    [0, 0, 1],
    [0, 0, 0]
];

