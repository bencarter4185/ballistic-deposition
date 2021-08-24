/*
Code to perform ballistic deposition experiments in Rust.

Change values within config.ini to perform memes

The code requires no user input, and all variables are defined as constants below. 
*/

mod sim;

use std::error::Error;
use sim::{T_MAX, SKIP_VALS};

fn main() -> Result<(), Box<dyn Error>> {
    // println!("{:?}", T_MAX[&8.to_string()]);
    // println!("{:?}", SKIP_VALS[&8.to_string()]);

    // Open the config file
    let config = sim::open_config().expect("Failed to open `config.ini`.");

    // Parse the config file for its contents
    let _l_vals: Vec<u32> = sim::parse_config_param(&config, "simulation_params", "substrate_lengths")
        .expect("Failed to parse substrate lengths.");
    let _seeds: Vec<u32> = sim::parse_config_param(&config, "simulation_params", "seeds")
        .expect("Failed to parse seeds.");
    let _angles: Vec<u32> = sim::parse_config_param(&config, "simulation_params", "angles")
        .expect("Failed to parse angles.");
    let _imp_rec: Vec<u32> = sim::parse_config_param(&config, "simulation_params", "impurity_recurrence")
        .expect("Failed to parse impurity recurrences.");

    let _imp_add: bool = sim::parse_config_option(&config, "options", "add_impurities")
        .expect("Failed to parse whether to add impurities.");

    Ok(())
}
