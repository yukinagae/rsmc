pub mod bernoulli;
pub mod uniform;
pub mod normal;

use distribution::bernoulli::Bernoulli;
use distribution::uniform::Uniform;
use distribution::normal::Normal;
use distribution::normal::HalfNormal;

#[derive(Debug)]
pub enum Distribution {
    Ber(Bernoulli),
    Uni(Uniform),
    Nor(Normal),
    HNor(HalfNormal),
}

#[allow(unused_variables)]
#[allow(dead_code)]
impl Distribution {
    fn random(&self) -> f64 {
        use self::Distribution::{Ber, HNor, Nor, Uni};
        match *self {
            Uni(ref uniform) => uniform.random(),
            Ber(ref bernoulli) => bernoulli.random(),
            Nor(ref normal) => normal.random(),
            HNor(ref half_normal) => half_normal.random(),
        }
    }

    fn logp(&self) -> f64 {
        use self::Distribution::{Ber, HNor, Nor, Uni};
        match *self {
            Uni(ref uniform) => uniform.logp(),
            Ber(ref bernoulli) => bernoulli.logp(),
            Nor(ref normal) => normal.logp(),
            HNor(ref half_normal) => half_normal.logp(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DistType {
    Discrete,
    Continuous,
}
