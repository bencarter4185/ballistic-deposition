/*
Code to perform ballistic deposition experiments in Rust.

Change values within config.ini to perform memes

The code requires no user input, and all variables are defined as constants below.
*/

mod ini_parser;
use ini_parser::InputParams;
mod sim;

use std::error::Error;

use crate::sim::SimulationParams;

#[macro_use]
extern crate itertools;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the params in the .ini file
    let params = InputParams::new();

    // Iterate through parameters parsed in the `.ini` and run simulations
    for (length, k_neighbour, max_seed) in
        iproduct!(params.lengths, params.k_neighbours, params.seeds)
    {
        // Re-parse the params so we're not trying to use a moved value every loop
        //  Wasteful? Probably. But insignificant compared to the runtime of the simulations
        let params = InputParams::new();

        let current_params: SimulationParams =
            SimulationParams::new(params, length, k_neighbour, max_seed);
        
            sim::run(current_params);
    }

    Ok(())
}
