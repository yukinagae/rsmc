extern crate probability;

use self::probability::source;
use self::probability::distribution::Sample;
use self::probability::distribution::Mean;
use self::probability::distribution::Median;
use self::probability::distribution::Continuous as _Continuous;

use distribution::Distribution;
use distribution::Continuous;

///
/// Uniform distribution
///
#[derive(Clone, Copy)]
pub struct Uniform<'a> {
    pub name: &'a str,
    pub dist: probability::distribution::Uniform,
    // TODO: below fields may be redundant, those can be replaced by getter functions.
    pub lower: f64,
    pub upper: f64,
    pub mean: f64,
    pub median: f64,
}

impl<'a> Uniform<'a> {
    pub fn new(name: &'a str, lower: f64, upper: f64) -> Self {
        let dist = probability::distribution::Uniform::new(lower, upper);
        Self {
            name: name,
            dist: dist,
            lower: lower,
            upper: upper,
            mean: dist.mean(),
            median: dist.median(),
        }
    }
}

impl<'a> Distribution for Uniform<'a> {
    fn random(&self) -> f64 {
        let mut source = source::default();
        self.dist.sample(&mut source)
    }

    fn logp(&self) -> f64 {
        (self.upper - self.lower).ln()
    }

    fn name(&self) -> &str {
        self.name
    }
}

impl<'a> Continuous for Uniform<'a> {
    fn density(&self, x: f64) -> f64 {
        self.dist.density(x)
    }
}
