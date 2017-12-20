extern crate autograd as ag;
extern crate ndarray;
extern crate ndarray_rand;
extern crate rand;
extern crate rsmc;

use rand::distributions::Range;
use ndarray::Array;
use ndarray_rand::RandomExt;

use rsmc::distribution::Distribution::{HNor, Nor};
use rsmc::distribution::normal::{HalfNormal, Normal};
use rsmc::model::Model;
use rsmc::var::Variable::ObservedRV;

///
/// TODO: example
///
/// basic_model = pm.Model()
///
/// with basic_model:
///
///     # Priors for unknown model parameters
///     alpha = pm.Normal('alpha', mu=0, sd=10)
///     beta = pm.Normal('beta', mu=0, sd=10, shape=2)
///     sigma = pm.HalfNormal('sigma', sd=1)
///
///     # Expected value of outcome
///     mu = alpha + beta[0]*X1 + beta[1]*X2
///
///     # Likelihood (sampling distribution) of observations
///     Y_obs = pm.Normal('Y_obs', mu=mu, sd=sigma, observed=Y)

#[allow(unused_variables)]
fn main() {
    // context
    let mut ctx = ag::Context::new();

    // data
    let size = 100;
    let x1 = ObservedRV(ctx.variable(Array::random((size, 1), Range::new(0., 1.))));
    let x2 = ObservedRV(ctx.variable(Array::random((size, 1), Range::new(0., 1.)) * 0.2));

    // distributions
    let normal1 = Nor(Normal::new(0.0, 10.0, None));
    let normal2 = Nor(Normal::new(0.0, 10.0, None));
    let normal3 = Nor(Normal::new(0.0, 10.0, None));
    let half_normal = HNor(HalfNormal::new(1.0, None));

    // create model
    let mut model = Model::new("model".to_string());

    // unobserved random variables
    let alpha = model.var("alpha".to_string(), &normal1);
    let beta1 = model.var("beta".to_string(), &normal2);
    let beta2 = model.var("beta".to_string(), &normal3);
    let sigma = model.var("sigma".to_string(), &half_normal);

    // operation on unobserved random variables
    let mu = alpha + beta1 * x1 + beta2 * x2;

    //let normal_obs = Nor(Normal::new(mu = mu, sd = sigma, observed = Y));
    //let y_obs = model.var("y_obs".to_string(), normal_obs);

    //println!("s: {:?}", s);

    // TODO: deterministic variables

    // TODO: ovserved random variables

    println!("{:?}", model);
}
