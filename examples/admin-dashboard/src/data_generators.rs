//! # Ferrox Front Algorithmic Data Generators
//!
//! High-precision, deterministic mathematical data generators for realistic
//! vector chart simulation: Brownian motion (financial random walk),
//! harmonic multi-frequency waves, stochastic server traffic spikes,
//! and normal bell distributions.

/// A lightweight, deterministic pseudo-random number generator (SplitMix64)
/// that requires zero external dependencies and runs identically in native Rust
/// and WebAssembly linear memory.
#[derive(Debug, Clone)]
pub struct FastRng {
    state: u64,
}

impl FastRng {
    /// Creates a new PRNG with the specified 64-bit seed.
    pub fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0x853c49e6748fea9b } else { seed },
        }
    }

    /// Generates a pseudo-random 64-bit unsigned integer using SplitMix64.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    /// Generates a floating point number uniformly distributed in [0.0, 1.0).
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// Generates a standard normally distributed variable (mean 0, std_dev 1)
    /// using the Box-Muller transform.
    pub fn next_gaussian(&mut self, mean: f64, std_dev: f64) -> f64 {
        let u1 = self.next_f64().max(1e-10);
        let u2 = self.next_f64();
        let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        mean + z0 * std_dev
    }
}

/// Generates a realistic financial random walk trajectory (Brownian Motion with drift).
///
/// # Arguments
/// * `steps` - Number of sequential points to generate.
/// * `start` - Initial price or value level.
/// * `volatility` - Standard deviation of the per-step shock.
/// * `drift` - Constant directional drift per step.
pub fn generate_brownian_motion(steps: usize, start: f64, volatility: f64, drift: f64) -> Vec<f64> {
    let mut rng = FastRng::new(0xdeadbeef1337cafe ^ (steps as u64));
    let mut current = start;
    let mut result = Vec::with_capacity(steps);

    for _ in 0..steps {
        let shock = rng.next_gaussian(drift, volatility);
        current = (current + shock).max(5.0).min(180.0);
        result.push((current * 10.0).round() / 10.0);
    }
    result
}

/// Generates a harmonic multi-frequency waveform with optional gaussian noise.
///
/// Combines primary and secondary sinusoidal harmonics to emulate periodic telemetry,
/// diurnal traffic patterns, or physical sensor vibrations.
pub fn generate_harmonic_wave(steps: usize, amplitude: f64, period: f64, noise: f64) -> Vec<f64> {
    let mut rng = FastRng::new(0x42424242cafebeef);
    let mut result = Vec::with_capacity(steps);
    let base_y = 50.0;

    for i in 0..steps {
        let t = i as f64;
        let primary = (2.0 * std::f64::consts::PI * t / period).sin() * amplitude;
        let secondary = (4.0 * std::f64::consts::PI * t / period).cos() * (amplitude * 0.35);
        let n = if noise > 0.0 { rng.next_gaussian(0.0, noise) } else { 0.0 };
        let val = (base_y + primary + secondary + n).clamp(5.0, 95.0);
        result.push((val * 10.0).round() / 10.0);
    }
    result
}

/// Generates realistic server/network traffic with Poisson-like stochastic bursts and exponential recovery.
///
/// Emulates microservice ingress throughput with sudden spike events.
pub fn generate_traffic_spikes(steps: usize, base: f64, spike_chance: f64, multiplier: f64) -> Vec<f64> {
    let mut rng = FastRng::new(0xfeedface98765432 ^ (steps as u64));
    let mut result = Vec::with_capacity(steps);
    let mut current_spike = 0.0;

    for _ in 0..steps {
        let normal_fluctuation = rng.next_gaussian(0.0, base * 0.08);
        if rng.next_f64() < spike_chance {
            current_spike = base * multiplier * (0.8 + rng.next_f64() * 0.4);
        } else {
            current_spike *= 0.55; // Exponential decay
        }

        let total = (base + normal_fluctuation + current_spike).clamp(10.0, 100.0);
        result.push((total * 10.0).round() / 10.0);
    }
    result
}

/// Generates a normal Gaussian bell distribution centered across the discrete steps.
pub fn generate_normal_bell(steps: usize, mean_val: f64, peak_height: f64) -> Vec<f64> {
    let mut result = Vec::with_capacity(steps);
    let center = (steps as f64 - 1.0) / 2.0;
    let sigma = steps as f64 / 4.0;

    for i in 0..steps {
        let x = i as f64;
        let exponent = -0.5 * ((x - center) / sigma).powi(2);
        let bell = peak_height * exponent.exp();
        let val = (mean_val * 0.15 + bell).clamp(5.0, 100.0);
        result.push((val * 10.0).round() / 10.0);
    }
    result
}

/// Generates categorical metrics distributed with controlled variance.
pub fn generate_categorical_distribution(
    categories: &[&str],
    mean: f64,
    variance: f64,
) -> Vec<(String, f64)> {
    let mut rng = FastRng::new(0x12345678abcdef01);
    categories
        .iter()
        .map(|&cat| {
            let val = (rng.next_gaussian(mean, variance)).clamp(10.0, 100.0);
            (cat.to_string(), (val * 10.0).round() / 10.0)
        })
        .collect()
}
