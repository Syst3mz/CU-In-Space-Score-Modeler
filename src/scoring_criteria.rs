use std::fmt::{Display, Formatter};
use itertools::Itertools;
use crate::CONFIG;

#[derive(Debug, Clone)]
pub struct ScoringCriteria {
    pub apogee: f32,
    pub golf_balls: u16,
    pub stage_impulses: Vec<u16>
}

impl ScoringCriteria {
    pub fn new(apogee: f32, golf_balls: u16, stage_impulses: Vec<u16>) -> Self {
        Self {
            apogee,
            golf_balls,
            stage_impulses,
        }
    }
    pub fn score(&self) -> f32 {
        let golf_ball_count_power = -(self.golf_balls as f32) / 3.0;
        let impulse_score = self.stage_impulses.iter().sum::<u16>() as f32 / 4000.0;
        self.apogee * (3.0 - f32::exp(golf_ball_count_power) - impulse_score)
    }

    /// Decide if a given ScoringCriteria is a valid solution to be considered further.
    pub fn invalid(&self) -> bool {
        self.apogee < CONFIG.min_altitude_m || self.apogee > CONFIG.max_altitude_m
    }
}

impl Display for ScoringCriteria {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let impulse = if self.stage_impulses.len() == 1 {
            format!("Impulse: {}", self.stage_impulses[0])
        } else {
            format!("{}, Total Impulse: {}",
                self.stage_impulses.iter()
                    .enumerate().map(|(stage_number, impulse)| format!("Stage #{} impulse {}ns", stage_number + 1, impulse))
                    .join(", "),
                self.stage_impulses.iter().sum::<u16>()
            )
        };
        write!(f, "Apogee: {}, Golf Balls: {}, {}",
               self.apogee, self.golf_balls, impulse)
    }
}