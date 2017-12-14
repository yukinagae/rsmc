// TOOD: hoge
extern crate probability;
extern crate rsmc;

use rsmc::distributions::continuous::Uniform;
use rsmc::distributions::distribution::Distribution;

fn main() {
    let uniform = Uniform::new(1.0, 3.0);

    println!("lower: {:}, upper: {:}", uniform.lower, uniform.upper);
    println!("mean: {:}, median: {:}", uniform.mean, uniform.median);
    println!("logp: {:}", uniform.logp());
    println!("{:?}", uniform.random());
}
