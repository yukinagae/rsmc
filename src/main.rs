extern crate nalgebra;
extern crate rsmc;

use rsmc::distribution::uniform::Uniform;
use rsmc::distribution::bernoulli::Bernoulli;
use rsmc::model::Model;

use nalgebra::Vector3;

fn main() {
    let v = Vector3::new(1, 2, 3);

    println!("{:?}", v);

    let uniform = Uniform::new(1.0, 3.0);
    let bernoulli = Bernoulli::new(0.6);

    let model2 = Model::new();

    let mut model = Model::new();

    model.var("alpha", &uniform);
    model.var("beta", &bernoulli);

    println!("# in named_vars");
    for (name, dist) in model.named_vars.values.iter() {
        println!("name: {}, random: {:?}", name, dist.random());
    }

    println!("# in free_rvs");
    for dist in model.free_rvs.values.iter() {
        println!("random: {:?}", dist.random());
    }

    model.set_parent(&model2);

    if model.parent.is_some() {
        println!("true");
    }
}
