extern crate rand;

use self::rand::Rng;

///
///
///
pub trait Distribution {
    fn random(&self) -> f64;
}

///
///
///
pub trait Continuous: Distribution {
    fn density(&self) -> f64;
}

///
///
///
pub trait Discrete: Distribution {
    type Value;

    fn mass(&self, x: Self::Value) -> f64;
}

///
/// Uniform distribution
///
#[derive(Debug)]
pub struct Uniform {
    pub lower: f64,
    pub upper: f64,
    pub mean: f64,
    pub median: f64,
}

impl Distribution for Uniform {
    fn random(&self) -> f64 {
        rand::thread_rng().gen_range(self.lower, self.upper)
    }
}

impl Continuous for Uniform {
    fn density(&self) -> f64 {
        1.0 // TODO: not yet
    }
}

///
///
///
pub struct Bernoulli {
    pub p: f64,
    pub mode: f64,
}

impl Distribution for Bernoulli {
    fn random(&self) -> f64 {
        // rand::thread_rng().gen_range(self.lower, self.upper)
        1.0 // TODO: not yet
    }
}

impl Discrete for Bernoulli {
    type Value = bool;

    fn mass(&self, x: bool) -> f64 {
        if x {
            self.p
        } else {
            1.0 - self.p
        }
    }
}

#[test]
fn it_works() {
    assert!(true)
}
