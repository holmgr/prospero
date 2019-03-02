use serde::Deserialize;

/// Application level configuration options.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub simulation: Simulation,
}

/// Parameters used for simulation and generation.
#[derive(Debug, Deserialize)]
pub struct Simulation {
    pub map_seed: u32,
    pub number_of_systems: u64,
    pub system_spread: f64,
}
