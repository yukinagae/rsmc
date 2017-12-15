extern crate probability;

use std::fmt;

use self::probability::source;
use self::probability::distribution::Sample;
use self::probability::distribution::Mean;
use self::probability::distribution::Median;

use distribution::Distribution;
use distribution::DistType;

///
/// Uniform distribution
///
#[derive(Clone, Copy)]
pub struct Uniform {
    pub dist_type: DistType,
    pub dist: probability::distribution::Uniform,
    pub lower: f64,
    pub upper: f64,
    pub mean: f64,
    pub median: f64,
}

impl fmt::Debug for Uniform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Uniform {{ a: {}, b: {} }}", self.lower, self.upper)
    }
}

impl Uniform {
    pub fn new(lower: f64, upper: f64) -> Self {
        let dist = probability::distribution::Uniform::new(lower, upper);
        Self {
            dist_type: DistType::Continuous,
            dist: dist,
            lower: lower,
            upper: upper,
            mean: dist.mean(),
            median: dist.median(),
        }
    }
}

impl Distribution for Uniform {
    fn random(&self) -> f64 {
        let mut source = source::default();
        self.dist.sample(&mut source)
    }

    fn logp(&self) -> f64 {
        (self.upper - self.lower).ln()
    }
}
