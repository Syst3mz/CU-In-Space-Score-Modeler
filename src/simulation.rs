use crate::CONFIG;

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

pub fn simulate_rocket_apogee(mass: f32, impulse: f32) -> f32 {
    let initial_velocity = compute_initial_velocity(mass, impulse);
    let time_of_apogee = compute_time_of_apogee(initial_velocity, CONFIG.gravity_ms);
    compute_apogee(initial_velocity, time_of_apogee, CONFIG.gravity_ms)
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
        let height = simulate_rocket_apogee(2.0, 40.0);
        assert_eq!(height, 20.38736)
    }
}