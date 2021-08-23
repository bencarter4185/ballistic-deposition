/*
Code to perform ballistic deposition experiments in Rust.

The code requires no user input, and all variables are defined as constants below. 
*/

mod sim;
use sim::{T_MAX, SKIP_VALS};

// extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;
use std::env;

fn main() {
    println!("Hello, world!");
    sim::run_sim();

    println!("{:?}", T_MAX[&8.to_string()]);
    println!("{:?}", SKIP_VALS[&8.to_string()]);

    // dotenv().ok();

    // // for (key, value) in env::vars() {
    // //     println!("{}: {}", key, value);
    // // }

    println!("{}", dotenv!("VAR"));
}
