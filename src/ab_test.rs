extern crate rand;

use rand::Rng;
use rand::distributions::{IndependentSample, Normal};

fn ab_test() {
    let n_visitors_a = 100; // number of visitors shown layout A
    let n_conv_a = 4; // number of vistors shown layout A who converted

    let n_visitors_b = 40;
    let n_conv_b = 2;

    println!("{:?}", simulmate_conversion(0.1, 100));
    println!("{:?}", simulate_conversion(0.1, 100));
    println!("{:?}", simulate_conversion(0.1, 100));

    println!("{:?}", take_samples(3, uniform_prior_sampler));

    let posterior_a_sampler = || -> f64 {
        posterior_sampler(n_conv_a, uniform_prior_sampler, |p: f64| -> usize {
            simulate_conversion(p, n_visitors_a)
        })
    };

    let a_samples = take_samples(100, posterior_a_sampler);
    println!("length: {:}", a_samples.len());
    println!(
        "mean: {:?}",
        a_samples.iter().sum::<f64>() / a_samples.len() as f64
    );

    let prob_a: f64 = (a_samples
        .iter()
        .filter(|&&a| a > 0.1 as f64)
        .collect::<Vec<&f64>>()
        .len() as f64) / (a_samples.len() as f64);
    println!("{:?}", prob_a);

    let posterior_b_sampler = || -> f64 {
        posterior_sampler(n_conv_b, normal_prior_sampler, |p: f64| -> usize {
            simulate_conversion(p, n_visitors_b)
        })
    };

    let b_samples = take_samples(100, posterior_b_sampler);
    println!("length: {:}", b_samples.len());
    println!(
        "mean: {:?}",
        b_samples.iter().sum::<f64>() / b_samples.len() as f64
    );

    let prob_b: f64 = (a_samples
        .iter()
        .zip(b_samples.iter())
        .filter(|&pair| pair.1 > pair.0)
        .collect::<Vec<(&f64, &f64)>>()
        .len() as f64) / (a_samples.len() as f64);
    println!("{:?}", prob_b);
}

fn simulate_conversion(p: f64, n_visitors: i32) -> usize {
    (0..n_visitors)
        .map(|_| rand::thread_rng().gen_range(0.0, 1.0))
        .filter(|value| value < &p)
        .collect::<Vec<f64>>()
        .len()
}

fn uniform_prior_sampler() -> f64 {
    rand::thread_rng().gen_range(0.0, 1.0)
}

fn normal_prior_sampler() -> f64 {
    let normal = Normal::new(0.06, 0.02);
    normal.ind_sample(&mut rand::thread_rng())
}

fn posterior_sampler<F>(data: usize, prior_sampler: fn() -> f64, simulate: F) -> f64
where
    F: Fn(f64) -> usize,
{
    loop {
        let p = prior_sampler();
        if simulate(p) == data {
            return p;
        }
    }
}

fn take_samples<F>(n: i32, sampler: F) -> Vec<f64>
where
    F: Fn() -> f64,
{
    (0..n).map(|_| sampler()).collect()
}
