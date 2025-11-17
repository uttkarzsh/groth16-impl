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

pub struct R1CS<F: Field> {
    pub constraints: Vec<Constraint<F>>,
    pub num_vars: usize
}

impl<F: Field> R1CS<F>{
    pub fn new()-> Self{
        Self {constraints: Vec[]!, num_vars: 1}
    }

    pub fn new_var(&mut self)-> Var {
        self.num_vars += 1;
        self.num_vars - 1
    }

    pub fn add_constraint(&mut self, a: LinearCombination<F>, b: LinearCombination<F>, c: LinearCombination<F>){
        self.constraints.push(Constraint {a, b, c});
    }
}