///
///
///
pub trait Distribution {
    fn random(&self) -> f64;
    fn logp(&self) -> f64;
}

pub trait Discrete: Distribution {
    type Value;

    fn mass(&self, x: Self::Value) -> f64;
}

pub trait Continuous: Distribution {
    fn density(&self) -> f64;
}
