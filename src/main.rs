extern crate probability;
extern crate rsmc;

use rsmc::distribution::uniform::Uniform;
use rsmc::distribution::bernoulli::Bernoulli;
use rsmc::model::Model;

fn main() {
    let uniform = Uniform::new("alpha", 1.0, 3.0);
    let bernoulli = Bernoulli::new("beta", 0.6);

    let mut model = Model::new();

    model.add_dist(&uniform);
    model.add_dist(&bernoulli);
    println!("{:?}", model.dists[0].name());
    println!("{:?}", model.dists[1].name());
}
