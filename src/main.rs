mod scoring_criteria;
mod config;
mod simulation;

use std::io::Write;
use std::sync::LazyLock;
use crate::scoring_criteria::ScoringCriteria;
use crate::config::Config;
use crate::simulation::simulate_rocket_apogee;

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

fn main() {
    let mut best = vec![];

    print!("Sweeping...");
    // test every possible combination of impulse and golf balls.
    for max_impulse in 0..CONFIG.max_impulse_ns {
        for golf_balls in 0..CONFIG.max_golf_balls {
            let mass = (CONFIG.golf_ball_mass_kg * golf_balls as f32) + CONFIG.dry_mass_kg;
            let impulse = max_impulse as f32;

            let setup = ScoringCriteria::new(
                simulate_rocket_apogee(mass, impulse),
                golf_balls,
                impulse,
            );

            // if the score is invalid, just move on.
            if setup.invalid() {
                continue;
            }

            let score = setup.score();
            best.push((setup, score));

            if best.len() > CONFIG.max_tracked_solutions {
                // Take out the worst tracked solution.
                best.sort_by(|(_, a), (_,b)| b.total_cmp(a));
                best.pop();
            }
        }
    }
    println!("Done!");
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