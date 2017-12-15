pub mod bernoulli;
pub mod uniform;

use std::fmt;

//use var::Stochastic;

///
///
///
pub trait Distribution {
    fn random(&self) -> f64;
    fn logp(&self) -> f64;
}

impl fmt::Debug for Distribution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO Distribution Debug should be implemented")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DistType {
    Discrete,
    Continuous,
}
