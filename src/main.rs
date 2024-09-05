mod scoring_criteria;
mod config;

use std::io::Write;
use crate::scoring_criteria::ScoringCriteria;
use crate::config::Config;

/// Assuming $v_0 = 0$, and that the entire impulse is applied instantly, the velocity on the
/// launchpad is (from https://www.calculatorsoup.com/calculators/physics/impulse-momentum.php)
/// $Delta p = m\cdot\Delta v$
/// $\frac{\Delta p}{m} = \Delta v$
fn compute_initial_velocity(mass: f32, impulse: f32) -> f32 {
    impulse / mass
}

/// $v(t) = u + at$
/// where $v(t)$ is the velocity at $t$, $u$ is the initial velocity, and $a$ is the acceleration.
/// The apogee is where $v = 0$, since the only acceleration we care about is gravity $a$ will always
/// be negative.
/// $v = u - at$
/// $0 = u - at$
/// $at = u$
/// $t = \frac{u}{a}$
/// from: https://www.quora.com/A-ball-is-thrown-vertically-upward-with-a-speed-of-20-m-s-When-will-it-reach-the-maximum-height-What-is-the-maximum-height-reached
fn compute_time_of_apogee(initial_velocity: f32, gravity: f32) -> f32 {
    initial_velocity / gravity
}

/// Then we can use the $t$ found above in the following equation to compute apogee.
/// $h = ut+\frac{1}{2}at^2$
/// /// from: https://www.quora.com/A-ball-is-thrown-vertically-upward-with-a-speed-of-20-m-s-When-will-it-reach-the-maximum-height-What-is-the-maximum-height-reached
fn compute_apogee(initial_velocity: f32, time_of_apogee: f32, gravity: f32) -> f32 {
    // powi is raise the preceding f32 to an integer power, in this case 2.
    initial_velocity * time_of_apogee + (0.5 * -gravity * time_of_apogee.powi(2))
}

fn simulate_rocket_apogee(mass: f32, impulse: f32, gravity: f32) -> f32 {
    let initial_velocity = compute_initial_velocity(mass, impulse);
    let time_of_apogee = compute_time_of_apogee(initial_velocity, gravity);
    compute_apogee(initial_velocity, time_of_apogee, gravity)
}


const CONFIG_PATH: &'static str = "config.json";
/// Read in a config JSON file.
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
    let config = get_config();

    let mut best = vec![];

    print!("Sweeping...");
    // test every possible combination of impulse and golf balls.
    for max_impulse in 0..config.max_impulse {
        for golf_balls in 0..config.max_golf_balls {
            let mass = (config.golf_ball_mass_kg * golf_balls as f32) + config.dead_mass_kilo_grams;
            let impulse = max_impulse as f32;

            let setup = ScoringCriteria::new(
                simulate_rocket_apogee(mass, impulse, config.gravity),
                golf_balls,
                impulse,
            );

            // if the score is invalid, just move on.
            if setup.invalid() {
                continue;
            }

            let score = setup.score();
            best.push((setup, score));

            if best.len() > config.max_tracked_solutions {
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

    std::fs::write(config.output_file, output.as_bytes())
        .expect("Unable to write the output file. Running from command line will let you view the results.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_of_apogee() {
        let time = compute_time_of_apogee(20.0, 9.81);
        assert_eq!(time, 2.0387359)
    }

    #[test]
    fn height_of_apogee() {
        let height = compute_apogee(20.0, 2.04, 9.81);
        assert_eq!(height, 20.387352)
    }

    #[test]
    fn rocket_apogee() {
        let height = simulate_rocket_apogee(2.0, 40.0, 9.81);
        assert_eq!(height, 20.38736)
    }
}