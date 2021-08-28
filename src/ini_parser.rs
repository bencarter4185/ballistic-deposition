extern crate configparser;
use configparser::ini::Ini;

use std::error::Error;

#[derive(Debug)]
pub struct InputParams {
    pub lengths: Vec<u32>,
    pub k_neighbours: Vec<u32>,
    pub seeds: Vec<u32>,
    pub periodic_bc: bool,
    pub init_seed: u32,
}

impl InputParams {
    pub fn new() -> InputParams {
        // Instantiate our parameters for the simulation
        let config = open_config().expect("Failed to open `config.ini`.");

        let lengths: Vec<u32> =
            parse_config_array(&config, "simulation_params", "substrate_lengths")
                .expect("Failed to parse substrate lengths.");
        let k_neighbours: Vec<u32> =
            parse_config_array(&config, "simulation_params", "k_neighbours")
                .expect("Failed to parse number of nearest neighbours.");
        let seeds: Vec<u32> =
            parse_config_array(&config, "simulation_params", "seeds")
                .expect("Failed to parse number of seeds.");
        let periodic_bc: bool = 
            parse_config_option(&config, "options", "periodic_bc")
                .expect("Failed to parse whether to apply periodic boundary conditions.");
        let init_seed: u32 = 
            parse_config_u32(&config, "options", "init_seed")
                .expect("Failed to parse initial random number seed.");

        let params: InputParams = InputParams {
            lengths: lengths,
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
) -> Result<Vec<u32>, Box<dyn Error>> {
    // Parse the config.ini file for its specified key value pair.
    // This code works for parsing a vector of values only.
    let config_entry = &config
        .get(section, key)
        .expect("Invalid section/key pair in `config.ini`.")[..]; // Convert to string literal

    let mut vals: Vec<u32> = vec![];

    let values = config_entry.split(',');
    for v in values {
        let temp: u32 = v.trim().parse().unwrap_or(0);
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

pub fn parse_config_u32 (
    config: &Ini,
    section: &str,
    key: &str,
) -> Result<u32, Box<dyn Error>> {
    // Parse the config.ini file for its specified key value pair.
    // This code works for parsing u32 values only.
    let config_entry = &config
        .get(section, key)
        .expect("Invalid section/key pair in config.ini.")[..]; // convert to string literal &str
    
    let val: u32 = config_entry.parse().unwrap();
    Ok(val)
}