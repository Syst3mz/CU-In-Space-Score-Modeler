use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub golf_ball_mass_kg: f32,

    #[serde(default)]
    pub dry_mass_kg: f32,

    #[serde(default)]
    pub max_impulse_ns: u16,

    #[serde(default)]
    pub max_golf_balls: u16,

    #[serde(default)]
    pub min_altitude_m: f32,

    #[serde(default)]
    pub max_altitude_m: f32,

    #[serde(default)]
    pub gravity_ms: f32,

    #[serde(default)]
    pub max_tracked_solutions: usize,
    
    #[serde(default)]
    pub output_file: String
}

impl Default for Config {
    fn default() -> Self {
        Self {
            golf_ball_mass_kg: 0.04592623,
            dry_mass_kg: 4.0,
            max_impulse_ns: 5120,
            max_golf_balls: 1000,
            min_altitude_m: 2743.2,
            max_altitude_m: 12192.0,
            gravity_ms: 9.81,
            max_tracked_solutions: 50,
            output_file: String::from("output.txt"),
        }
    }
}

