extern crate configparser;
use configparser::ini::Ini;

use std::error::Error;

#[derive(Debug)]
pub struct InputParams {
    pub lengths_t_max: Vec<(i32, i32)>,
    pub k_neighbours: Vec<i32>,
    pub seeds: Vec<i32>,
    pub periodic_bc: bool,
    pub init_seed: i32,
}

impl InputParams {
    pub fn new() -> InputParams {
        // Instantiate our parameters for the simulation
        let config = open_config().expect("Failed to open `config.ini`.");
        
        let lengths: Vec<i32> =
            parse_config_array(&config, "simulation_params", "substrate_lengths")
            .expect("Failed to parse substrate lengths.");
        let lengths_t_max = gen_lengths_t_max(&lengths);
        
        let k_neighbours: Vec<i32> =
        parse_config_array(&config, "simulation_params", "k_neighbours")
        .expect("Failed to parse number of nearest neighbours.");
        let seeds: Vec<i32> =
        parse_config_array(&config, "simulation_params", "seeds")
        .expect("Failed to parse number of seeds.");
        let periodic_bc: bool = 
        parse_config_option(&config, "options", "periodic_bc")
        .expect("Failed to parse whether to apply periodic boundary conditions.");
        let init_seed: i32 = 
        parse_config_i32(&config, "options", "init_seed")
        .expect("Failed to parse initial random number seed.");
        
        let params: InputParams = InputParams {
            lengths_t_max: lengths_t_max,
            k_neighbours: k_neighbours,
            seeds: seeds,
            periodic_bc: periodic_bc,
            init_seed: init_seed,
        };
        
        params
    }
}

pub fn open_config() -> Result<Ini, Box<dyn Error>> {
    // Open the `config.ini` file and extract the contents
    let mut config = Ini::new();
    config.load("config.ini")?;

    Ok(config)
}

pub fn parse_config_array (
    config: &Ini,
    section: &str,
    key: &str,
) -> Result<Vec<i32>, Box<dyn Error>> {
    // Parse the config.ini file for its specified key value pair.
    // This code works for parsing a vector of values only.
    let config_entry = &config
        .get(section, key)
        .expect("Invalid section/key pair in `config.ini`.")[..]; // Convert to string literal

    let mut vals: Vec<i32> = Vec::new();

    let values = config_entry.split(',');
    for v in values {
        let temp: i32 = v.trim().parse().unwrap_or(0);
        vals.push(temp);
    }

    Ok(vals)
}

pub fn parse_config_option (
    config: &Ini,
    section: &str,
    key: &str,
) -> Result<bool, Box<dyn Error>> {
    // Parse the config.ini file for its specified key value pair.
    // This code works for parsing boolean values only.
    let config_entry = &config
        .get(section, key)
        .expect("Invalid section/key pair in config.ini.")[..]; // convert to string literal &str

    let aliases_true: Vec<&str> = vec!["true", "True", "t", "yes", "Yes", "y"];
    let aliases_false: Vec<&str> = vec!["false", "False", "f", "no", "No", "n"];

    let val: bool = match config_entry {
        config_entry if aliases_true.iter().any(|&i| i == config_entry) => true,
        config_entry if aliases_false.iter().any(|&i| i == config_entry) => false,
        _ => {
            println!(
                r"WARNING: The entry for `{}, {}` is malformed. Setting value to `false` by default.
            Please specify one of {:?} for `true`,
            or one of {:?} for `false`.",
                section, key, aliases_true, aliases_false
            );
            false
        }
    };

    Ok(val)
}

pub fn parse_config_i32 (
    config: &Ini,
    section: &str,
    key: &str,
) -> Result<i32, Box<dyn Error>> {
    // Parse the config.ini file for its specified key value pair.
    // This code works for parsing i32 values only.
    let config_entry = &config
        .get(section, key)
        .expect("Invalid section/key pair in config.ini.")[..]; // convert to string literal &str
    
    let val: i32 = config_entry.parse().unwrap();
    Ok(val)
}

fn gen_lengths_t_max (lengths: &Vec<i32>) -> Vec<(i32, i32)> {
    // Only supports powers of 2 from 8 to 4096 at the moment
    // TODO: include support for different substrate lengths?

    // Create an empty vector for zipping together length and t_max
    let mut lengths_with_t_max: Vec<(i32, i32)> = Vec::new(); // vec![(0, 0); lengths.len()];
    
    for length in lengths {
        let t_max: i32 = match *length {
            8 => 10_000,
            16 => 10_000,
            32 => 10_000,
            64 => 10_000,
            128 => 10_000,
            256 => 10_000,
            512 => 10_000,
            1024 => 100_000,
            2048 => 100_000,
            4096 => 100_000,
            8192 => 1_000_000, // test

            // If length isn't one of the hard-coded values, panic and tell the user
            _ => {
                panic!("Error: {length} isn't a valid substrate length! Please supply a power of 2 from 8 to 4096 inclusive.", length = *length);
            }
        };
        lengths_with_t_max.push((*length, t_max));
    }
    lengths_with_t_max
}