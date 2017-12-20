extern crate probability;

use std::fmt;
use std::f64::consts::PI;

use distribution::DistType;

///
/// Normal distribution
///
#[derive(Clone, Copy)]
pub struct Normal {
    pub dist_type: DistType,
    pub dist: probability::distribution::Gaussian,
    pub sd: f64,
    pub tau: f64,
    pub mean: f64,
    pub variance: f64,
}

#[derive(Clone, Copy)]
pub struct HalfNormal {
    pub dist_type: DistType,
    pub dist: probability::distribution::Gaussian,
    pub sd: f64,
    pub tau: f64,
    pub mean: f64,
    pub variance: f64,
}

impl fmt::Debug for Normal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Normal {{ sd: {}, tau: {}, mean: {}, variance: {} }}",
            self.sd, self.tau, self.mean, self.variance,
        )
    }
}

impl fmt::Debug for HalfNormal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "HalfNormal {{ sd: {}, tau: {}, mean: {}, variance: {} }}",
            self.sd, self.tau, self.mean, self.variance,
        )
    }
}

impl Normal {
    pub fn new(mu: f64, sd: f64, tau: Option<f64>) -> Self {
        let new_tau: Option<f64> = if tau.is_none() {
            Some(get_tau_sd(tau, Some(sd)).0)
        } else {
            tau
        };

        let dist = probability::distribution::Gaussian::new(mu, sd);
        Self {
            dist_type: DistType::Continuous,
            dist: dist,
            sd: sd,
            tau: new_tau.unwrap(),
            mean: mu,
            variance: new_tau.unwrap(),
        }
    }

    pub fn random(&self) -> f64 {
        unimplemented!();
    }

    pub fn logp(&self) -> f64 {
        unimplemented!();
    }
}

#[allow(unused_variables)]
impl HalfNormal {
    pub fn new(sd: f64, tau: Option<f64>) -> Self {
        let new_tau: Option<f64> = if tau.is_none() {
            Some(get_tau_sd(tau, Some(sd)).0)
        } else {
            tau
        };

        let mu = (2.0 / (PI * new_tau.unwrap())).sqrt();
        let dist = probability::distribution::Gaussian::new(mu, sd);
        Self {
            dist_type: DistType::Continuous,
            dist: dist,
            sd: sd,
            tau: new_tau.unwrap(),
            mean: mu,
            variance: (1.0 - 2.0 / PI) / new_tau.unwrap(),
        }
    }

    pub fn random(&self) -> f64 {
        unimplemented!();
    }

    pub fn logp(&self) -> f64 {
        unimplemented!();
    }
}

fn get_tau_sd(tau: Option<f64>, sd: Option<f64>) -> (f64, f64) {
    let new_tau: f64;
    let new_sd: f64;

    if tau.is_none() {
        if sd.is_none() {
            //
            new_sd = 1.0;
            new_tau = 1.0;
        } else {
            //
            new_sd = sd.unwrap();
            new_tau = new_sd.powf(-2.0);
        }
    } else {
        if sd.is_some() {
            panic!("Can't pass both tau and sd");
        } else {
            new_tau = tau.unwrap();
            new_sd = new_tau.powf(-0.5);
        }
    }

    (new_tau, new_sd)
}
