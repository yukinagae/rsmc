extern crate probability;

use std::fmt;

use self::probability::source;
use self::probability::distribution::Sample;
use self::probability::distribution::Modes;

use distribution::DistType;

///
/// Bernoulli distribution
///
#[derive(Clone)]
pub struct Bernoulli {
    pub dist_type: DistType,
    pub dist: probability::distribution::Bernoulli,
    pub p: f64,
    pub modes: Vec<u8>,
}

impl Bernoulli {
    pub fn new(p: f64) -> Self {
        let dist = probability::distribution::Bernoulli::new(p);
        Self {
            dist_type: DistType::Discrete,
            dist: dist,
            p: p,
            modes: dist.modes(),
        }
    }

    pub fn random(&self) -> f64 {
        let mut source = source::default();
        self.dist.sample(&mut source) as f64
    }

    pub fn logp(&self) -> f64 {
        self.p.ln() // TODO: maybe wrong
    }
}

impl fmt::Debug for Bernoulli {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bernoulli {{ p: {}, modes: {:?}}}", self.p, self.modes)
    }
}
