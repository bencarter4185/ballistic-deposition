/*
Library file used for running the ballistic deposition simulations.
*/

extern crate phf;
use std::{error::Error, io::Split, vec};

use phf::phf_map;

extern crate configparser;
use configparser::ini::Ini;


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

pub fn open_config() -> Result<Ini, Box<dyn Error>> {
    // Open the `config.ini` file and extract the contents

    let mut config = Ini::new();
    config.load("config.ini")?;
    
    Ok(config)
}

pub fn parse_config_param(config: &Ini, section: &str, key: &str) -> Result<Vec<u32>, Box<dyn Error>> {
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

pub fn parse_config_option(config: &Ini, section: &str, key: &str) -> Result<bool, Box<dyn Error>> {
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
            println!(r"WARNING: The entry for `{}, {}` is malformed. Setting value to `false` by default.
            Please specify one of {:?} for `true`,
            or one of {:?} for `false`.", section, key, aliases_true, aliases_false);
            false
        }, 
    };

    Ok(val)
}

pub fn run_sim() {
    println!("Running simulation...");
}