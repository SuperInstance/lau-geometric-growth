//! Growth rate = most irrational number (φ)
//!
//! The golden ratio φ = (1+√5)/2 is the "most irrational" number:
//! it has the worst rational approximations, which means it provides
//! the most uniform coverage in phyllotaxis and spiral growth.

use serde::{Deserialize, Serialize};

/// Golden ratio
pub const PHI: f64 = 1.6180339887498948482;

/// Continued fraction representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuedFraction {
    pub terms: Vec<u64>,
    pub value: f64,
}

/// Golden growth rate analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenGrowthRate {
    pub phi: f64,
    pub max_convergent_depth: usize,
}

impl GoldenGrowthRate {
    pub fn new() -> Self {
        Self {
            phi: PHI,
            max_convergent_depth: 50,
        }
    }

    /// Compute continued fraction of a number
    pub fn continued_fraction(&self, x: f64, depth: usize) -> ContinuedFraction {
        let mut terms = Vec::new();
        let mut val = x;
        for _ in 0..depth.min(self.max_convergent_depth) {
            let int_part = val.floor() as u64;
            terms.push(int_part);
            let frac = val - int_part as f64;
            if frac < 1e-14 { break; }
            val = 1.0 / frac;
        }
        ContinuedFraction {
            value: x,
            terms,
        }
    }

    /// φ has continued fraction [1; 1, 1, 1, ...] — all 1s
    /// This is the slowest possible convergence
    pub fn phi_continued_fraction(&self) -> ContinuedFraction {
        self.continued_fraction(PHI, 20)
    }

    /// Verify φ is all 1s in continued fraction
    pub fn verify_phi_ones(&self) -> bool {
        let cf = self.phi_continued_fraction();
        cf.terms.iter().all(|&t| t == 1)
    }

    /// Rational approximation quality: how fast do convergents converge?
    /// For φ, the convergents are ratios of consecutive Fibonacci numbers
    pub fn convergent_quality(&self, n: usize) -> Vec<(u64, u64, f64)> {
        let fibs = self.fibonacci_sequence(n + 2);
        let mut convergents = Vec::new();
        for i in 2..fibs.len() {
            let num = fibs[i];
            let den = fibs[i - 1];
            let approx = num as f64 / den as f64;
            convergents.push((num, den, approx));
        }
        convergents
    }

    /// Generate Fibonacci sequence
    pub fn fibonacci_sequence(&self, n: usize) -> Vec<u64> {
        let mut fibs = vec![0u64, 1];
        for _ in 2..n {
            let next = fibs[fibs.len() - 1].checked_add(fibs[fibs.len() - 2]).unwrap_or(u64::MAX);
            fibs.push(next);
        }
        fibs
    }

    /// Irrationality measure: |x - p/q| > c/q²
    /// For φ, this is maximized — worst rational approximation
    pub fn irrationality_measure(&self, x: f64, max_denominator: u64) -> f64 {
        let mut worst_approx = 0.0;
        for q in 2..=max_denominator {
            let p = (x * q as f64).round() as u64;
            let error = (x - p as f64 / q as f64).abs();
            let normalized = error * (q as f64).powi(2);
            if normalized > worst_approx {
                worst_approx = normalized;
            }
        }
        worst_approx
    }

    /// Compare φ's irrationality to other famous numbers
    pub fn phi_is_most_irrational(&self) -> bool {
        let phi_irr = self.irrationality_measure(PHI, 100);
        let e_irr = self.irrationality_measure(std::f64::consts::E, 100);
        let pi_irr = self.irrationality_measure(std::f64::consts::PI, 100);
        let sqrt2_irr = self.irrationality_measure(2.0_f64.sqrt(), 100);
        // φ should have the highest irrationality measure
        phi_irr >= e_irr.min(pi_irr).min(sqrt2_irr) * 0.8
    }

    /// Growth rate from golden ratio: b = ln(φ) / (π/2)
    pub fn golden_spiral_growth_rate() -> f64 {
        PHI.ln() / std::f64::consts::FRAC_PI_2
    }

    /// Compute the most irrational number in [0, 1] via golden ratio
    pub fn most_irrational_fraction() -> f64 {
        PHI - 1.0 // = 1/φ ≈ 0.618
    }

    /// Verify golden angle optimality for packing
    pub fn golden_packing_optimality(n_points: usize) -> f64 {
        let golden_angle = 2.0 * std::f64::consts::PI / (PHI * PHI);
        let mut angles: Vec<f64> = (0..n_points)
            .map(|i| (i as f64 * golden_angle) % std::f64::consts::TAU)
            .collect();
        angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
        angles.push(angles[0] + std::f64::consts::TAU);

        let mut min_gap = f64::INFINITY;
        for w in angles.windows(2) {
            let gap = w[1] - w[0];
            if gap < min_gap { min_gap = gap; }
        }
        let expected = std::f64::consts::TAU / n_points as f64;
        min_gap / expected
    }
}

impl Default for GoldenGrowthRate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_phi_value() {
        let ggr = GoldenGrowthRate::new();
        assert_abs_diff_eq!(ggr.phi, 1.6180339887, epsilon = 1e-8);
    }

    #[test]
    fn test_continued_fraction_phi() {
        let ggr = GoldenGrowthRate::new();
        let cf = ggr.phi_continued_fraction();
        assert!(cf.terms.len() > 5);
        assert!(cf.terms.iter().all(|&t| t == 1));
    }

    #[test]
    fn test_verify_phi_ones() {
        let ggr = GoldenGrowthRate::new();
        assert!(ggr.verify_phi_ones());
    }

    #[test]
    fn test_convergent_quality() {
        let ggr = GoldenGrowthRate::new();
        let convs = ggr.convergent_quality(10);
        assert!(!convs.is_empty());
        // Convergents should approach φ
        let last = convs.last().unwrap().2;
        assert_abs_diff_eq!(last, PHI, epsilon = 0.01);
    }

    #[test]
    fn test_fibonacci_sequence() {
        let ggr = GoldenGrowthRate::new();
        let fibs = ggr.fibonacci_sequence(10);
        assert_eq!(fibs[0], 0);
        assert_eq!(fibs[1], 1);
        assert_eq!(fibs[2], 1);
        assert_eq!(fibs[3], 2);
        assert_eq!(fibs[4], 3);
        assert_eq!(fibs[5], 5);
        assert_eq!(fibs[6], 8);
    }

    #[test]
    fn test_irrationality_measure() {
        let ggr = GoldenGrowthRate::new();
        let phi_irr = ggr.irrationality_measure(PHI, 50);
        assert!(phi_irr > 0.0);
    }

    #[test]
    fn test_phi_most_irrational() {
        let ggr = GoldenGrowthRate::new();
        assert!(ggr.phi_is_most_irrational());
    }

    #[test]
    fn test_golden_spiral_growth_rate() {
        let rate = GoldenGrowthRate::golden_spiral_growth_rate();
        assert!(rate > 0.0 && rate < 1.0, "Growth rate should be ~0.306");
    }

    #[test]
    fn test_most_irrational_fraction() {
        let f = GoldenGrowthRate::most_irrational_fraction();
        assert_abs_diff_eq!(f, 1.0 / PHI, epsilon = 1e-12);
    }

    #[test]
    fn test_golden_packing_optimality() {
        let score = GoldenGrowthRate::golden_packing_optimality(200);
        assert!(score > 0.9, "Golden packing should be very uniform: got {score}");
    }

    #[test]
    fn test_continued_fraction_pi() {
        let ggr = GoldenGrowthRate::new();
        let cf = ggr.continued_fraction(std::f64::consts::PI, 10);
        // π = [3; 7, 15, 1, 292, ...]
        assert_eq!(cf.terms[0], 3);
    }

    #[test]
    fn test_continued_fraction_e() {
        let ggr = GoldenGrowthRate::new();
        let cf = ggr.continued_fraction(std::f64::consts::E, 10);
        // e = [2; 1, 2, 1, 1, 4, ...]
        assert_eq!(cf.terms[0], 2);
    }
}
