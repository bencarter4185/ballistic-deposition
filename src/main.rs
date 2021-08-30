/*
Code to perform ballistic deposition experiments in Rust.

Change values within config.ini to perform memes

The code requires no user input, and all variables are defined as constants below.
*/

mod ini_parser;
use ini_parser::InputParams;
mod sim;
use sim::SimulationParams;

use std::error::Error;

use std::time::Instant;

#[macro_use]
extern crate itertools;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the params in the .ini file
    let params = InputParams::new();

    // Iterate through parameters parsed in the `.ini` and run simulations
    for (lengths_t_max, k_neighbour, max_seed) in
        iproduct!(params.lengths_t_max, params.k_neighbours, params.seeds)
    {
        // Re-parse the params so we're not trying to use a moved value every loop
        //  Wasteful? Probably. But insignificant compared to the runtime of the simulations
        let params = InputParams::new();

        let current_params: SimulationParams =
            SimulationParams::new(params, lengths_t_max, k_neighbour, max_seed);

        let now = Instant::now();
        sim::run(current_params)?;
        let new_now = Instant::now();
        println!(
            r"Done! Took {:?}
                ",
            new_now.duration_since(now)
        );
    }

    Ok(())
}
