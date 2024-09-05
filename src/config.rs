use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum StageAdditionFunction {
    None,
    Addition
}

impl Default for StageAdditionFunction {
    fn default() -> Self {
        StageAdditionFunction::Addition
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stages {
    #[serde(default)]
    stage_addition_function: StageAdditionFunction,
    #[serde(default)]
    stages_dry_mass: Vec<f32>
}

impl Stages {
    pub fn stage_masses(&self) -> Vec<f32> {
        match self.stage_addition_function {
            StageAdditionFunction::None => self.stages_dry_mass.clone(),
            StageAdditionFunction::Addition => {
                let mut sum = 0.0;
                let mut masses = vec![0_f32; self.stages_dry_mass.len()];

                for stage_index in (0..self.stages_dry_mass.len()).rev() {
                    sum += self.stages_dry_mass[stage_index];
                    masses[stage_index] = sum
                }

                masses
            }
        }
    }
}
impl Default for Stages {
    fn default() -> Self {
        Self {
            stage_addition_function: StageAdditionFunction::default(),
            stages_dry_mass: vec![4.0]
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub golf_ball_mass_kg: f32,

    #[serde(default)]
    pub stages: Stages,

    #[serde(default)]
    pub min_stage_impulse: u16,

    #[serde(default)]
    pub max_total_impulse: u16,

    #[serde(default)]
    pub min_golf_balls: u16,
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
    pub output_file: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            golf_ball_mass_kg: 0.04592623,
            stages: Stages::default(),
            max_total_impulse: 5120,
            min_golf_balls: 1,
            max_golf_balls: 1000,
            min_altitude_m: 2743.2,
            max_altitude_m: 12192.0,
            gravity_ms: 9.81,
            max_tracked_solutions: 50,
            output_file: String::from("output.txt"),
            min_stage_impulse: 0,
        }
    }
}

