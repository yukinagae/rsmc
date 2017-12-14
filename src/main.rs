// TOOD: hoge
extern crate probability;
extern crate rsmc;

use rsmc::distributions::continuous::Uniform;
use rsmc::distributions::discrete::Bernoulli;
use rsmc::distributions::distribution::Distribution;
use rsmc::model::Model;

fn main() {
    let uniform = Uniform::new(1.0, 3.0);

    println!("lower: {:}, upper: {:}", uniform.lower, uniform.upper);
    println!("mean: {:}, median: {:}", uniform.mean, uniform.median);
    println!("logp: {:}", uniform.logp());
    println!("{:?}", uniform.random());

    let bernoulli = Bernoulli::new(0.6);

    println!("p: {:}", bernoulli.p);
    println!("modes: {:?}", bernoulli.modes);
    println!("logp: {:}", bernoulli.logp());
    println!("{:?}", bernoulli.random());

    let mut model = Model::new();
    model.add_dist(&uniform);
    model.add_dist(&bernoulli);
}
