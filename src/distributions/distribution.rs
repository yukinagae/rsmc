///
///
///
pub trait Distribution {
    type Value;

    fn random(&self) -> Self::Value;
    fn logp(&self) -> f64;
}

pub trait Discrete: Distribution {
    fn mass(&self, x: Self::Value) -> f64;
}

pub trait Continuous: Distribution {
    fn density(&self, x: f64) -> f64;
}
