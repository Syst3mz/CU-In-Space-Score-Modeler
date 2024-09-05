use std::fmt::{Display, Formatter};
use crate::config::Config;

#[derive(Debug, Copy, Clone)]
pub struct ScoringCriteria {
    pub apogee: f32,
    pub golf_balls: u16,
    pub impulse: f32
}

impl ScoringCriteria {
    pub fn new(apogee: f32, golf_balls: u16, impulse: f32) -> Self {
        Self {
            apogee,
            golf_balls,
            impulse,
        }
    }
    pub fn score(&self) -> f32 {
        let golf_ball_count_power = -(self.golf_balls as f32) / 3.0;
        let impulse_score = self.impulse / 4000.0;
        self.apogee * (3.0 - f32::exp(golf_ball_count_power) - impulse_score)
    }

    /// Decide if a given ScoringCriteria is a valid solution to be considered further.
    pub fn invalid(&self, config: &Config) -> bool {
        self.apogee < config.min_altitude_m || self.apogee > config.max_altitude_m
    }
}

impl Display for ScoringCriteria {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Apogee: {}, Golf Balls: {}, Impulse: {}", self.apogee, self.golf_balls, self.impulse)
    }
}