extern crate probability;

use self::probability::source;
use self::probability::distribution::Sample;
use self::probability::distribution::Modes;
use self::probability::distribution::Discrete as _Discrete;

use distribution::Distribution;
use distribution::Discrete;

///
/// Bernoulli distribution
///
#[derive(Clone)]
pub struct Bernoulli {
    pub dist: probability::distribution::Bernoulli,
    // TODO: below f
    pub p: f64,
    pub modes: Vec<u8>,
}

impl Bernoulli {
    pub fn new(p: f64) -> Self {
        let dist = probability::distribution::Bernoulli::new(p);
        Self {
            dist: dist,
            p: p,
            modes: dist.modes(),
        }
    }
}

impl Distribution for Bernoulli {
    fn random(&self) -> f64 {
        let mut source = source::default();
        self.dist.sample(&mut source) as f64
    }

    fn logp(&self) -> f64 {
        self.p.ln() // TODO: maybe wrong
    }
}

impl Discrete for Bernoulli {
    fn mass(&self, x: f64) -> f64 {
        self.dist.mass(x as u8) as f64
    }
}
