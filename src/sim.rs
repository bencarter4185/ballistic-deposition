/*
Library file used for running the ballistic deposition simulations.
*/

use std::time::Instant;

mod random;
use random::Ran2Generator;

pub fn run(length: u32, max_seed: u32, impurity: u32) {
    println!(
        r"Running simulation for
        Substrate Length = {}, Seeds = {}, Impurity Recurrence = {}...",
        length, max_seed, impurity
    );

    /*
    TEST: Generate 2^32 - 1 random numbers and benchmark how long it takes
    */
    let iterations: u32 = u32::MAX;

    let now = Instant::now();
    println!("Generating {} random numbers...", iterations);

    let mut x: f64;
    let mut sum: f64 = 0.0;
    let mut rng: Ran2Generator = Ran2Generator::new();

    for _ in 0..iterations as usize {
        x = rng.next();
        sum += x;
    }

    let new_now = Instant::now();
    println!("Done! Took {:?}", new_now.duration_since(now));

    // Gross error check: expect a mean value of ~0.5 for this normal distribution
    let avg: f64 = sum / iterations as f64;
    println!("Mean of the random numbers generated is: {}", avg);
}
