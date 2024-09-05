use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub golf_ball_mass_kg: f32,

    #[serde(default)]
    pub dead_mass_kilo_grams: f32,

    #[serde(default)]
    pub max_impulse: u16,

    #[serde(default)]
    pub max_golf_balls: u16,

    #[serde(default)]
    pub gravity: f32,

    #[serde(default)]
    pub max_tracked_solutions: usize,
    
    #[serde(default)]
    pub output_file: String
}

impl Default for Config {
    fn default() -> Self {
        Self {
            golf_ball_mass_kg: 0.04592623,
            dead_mass_kilo_grams: 4.0,
            max_impulse: 5120,
            max_golf_balls: 1000,
            gravity: 9.81,
            max_tracked_solutions: 50,

            output_file: String::from("output.txt"),
        }
    }
}

