extern crate nalgebra;
extern crate rsmc;

use rsmc::distribution::Distribution::{Ber, Uni};
use rsmc::distribution::uniform::Uniform;
use rsmc::distribution::bernoulli::Bernoulli;
use rsmc::model::Model;

//use nalgebra::Vector3;

fn main() {
    //let v = Vector3::new(1, 2, 3);

    //println!("{:?}", v);

    let uniform = Uni(Uniform::new(1.0, 3.0));
    let bernoulli = Ber(Bernoulli::new(0.6));
    //let s = uniform + bernoulli;

    let mut model = Model::new("model".to_string());

    model.var("alpha".to_string(), &uniform);
    model.var("beta".to_string(), &bernoulli);
    //model.var("s".to_string(), &s);

    println!("{:?}", model);
    println!("{:?}", model.graph);
}
