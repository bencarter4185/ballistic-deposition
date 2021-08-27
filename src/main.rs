/*
Code to perform ballistic deposition experiments in Rust.

Change values within config.ini to perform memes

The code requires no user input, and all variables are defined as constants below. 
*/

mod sim;
mod rw;

use std::error::Error;

#[macro_use]
extern crate itertools;

fn main() -> Result<(), Box<dyn Error>> {
    // Open the config file
    let config = rw::open_config().expect("Failed to open `config.ini`.");

    // Parse the config file for its params and options
    let lengths: Vec<u32> = rw::parse_config_param(&config, "simulation_params", "substrate_lengths")
        .expect("Failed to parse substrate lengths.");
    let seeds: Vec<u32> = rw::parse_config_param(&config, "simulation_params", "seeds")
        .expect("Failed to parse seeds.");
    let impurities: Vec<u32> = rw::parse_config_param(&config, "simulation_params", "impurity_recurrence")
        .expect("Failed to parse impurity recurrences.");

    let _imp_add: bool = rw::parse_config_option(&config, "options", "add_impurities")
        .expect("Failed to parse whether to add impurities.");

    
    // Iterate through parameters parsed in the .ini and run simulation
    for (length, max_seed, impurity) in iproduct!(lengths, seeds, impurities) {
        sim::run(length, max_seed, impurity);
    }

    Ok(())
}
