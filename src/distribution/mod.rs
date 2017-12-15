pub mod bernoulli;
pub mod uniform;

///
///
///
pub trait Distribution {
    fn random(&self) -> f64;
    fn logp(&self) -> f64;
}

pub trait Discrete: Distribution {
    fn mass(&self, x: f64) -> f64;
}

pub trait Continuous: Distribution {
    fn density(&self, x: f64) -> f64;
}
