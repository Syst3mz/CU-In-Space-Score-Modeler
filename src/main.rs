mod scoring_criteria;
use crate::scoring_criteria::ScoringCriteria;

const GOLF_BALL_MASS_KILOGRAMS: f32 = 0.04592623;

/// Mass of everything that is not payload on the launchpad.
const DEAD_MASS_KILO_GRAMS: f32 = 4.0;
const MAX_IMPULSE: u16 = 5120;
const MAX_GOLF_BALLS: u16 = 1000;
const GRAVITY: f32 = 9.81;

/// How many of the top solutions should be kept.
const MAX_TRACKED_SOLUTIONS: usize = 50;

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
fn compute_time_of_apogee(initial_velocity: f32) -> f32 {
    initial_velocity / GRAVITY
}

/// Then we can use the $t$ found above in the following equation to compute apogee.
/// $h = ut+\frac{1}{2}at^2$
/// /// from: https://www.quora.com/A-ball-is-thrown-vertically-upward-with-a-speed-of-20-m-s-When-will-it-reach-the-maximum-height-What-is-the-maximum-height-reached
fn compute_apogee(initial_velocity: f32, time_of_apogee: f32) -> f32 {
    // powi is raise the preceding f32 to an integer power, in this case 2.
    initial_velocity * time_of_apogee + (0.5 * -GRAVITY * time_of_apogee.powi(2))
}

fn simulate_rocket_apogee(mass: f32, impulse: f32) -> f32 {
    let initial_velocity = compute_initial_velocity(mass, impulse);
    let time_of_apogee = compute_time_of_apogee(initial_velocity);
    compute_apogee(initial_velocity, time_of_apogee)
}

// test every possible combination of impulse and golf balls.
fn main() {
    let mut best = vec![];

    print!("Sweeping...");
    for max_impulse in 0..MAX_IMPULSE {
        for golf_balls in 0..MAX_GOLF_BALLS {
            let mass = (GOLF_BALL_MASS_KILOGRAMS * golf_balls as f32) + DEAD_MASS_KILO_GRAMS;
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

            if best.len() > MAX_TRACKED_SOLUTIONS {
                // Take out the worst tracked solution.
                best.sort_by(|(_, a), (_,b)| b.total_cmp(a));
                best.pop();
            }
        }
    }
    println!("Done!");

    for (index, (setup, score)) in best.iter().enumerate() {
        println!("#{}: {} @ {}", index + 1, setup, score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_of_apogee() {
        let time = compute_time_of_apogee(20.0);
        assert_eq!(time, 2.0387359)
    }

    #[test]
    fn height_of_apogee() {
        let height = compute_apogee(20.0, 2.04);
        assert_eq!(height, 20.387352)
    }

    #[test]
    fn rocket_apogee() {
        let height = simulate_rocket_apogee(2.0, 40.0);
        assert_eq!(height, 20.38736)
    }
}