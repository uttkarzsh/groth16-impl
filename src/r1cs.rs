use ark_ff::Field;

pub type Var = usize;

pub struct LinearTerm<F: Field> {
    pub var: Var,
    pub coeff: F
}

pub struct LinearCombination<F: Field>(pub Vec<LinearTerm<F>>);

pub struct Constraint<F: Field> {
    pub a: LinearCombination<F>,
    pub b: LinearCombination<F>,
    pub c: LinearCombination<F>
}