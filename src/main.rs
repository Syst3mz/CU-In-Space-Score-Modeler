mod scoring_criteria;
mod config;
mod simulation;
mod impulse_iterator;

use std::io::Write;
use std::sync::LazyLock;
use crate::config::Config;
use crate::scoring_criteria::ScoringCriteria;
use crate::simulation::compute_stage_apogee;

static CONFIG: LazyLock<Config> = LazyLock::new(|| get_config());

const CONFIG_PATH: &'static str = "config.json";
fn read_in_config() -> anyhow::Result<Config> {
    let json_text = std::fs::read_to_string(CONFIG_PATH)?;
    Ok(serde_json::from_str::<Config>(&json_text)?)
}
fn write_config_to_file(config: &Config) -> anyhow::Result<()> {
    let mut file = std::fs::File::create(CONFIG_PATH)?;
    let text = serde_json::to_string_pretty(config)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}
fn get_config() -> Config {
    read_in_config().unwrap_or_else(|e| {
        println!("{}", e);
        println!("Unable to read in config file due to above error. A new one will be generated and written to disk.");
        let config = Config::default();

        write_config_to_file(&config).unwrap_or_else(|e| {
            println!("{}", e);
            println!("Unable to write out config file due to above error. Program will proceed with defaults.");
            println!("Default configuration:\n{:#?}", &config)
        });

        config
    })
}
fn compute_multi_stage_apogee(stage_masses: &Vec<f32>, stage_impulses: &Vec<u16>, golf_balls_mass: f32) -> f32 {
    let mut apogee = 0.0;

    for stage_index in 0..stage_impulses.len() {
        let mass = stage_masses[stage_index] + golf_balls_mass;
        apogee += compute_stage_apogee(mass, stage_impulses[stage_index] as f32)
    }

    apogee
}

fn main() {
    let mut best = vec![];
    let stage_masses = CONFIG.stages.stage_masses();

    // test every possible combination of staged impulse and golf balls.
    for golf_balls in 0..CONFIG.max_golf_balls {
        let golf_ball_mass = golf_balls as f32 * CONFIG.golf_ball_mass_kg;

        for stage_impulses in impulse_iterator::new(stage_masses.len(), CONFIG.max_impulse_ns) {
            let setup = ScoringCriteria::new(
                compute_multi_stage_apogee(&stage_masses, &stage_impulses, golf_ball_mass),
                golf_balls,
                stage_impulses,
            );

            // if the score is invalid, just move on.
            if setup.invalid() {
                continue;
            }

            let score = setup.score();

            if best.len() == CONFIG.max_tracked_solutions {
                for best_score_index in 0..best.len() {
                    if score > best[best_score_index].1 {
                        best.insert(best_score_index, (setup, score))
                    }

                    // drop off the worst after replacement
                    best.pop();
                    break;
                }
            } else {
                best.push((setup, score));
            }
        }
        println!("Finished {} / {}", golf_balls + 1, CONFIG.max_golf_balls)
    }
    let mut output = String::new();

    for (index, (setup, score)) in best.iter().enumerate() {
        let text = format!("#{}: {} @ {}", index + 1, setup, score);
        println!("{}", &text);
        output += &text;
        output += "\n"
    }

    std::fs::write(&CONFIG.output_file, output.as_bytes())
        .expect("Unable to write the output file. Running from command line will let you view the results.")
}