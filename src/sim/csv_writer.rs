/*
Library file used for writing csv files.
*/

extern crate csv;
use std::error::Error;
use std::{env, fs};

use super::{SimulationParams, SimulationResults};

pub fn check_folder_exists(folder: &String) -> Result<bool, Box<dyn Error>> {
    // Generate path to folder
    let mut path = env::current_dir()?;
    path.push(folder);
    
    // Check if metadata for the folder exists. If it fails, return false
    let metadata = fs::metadata(path);
    let folder_exists = match metadata {
        Ok(_) => true,
        Err(_) => false
    };

    Ok(folder_exists)
}

pub fn write_csv(
    params: &SimulationParams,
    results: &SimulationResults,
    t_points: usize
) -> Result<(), Box<dyn Error>> {
    // Unpack struct of params
    let (l, _t_max) = params.length_t_max;
    let k_neighbour = params.k_neighbour;
    let max_seed = params.max_seed;
    let pbc: i32 = if params.periodic_bc == true {1} else {0};
    let init_seed = params.init_seed;
    
    // Check if the `data` folder exists. If not, create it
    let folder: String = String::from("data");
    if !(check_folder_exists(&folder)?) {
        fs::create_dir(&folder)?
    }


    // Create the filename to write to disk based on the input parameters
    let filepath = format!("./{}/L{}_k{}_seeds{}_pbc{}_iseed{}.csv", folder, l, k_neighbour, max_seed, pbc, init_seed);

    let mut wtr = csv::Writer::from_path(filepath)?;

    for i in 0..t_points {
        wtr.write_record(&[&results.avg_v_out[i].to_string()[..], &results.avg_h_out[i].to_string()[..], &results.t_out[i].to_string()[..]])?;
    }

    wtr.flush()?;
    Ok(())
}
