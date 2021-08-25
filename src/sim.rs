/*
Library file used for running the ballistic deposition simulations.
*/

extern crate phf;
use phf::phf_map;

use std::error::Error;

use std::f32::consts; 

// Map the maximum value of `t` for each substrate length `L` 
pub static T_MAX: phf::Map<&'static str, i64> = phf_map! {
    "8" => 10_000,
    "16" => 10_000,
    "32" => 10_000,
    "64" => 10_000,
    "128" => 10_000,
    "256" => 10_000,
    "512" => 10_000,
    "1024" => 100_000,
    "2048" => 100_000,
    "4096" => 100_000,
};

// Map the maximum value of `t` for each substrate length `L` 
pub static SKIP_VALS: phf::Map<&'static str, i64> = phf_map! {
    "8" => 5,
    "16" => 10,
    "32" => 20,
    "64" => 40,
    "128" => 80,
    "256" => 160,
    "512" => 320,
    "1024" => 6400,
    "2048" => 12800,
    "4096" => 25600,
};

fn deg_to_rad(val: f32) -> f32 {
    val * (consts::PI / 180.)
}

pub fn gen_angles(angle_max: u32) -> Result<(), Box<dyn Error>> {
    // Need to convert theta_max from u32 to f32 in order to use it as a negative.
    let angle_max = angle_max as i32;
    
    let mut angles: Vec<i32> = Vec::new();
    let mut prob_down_arr: Vec<f32> = Vec::new();
    let mut prob_left_arr: Vec<f32> = Vec::new();
    let mut prob_right_arr: Vec<f32> = Vec::new();
    let mut prob_arr: Vec<f32> = Vec::new();
    let mut prob_arr_sum: f32 = 0.;

    for angle in -angle_max ..= angle_max {
        
        let prob_temp = 0.5 * ((deg_to_rad(angle as f32 + 0.5)).sin() - (deg_to_rad(angle as f32 - 0.5)).sin());
        prob_arr_sum += prob_temp;
        prob_arr.push(prob_temp);
        
        angles.push(angle);
        // Generate probability distribution for moving down every iteration
        match angle {
            -90..=-46 => {
                prob_down_arr.push((angle as f32)/45. + 2.);
            },
            -45..=45 => {
                prob_down_arr.push(1.);
            }
            46..=90 => {
                prob_down_arr.push((-angle as f32)/45. + 2.);
            }
            _ => panic!("Angle out of bounds when generating probability distributions!")
        }

        match angle {
            -90..=-46 => {
                prob_left_arr.push(1.);
                prob_right_arr.push(0.);
            }
            -45..=-1 => {
                prob_left_arr.push((-angle as f32)/45.);
                prob_right_arr.push(0.);
            }
            0 => {
                prob_left_arr.push(0.);
                prob_right_arr.push(0.);
            }
            1..=45 => {
                prob_left_arr.push(0.);
                prob_right_arr.push((angle as f32)/45.);
            }
            46..=90 => {
                prob_left_arr.push(0.);
                prob_right_arr.push(1.);
            }
            _ => panic!("Angle out of bounds when generating probability distributions!")
        }
    }

    for i in 0..prob_arr.len() {
        prob_arr[i] *= 1./prob_arr_sum
    } 

    let mut prob_arr_sorted = prob_arr.clone();
    prob_arr_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    Ok(())
}

pub fn run(length: u32, angle: u32, seed: u32, impurity: u32) {
    println!(r"Running simulation for:
    length = {}, angle = {}, seed = {}, impurity = {}...
    ", length, angle, seed, impurity);


}