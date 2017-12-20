extern crate rsmc;

use rsmc::distribution::Distribution::{Ber, Uni};
use rsmc::distribution::uniform::Uniform;
use rsmc::distribution::bernoulli::Bernoulli;
use rsmc::model::Model;

fn main() {
    // distributions
    let uniform = Uni(Uniform::new(1.0, 3.0));
    let bernoulli = Ber(Bernoulli::new(0.6));

    // create model
    let mut model = Model::new("model".to_string());

    // unobserved random variables
    let alpha = model.var("alpha".to_string(), &uniform);
    let beta = model.var("beta".to_string(), &bernoulli);

    // operation on unobserved random variables
    let s = alpha + beta;

    println!("s: {:?}", s);

    // TODO: deterministic variables

    // TODO: ovserved random variables

    println!("{:?}", model);
}
