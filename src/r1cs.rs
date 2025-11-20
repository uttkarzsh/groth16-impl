use ark_ff: Field;

// x^2 + 6x + 8 = 0

// x * x = v
// (v + 6x + 8) * 1 = 0

let left_matrix: [[Field; 3]; 2] = [
    [0, 1, 0],
    [8, 6, 1]
];

let right_matrix: [[Field; 3]; 2] = [
    [0, 1, 0],
    [1, 0, 0]
];

let multiplied_matrix: [[Field; 3]; 2] = [
    [0, 0, 1],
    [0, 0, 0]
];

