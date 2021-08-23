use phf::phf_map;

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

pub fn run_sim() {
    println!("Running simulation...");
}