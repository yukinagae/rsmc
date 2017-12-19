use std::ops::Add;

pub enum Variable {
    Stochastic,
    Deterministic,
    Potential,
}

impl Add for Variable {
    type Output = Variable;

    fn add(self, other: Variable) -> Variable {
        Variable::Deterministic
    }
}

pub struct FreeRV {
    total_size: usize,
}

#[derive(Debug)]
pub struct Expression {}
