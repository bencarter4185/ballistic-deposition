/*
Library file used for running the ballistic deposition simulations.
*/

use num::{FromPrimitive, ToPrimitive};

use std::iter::Sum;
use std::time::Instant;

mod random;
use random::Ran2Generator;

use crate::ini_parser::InputParams;

/*
Structs
*/

// Struct containing all the parameters to be used 
#[derive(Debug)]
pub struct SimulationParams {
    length_t_max: (u32, u32),
    k_neighbour: u32,
    max_seed: u32,
    periodic_bc: bool,
    init_seed: u32,
}

impl SimulationParams {
    pub fn new(total_params: InputParams, length_t_max: (u32, u32), k_neighbour: u32, max_seed: u32) -> SimulationParams {
        let current_params: SimulationParams = SimulationParams {
            length_t_max: length_t_max,
            k_neighbour: k_neighbour,
            max_seed: max_seed,
            periodic_bc: total_params.periodic_bc,
            init_seed: total_params.init_seed,
        };
        current_params
    }
}


/*
Functions
*/

// Return the largest of two values
fn max<T: Ord>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}

// Return the smallest of two values
fn min<T: Ord>(a: T, b: T) -> T {
    if a <= b {
        a
    } else {
        b
    }
}

// Return the largest of three values
fn max3<T: Ord>(a: T, b: T, c: T) -> T {
    if a <= b {
        if b <= c {
            c
        } else {
            b
        }
    } else {
        if a <= c {
            c
        } else {
            a
        }
    }
}

fn mean<'a, T: 'a>(numbers: &'a [T]) -> Option<f64>
where
    T: ToPrimitive + Sum<&'a T>,
{
    match numbers.len() {
        positive if positive > 0 => {
            // Sum the generics, convert the length of array to a float
            let sum = numbers.iter().sum::<T>();
            let length = f64::from_usize(numbers.len())?;
            // Cast the sum as f64 and return the mean
            T::to_f64(&sum).map(|sum| sum / length)
        }
        _ => None,
    }
}

fn variance<'a, T: 'a>(numbers: &'a [T]) -> Option<f64>
where
    T: ToPrimitive + Sum<&'a T>,
{
    match (mean(numbers), numbers.len()) {
        (Some(mean_val), count) if count > 0 => {
            let variance = numbers
                .iter()
                .map(|value| {
                    let value = T::to_f64(&value).unwrap();
                    let diff = mean_val - value;
                    diff * diff
                })
                .sum::<f64>()
                / count as f64;
            Some(variance)
        }
        _ => None,
    }
}


// pub struct SimulationParams {
//     length_t_max: (u32, u32),
//     k_neighbour: u32,
//     max_seed: u32,
//     periodic_bc: bool,
//     init_seed: u32,
// }

pub fn run(params: SimulationParams) {
    // Unpack struct of params
    let (length, _t_max) = params.length_t_max;
    let k_neighbour = params.k_neighbour;
    let max_seed = params.max_seed;
    let _periodic_bc = params.periodic_bc;
    let _init_seed = params.init_seed;
    
    println!(
        r"Running simulation for
        Substrate Length = {}, Nearest Neighbours = {}, Max Seed = {}...",
        length, k_neighbour, max_seed
    );

    // Set current time and counter of total time points
    let _t: f64 = 0.0;
    let _t_points: i32 = 0;


    /*
    Placeholder code to test the added functions
    */

    dbg!(&params);

    let a = 2; let b = 3; let c = 4;
    println!("The max value of {} and {} is {}", a, b, max(a, b));
    println!("The min value of {} and {} is {}", a, b, min(a, b));
    println!("The max value of {}, {}, and {} is {}", a, b, c, max3(a, b, c));
    
    let some_array = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];
    println!("The mean of the array is: {:?}", mean(&some_array).unwrap());
    println!("The variance of the array is: {:?}", variance(&some_array).unwrap());


    let iterations: u16 = u16::MAX;

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

    println!("");
}
