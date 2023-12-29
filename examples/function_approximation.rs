use std::{
    f64::consts::{E, TAU},
    mem::size_of,
    time::Instant,
};

use path_tracer::function_approximation::FunctionApproximation;
use rand::{thread_rng, Rng};

fn pdf(x: f64, roughness: f64) -> f64 {
    E.powf(-(x * x) / (roughness * roughness) / 2.) / (roughness * TAU.sqrt())
}

fn main() {
    const AMOUNT: usize = 100_000_000;
    let mut rng = thread_rng();
    let random_xs: Vec<_> = (0..AMOUNT).map(|_| rng.gen::<f64>()).collect();
    //let random_roughness: Vec<_> = (0..AMOUNT).map(|_| rng.gen::<f64>()).collect();

    let start_time = Instant::now();
    let mut total = 0.;
    for x in &random_xs {
        total += pdf(*x, 0.5);
    }
    let duration = start_time.elapsed();
    println!("{duration:?} {}", total / AMOUNT as f64);

    let approximation: FunctionApproximation =
        FunctionApproximation::build(|x| pdf(x, 0.5), 0., 1., 1000);
    println!("{}", size_of::<FunctionApproximation>());
    let start_time = Instant::now();
    let mut total = 0.;
    for x in &random_xs {
        total += approximation.apply(*x).unwrap()
    }
    let duration = start_time.elapsed();
    println!("{duration:?} {}", total / AMOUNT as f64);

    let integration = approximation.integrate().normalize().invert();
    for i in 0..11 {
        let x = i as f64 / 10.;
        println!("{}", integration.apply(x).unwrap())
    }
}
