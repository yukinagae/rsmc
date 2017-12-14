extern crate probability;
extern crate rand;


//use self::rand::Rng;

use self::probability::source;
use self::probability::sampler::Independent;
use self::probability::distribution::Mean;
use self::probability::distribution::Median;

use distributions::distribution::Distribution;
use distributions::distribution::Continuous;

///
/// Uniform distribution
///
#[derive(Clone, Copy)]
pub struct Uniform {
    pub dist: probability::distribution::Uniform,
    // TODO: below fields may be redundant, those can be replaced by getter functions.
    pub lower: f64,
    pub upper: f64,
    pub mean: f64,
    pub median: f64,
}

impl Uniform {
    pub fn new(lower: f64, upper: f64) -> Self {
        let dist = probability::distribution::Uniform::new(lower, upper);
        Self {
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
        // rand::thread_rng().gen_range(self.lower, self.upper)
        let sampler = Independent(&self.dist, &mut source);
        let samples = sampler.take(1).collect::<Vec<_>>();
        samples[0]
    }

    fn logp(&self) -> f64 {
        (self.upper - self.lower).ln()
    }
}

impl Continuous for Uniform {
    fn density(&self) -> f64 {
        1.0 // TODO: not yet
    }
}
