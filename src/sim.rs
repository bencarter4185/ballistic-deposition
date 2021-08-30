/*
Library file used for running the ballistic deposition simulations.
*/
use num::{FromPrimitive, ToPrimitive};

use std::error::Error;
use std::iter::Sum;

mod random;
use random::Ran2Generator;

mod csv_writer;
use csv_writer::write_csv;

use crate::ini_parser::InputParams;

use rayon::prelude::*;

/*
Structs
*/

// Struct containing all the parameters to be used
#[derive(Debug)]
pub struct SimulationParams {
    length_t_max: (i32, i32),
    k_neighbour: i32,
    max_seed: i32,
    periodic_bc: bool,
    init_seed: i32,
}

impl SimulationParams {
    pub fn new(
        total_params: InputParams,
        length_t_max: (i32, i32),
        k_neighbour: i32,
        max_seed: i32,
    ) -> SimulationParams {
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

// Struct containing results
pub struct SimulationResults {
    avg_v_out: Vec<f64>,
    avg_h_out: Vec<f64>,
    t_out: Vec<f64>,
}

impl SimulationResults {
    pub fn new(v_out: Vec<f64>, h_out: Vec<f64>, t_out: Vec<f64>) -> SimulationResults {
        let results = SimulationResults {
            avg_v_out: v_out,
            avg_h_out: h_out,
            t_out: t_out,
        };

        results
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

fn mean<'a, T: 'a>(numbers: &'a [T], length: usize) -> Option<f64>
where
    T: ToPrimitive + Sum<&'a T>,
{
    match length {
        positive if positive > 0 => {
            // Sum the generics, convert the length of array to a float
            let sum = numbers.iter().sum::<T>();
            let length_f64 = f64::from_usize(length)?;
            // Cast the sum as f64 and return the mean
            T::to_f64(&sum).map(|sum| sum / length_f64)
        }
        _ => None,
    }
}

// May delete this later. Will require further testing but my hunch is
//  that parallelisation is pointless here as the overheads are too great
fn _mean_par<'a, T: 'a + Send + Sync>(numbers: &'a [T]) -> Option<f64>
where
    T: ToPrimitive + Sum<&'a T> + Sum,
{
    match numbers.len() {
        positive if positive > 0 => {
            // Sum the generics, convert the length of array to a float
            let sum = numbers.par_iter().sum::<T>();
            let length = f64::from_usize(numbers.len())?;
            // Cast the sum as f64 and return the mean
            T::to_f64(&sum).map(|sum| sum / length)
        }
        _ => None,
    }
}

fn std_dev<'a, T: 'a>(numbers: &'a [T], length: usize) -> Option<f64>
where
    T: ToPrimitive + Sum<&'a T>,
{
    match (mean(numbers, length), length) {
        (Some(mean_val), count) if count > 0 => {
            let dev = numbers
                .iter()
                .map(|value| {
                    let value = T::to_f64(&value).unwrap();
                    let diff = mean_val - value;
                    diff * diff
                })
                .sum::<f64>()
                / count as f64;
            Some(dev.sqrt())
        }
        _ => None,
    }
}

fn deposit_blocks(
    n: usize,
    l: i32,
    s: &mut Vec<usize>,
    rng: &mut Ran2Generator,
    k_neighbour: i32,
    periodic_bc: bool,
) {
    let mut j: i32; // column number

    // k-neighbour sticking
    let mut h_max: usize;
    let mut k: i32;

    for _ in 0..n {
        loop {
            // Generate a new random column
            j = (l as f64 * rng.next()) as i32;
            if j != l {
                break;
            }
        }
        h_max = s[j as usize] + 1; // initialize h_max to column j
        k = 0; // reset counter

        // find maximum height of all the neighbour columns and column j
        while k <= k_neighbour {
            if periodic_bc == true {
                h_max = max3(
                    s[if j - k < 0 {
                        (l + ((j - k) % l)) as usize
                    } else {
                        (j - k) as usize
                    }],
                    h_max,
                    s[((j + k) % l) as usize],
                );
            } else {
                h_max = max3(
                    s[max(j - k, 0) as usize],
                    h_max,
                    s[min(j + k, l - 1) as usize],
                );
            }
            // increment k
            k += 1;
        }
        // set column j to new height
        s[j as usize] = h_max;
    }
}

pub fn do_sim(
    params: &SimulationParams,
    seed: i32,
    t_points: usize,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    // Unpack struct of params
    let (l, t_max) = params.length_t_max;
    let k_neighbour = params.k_neighbour;
    let _ = params.max_seed;
    let periodic_bc = params.periodic_bc;
    let init_seed = params.init_seed;

    let mut h: f64; // average height
    let mut v: f64; // interface width

    // Initialise random seed, based on the current system
    let idum: i32 = -1 * (l + init_seed + seed).abs();

    // Instantiate a new random number generator with given initial seed
    let mut rng: Ran2Generator = Ran2Generator::new(idum);

    // Define separate variable for the length of surface as usize
    let s_len: usize = l as usize;

    // Declare surface array
    let mut s: Vec<usize> = vec![0; s_len];

    // Define some counting variables
    let mut i: usize = 0; // Counter for ensemble data array
    let mut n: usize; // Number of particles to be dropped next
    let mut t: f64 = 0.0;

    // Define vectors to store caluclated values
    let mut v_out: Vec<f64> = vec![0.0; t_points];
    let mut h_out: Vec<f64> = vec![0.0; t_points];
    let mut t_out: Vec<f64> = vec![0.0; t_points];

    // For each time-point in our logarithmic timescale
    while t < t_max as f64 {
        n = (t * l as f64 / 100.0 + 1.0) as usize; // Number of particles to drop next
        if n == 1 {
            n = l as usize
        }
        // Deposit n particles on surface s
        deposit_blocks(n, l, &mut s, &mut rng, k_neighbour, periodic_bc);
        h = mean(&s, s_len).unwrap();
        v = std_dev(&s, s_len).unwrap();
        t += n as f64 / l as f64;

        // Save calculated values for this time point to the vectors
        v_out[i] = v;
        h_out[i] = h;
        t_out[i] = t;

        i += 1;
    }
    (v_out, h_out, t_out)
}

pub fn run(params: SimulationParams) -> Result<(), Box<dyn Error>> {
    // Unpack struct of params
    let (l, t_max) = params.length_t_max;
    let k_neighbour = params.k_neighbour;
    let max_seed = params.max_seed;
    let periodic_bc = params.periodic_bc;
    let init_seed = params.init_seed;

    println!(
        r"Running simulation for
        Substrate Length = {}, Nearest Neighbours = {}, Max Seed = {},
        Periodic Boundary Conditions = {}, Initial Seed = {}...",
        l, k_neighbour, max_seed, periodic_bc, init_seed
    );

    // Set current time and counter of total time points
    let mut t: f64 = 0.0;
    let mut t_points: usize = 0;
    while t < t_max as f64 {
        // Perform a logarithmic timestep
        let mut n: u32 = (t * l as f64 / 100.0 + 1.0) as u32;
        if n == 1 {
            n = l as u32
        }
        t += n as f64 / l as f64;
        // println!("{}", t); // turn on for debugging as necessary
        t_points += 1;
    }

    // Generate vector through which to iterate seeds
    let seeds: Vec<i32> = (0..max_seed).collect();

    // Iterate through seeds in parallel and run the ballistic deposition simulation
    let data: Vec<(Vec<f64>, Vec<f64>, Vec<f64>)> = seeds
        .par_iter()
        .map(|seed| {
            let (v, h, t) = do_sim(&params, *seed, t_points);
            (v, h, t)
        })
        .collect();

    // Done depositing. Now calculate ensemble averages and save to file

    // Generate vectors to store outgoing data
    let mut v_avg: Vec<f64> = vec![0.0; t_points];
    let mut h_avg: Vec<f64> = vec![0.0; t_points];
    let mut t_avg: Vec<f64> = vec![0.0; t_points];

    // Average v and h across the ensemble
    for (i, j) in iproduct!(0..max_seed as usize, 0..t_points) {
        v_avg[j] += &data[i].0[j] / max_seed as f64;
        h_avg[j] += &data[i].1[j] / max_seed as f64;
    }

    // Don't need to average time, so can do it in its own loop
    for j in 0..t_points {
        t_avg[j] += &data[0].2[j];
    }

    let results = SimulationResults::new(v_avg, h_avg, t_avg);

    // Now need to write these results to a csv file
    write_csv(&params, &results, t_points)?;

    Ok(())
}
