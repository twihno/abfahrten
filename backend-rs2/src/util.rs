use std::time::Duration;

use rand::Rng;

pub mod config;
pub mod exit;
pub mod tracing;

// P(0 < X < 1) = 0.9876 with these parameters; clipping in function to [0.0, 1.0]
const PAUSE_NORMAL_DISTRIBUTION_MEAN: f64 = 0.5;
const PAUSE_NORMAL_DISTRIBUTION_STANDARD_DEVIATION: f64 = 0.2;

pub fn get_list_of_pause_durations_from_normal_distribution(
    min: Duration,
    max: Duration,
    count: usize,
) -> Vec<Duration> {
    let mut rng = rand::rng();
    (0..count)
        .map(|_| get_pause_duration_from_normal_distribution(min, max, &mut rng))
        .collect()
}

/// Generates a pause duration between `min` and `max` based on a normal distribution.
pub fn get_pause_duration_from_normal_distribution(
    min: Duration,
    max: Duration,
    rng: &mut impl rand::RngCore,
) -> Duration {
    if max <= min {
        return min;
    }

    let max_min_delta = max - min;

    // Generate a random pause duration based on a normal distribution
    // Use Box-Muller transform
    // See https://en.wikipedia.org/wiki/Box%E2%80%93Muller_transform

    let u1: f64 = rng.random_range(0.0..=1.0);
    let u2: f64 = rng.random_range(0.0..=1.0);

    let normal_distributed_random_number =
        ((-2.0 * u1.ln()).sqrt()) * (2.0 * std::f64::consts::PI * u2).cos();

    // Clamp to [0.0, 1.0] with expected value at 0.5
    let random_factor = ((normal_distributed_random_number
        * PAUSE_NORMAL_DISTRIBUTION_STANDARD_DEVIATION)
        + PAUSE_NORMAL_DISTRIBUTION_MEAN)
        .clamp(0.0, 1.0);

    let pause_millis = random_factor * max_min_delta.as_millis() as f64;

    min + Duration::from_millis(pause_millis as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_get_pause_duration_from_normal_distribution() {
        let min = Duration::from_millis(100);
        let max = Duration::from_secs(1);
        let mut rng = rand::rng();

        let pause_duration = get_pause_duration_from_normal_distribution(min, max, &mut rng);
        assert!(pause_duration >= min);
        assert!(pause_duration <= max);
    }
}
