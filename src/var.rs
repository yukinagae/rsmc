extern crate autograd as ag;

use std::ops::{Add, Mul};

//#[derive(Debug)]
pub enum Variable {
    FreeRV(ag::Tensor),
    //TransformedRV,
    //MultiObservedRV,
    ObservedRV(ag::Tensor),
}

impl Add for Variable {
    type Output = Variable;

    fn add(self, other: Variable) -> Variable {
        use self::Variable::{FreeRV, ObservedRV};
        match (self, other) {
            (FreeRV(t1), FreeRV(t2)) => FreeRV(t1 + t2),
            (FreeRV(t1), ObservedRV(t2)) => FreeRV(t1 + t2),
            (ObservedRV(t1), FreeRV(t2)) => FreeRV(t1 + t2),
            (ObservedRV(t1), ObservedRV(t2)) => FreeRV(t1 + t2),
        }
    }
}

impl Mul for Variable {
    type Output = Variable;

    fn mul(self, other: Variable) -> Variable {
        use self::Variable::{FreeRV, ObservedRV};
        match (self, other) {
            (FreeRV(t1), FreeRV(t2)) => FreeRV(t1 * t2),
            (FreeRV(t1), ObservedRV(t2)) => FreeRV(t1 * t2),
            (ObservedRV(t1), FreeRV(t2)) => FreeRV(t1 * t2),
            (ObservedRV(t1), ObservedRV(t2)) => FreeRV(t1 * t2),
        }
    }
}
