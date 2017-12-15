extern crate rsmc;

use rsmc::distribution::uniform::Uniform;
use rsmc::distribution::bernoulli::Bernoulli;
use rsmc::model::Model;

fn main() {
    let uniform = Uniform::new(1.0, 3.0);
    let bernoulli = Bernoulli::new(0.6);

    println!("{:?}", uniform);
    println!("{:?}", bernoulli);

    let mut model = Model::new();

    model.var("alpha", &uniform);
    model.var("beta", &bernoulli);

    for (name, value) in model.vars.iter() {
        println!("name: {:?}, random: {:?}", name, value.random());
    }
}
