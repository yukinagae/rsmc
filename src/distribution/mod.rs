extern crate probability;

pub mod bernoulli;
pub mod uniform;

use distribution::bernoulli::Bernoulli;
use distribution::uniform::Uniform;
//use var::Stochastic;

///
///
///
// pub trait Distribution {
//     fn random(&self) -> f64;
//     fn logp(&self) -> f64;
// }

pub enum Distribution {
    Ber(Bernoulli),
    Uni(Uniform),
}

impl Distribution {
    fn random(&self) -> f64 {
        match *self {
            Distribution::Uni(ref uniform) => uniform.random(),
            Distribution::Ber(ref bernoulli) => bernoulli.random(),
        }
    }

    fn logp(&self) -> f64 {
        match *self {
            Distribution::Uni(ref uniform) => uniform.logp(),
            Distribution::Ber(ref bernoulli) => bernoulli.logp(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DistType {
    Discrete,
    Continuous,
}
