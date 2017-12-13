// TOOD: hoge
extern crate rsmc;

use rsmc::distributions::Uniform;
use rsmc::distributions::Distribution;

fn main() {
    let uniform = Uniform {
        lower: 1.0,
        upper: 2.0,
        mean: 3.0,
        median: 4.0,
    };

    println!("{:?}", uniform);
    println!("{:?}", uniform.random());
}
