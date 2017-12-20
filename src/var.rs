extern crate autograd as ag;

use std::ops::Add;

#[derive(Debug)]
pub enum Variable {
    FreeRV,
    //TransformedRV,
    //MultiObservedRV,
    //ObservedRV,
}

impl Add for Variable {
    type Output = Variable;

    fn add(self, other: Variable) -> Variable {
        use self::Variable::FreeRV;
        match (self, other) {
            (FreeRV, FreeRV) => FreeRV,
        }
    }
}
