//! Fibonacci phyllotaxis and golden angle
//!
//! Sunflower spirals: optimal packing via golden angle 137.5°.
//! The golden angle divides the circle most uniformly because φ is
//! the "most irrational" number — worst rational approximation.

use serde::{Deserialize, Serialize};

/// Golden ratio
pub const PHI: f64 = 1.6180339887498948482;

/// Golden angle in radians
pub const GOLDEN_ANGLE_RAD: f64 = 2.0 * std::f64::consts::PI * (2.0 / (1.0 + PHI));

/// Golden angle in degrees (~137.508°)
pub const GOLDEN_ANGLE_DEG: f64 = 360.0 / (PHI * PHI);

/// Golden angle helper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenAngle;

impl GoldenAngle {
    /// Golden angle in radians
    pub fn radians() -> f64 {
        GOLDEN_ANGLE_RAD
    }

    /// Golden angle in degrees (~137.508°)
    pub fn degrees() -> f64 {
        360.0 / (PHI * PHI)
    }

    /// Verify optimality: how uniformly does the golden angle distribute points?
    pub fn uniformity_score(n_points: usize) -> f64 {
        if n_points < 2 {
            return 1.0;
        }
        let mut angles: Vec<f64> = (0..n_points)
            .map(|i| (i as f64 * GOLDEN_ANGLE_RAD) % std::f64::consts::TAU)
            .collect();
        angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
        angles.push(angles[0] + std::f64::consts::TAU);

        let mut min_gap = f64::INFINITY;
        for w in angles.windows(2) {
            let gap = w[1] - w[0];
            if gap < min_gap {
                min_gap = gap;
            }
        }
        let expected_gap = std::f64::consts::TAU / n_points as f64;
        min_gap / expected_gap
    }

    /// Compare golden angle to a rational angle for uniformity
    pub fn uniformity_vs_rational(n_points: usize, rational_angle: f64) -> (f64, f64) {
        let golden = Self::uniformity_score(n_points);
        
        let mut angles: Vec<f64> = (0..n_points)
            .map(|i| (i as f64 * rational_angle) % std::f64::consts::TAU)
            .collect();
        angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
        angles.push(angles[0] + std::f64::consts::TAU);
        let mut min_gap = f64::INFINITY;
        for w in angles.windows(2) {
            let gap = w[1] - w[0];
            if gap < min_gap {
                min_gap = gap;
            }
        }
        let expected_gap = std::f64::consts::TAU / n_points as f64;
        let rational = min_gap / expected_gap;

        (golden, rational)
    }
}

/// A phyllotaxis point (seed in a sunflower)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhyllotaxisPoint {
    pub index: usize,
    pub angle: f64,
    pub radius: f64,
    pub x: f64,
    pub y: f64,
}

/// Phyllotaxis pattern generator (Vogel's model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phyllotaxis {
    pub points: Vec<PhyllotaxisPoint>,
    pub divergence_angle: f64,
    pub scaling: f64,
}

impl Phyllotaxis {
    /// Create a golden angle phyllotaxis
    pub fn golden(scaling: f64) -> Self {
        Self {
            points: Vec::new(),
            divergence_angle: GOLDEN_ANGLE_RAD,
            scaling,
        }
    }

    /// Create with custom divergence angle
    pub fn new(divergence_angle: f64, scaling: f64) -> Self {
        Self {
            points: Vec::new(),
            divergence_angle,
            scaling,
        }
    }

    /// Generate n points
    pub fn generate(&mut self, n: usize) {
        self.points.clear();
        for i in 0..n {
            let angle = i as f64 * self.divergence_angle;
            let r = self.scaling * (i as f64).sqrt();
            let x = r * angle.cos();
            let y = r * angle.sin();
            self.points.push(PhyllotaxisPoint {
                index: i,
                angle: angle % std::f64::consts::TAU,
                radius: r,
                x,
                y,
            });
        }
    }

    /// Count visible spiral families (Fibonacci spirals)
    /// In a golden phyllotaxis, you see 21, 34, 55 spirals etc.
    pub fn count_fibonacci_spirals(&self) -> Vec<usize> {
        if self.points.len() < 10 {
            return Vec::new();
        }
        // Count parastichy numbers by looking at nearest neighbors
        let mut spiral_counts: Vec<usize> = Vec::new();
        let n = self.points.len();
        let check_count = n.min(200);

        for i in 0..check_count {
            let p = &self.points[i];
            let mut dists: Vec<(f64, usize)> = Vec::new();
            for j in 0..check_count {
                if i == j { continue; }
                let q = &self.points[j];
                let d = (p.x - q.x).hypot(p.y - q.y);
                dists.push((d, j));
            }
            dists.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            // The angular difference to nearest neighbors reveals spiral families
            if !dists.is_empty() {
                let nearest_idx = dists[0].1;
                // Parastichy number approximation
                let diff = if nearest_idx > i { nearest_idx - i } else { i - nearest_idx };
                if diff > 0 && diff < 100 {
                    spiral_counts.push(diff);
                }
            }
        }

        // Find most common counts (these should be Fibonacci numbers)
        spiral_counts.sort();
        spiral_counts.dedup();
        spiral_counts
    }

    /// Compute packing density
    pub fn packing_density(&self) -> f64 {
        if self.points.len() < 2 {
            return 0.0;
        }
        let max_r = self.points.iter().map(|p| p.radius).fold(f64::NEG_INFINITY, f64::max);
        if max_r <= 0.0 {
            return 0.0;
        }
        let area = std::f64::consts::PI * max_r * max_r;
        // Each point occupies ~π·d²/4 where d is average nearest-neighbor distance
        let mut total_nn_dist = 0.0;
        let sample = self.points.len().min(100);
        for p in &self.points[..sample] {
            let mut min_d = f64::INFINITY;
            for q in &self.points[..sample] {
                if p.index == q.index { continue; }
                let d = (p.x - q.x).hypot(p.y - q.y);
                if d < min_d { min_d = d; }
            }
            total_nn_dist += min_d;
        }
        let avg_nn = total_nn_dist / sample as f64;
        let point_area = std::f64::consts::PI * (avg_nn / 2.0).powi(2);
        (self.points.len() as f64 * point_area) / area
    }

    /// Point count
    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn test_golden_angle_radians() {
        let ga = GoldenAngle::radians();
        assert!(ga > 2.0 && ga < 2.5, "Golden angle should be ~2.3999 rad");
    }

    #[test]
    fn test_golden_angle_degrees() {
        let gd = GoldenAngle::degrees();
        assert!((gd - 137.508).abs() < 0.1, "Should be ~137.5°");
    }

    #[test]
    fn test_uniformity_score() {
        let score = GoldenAngle::uniformity_score(500);
        assert!(score > 0.9, "Golden angle should give high uniformity: got {score}");
    }

    #[test]
    fn test_golden_beats_rational() {
        let (golden, rational) = GoldenAngle::uniformity_vs_rational(500, std::f64::consts::PI / 3.0);
        assert!(golden > rational, "Golden angle should beat rational angles");
    }

    #[test]
    fn test_golden_beats_simple_fraction() {
        let (golden, rational) = GoldenAngle::uniformity_vs_rational(500, std::f64::consts::TAU * 0.4);
        assert!(golden >= rational * 0.95); // Golden should be at least as good
    }

    #[test]
    fn test_phyllotaxis_generation() {
        let mut ph = Phyllotaxis::golden(1.0);
        ph.generate(500);
        assert_eq!(ph.len(), 500);
    }

    #[test]
    fn test_phyllotaxis_radii_increase() {
        let mut ph = Phyllotaxis::golden(1.0);
        ph.generate(100);
        for w in ph.points.windows(2) {
            assert!(w[1].radius >= w[0].radius);
        }
    }

    #[test]
    fn test_phyllotaxis_no_overlap() {
        let mut ph = Phyllotaxis::golden(0.5);
        ph.generate(100);
        // Check that no two points are too close
        for i in 0..ph.points.len().min(50) {
            for j in (i+1)..ph.points.len().min(50) {
                let d = (ph.points[i].x - ph.points[j].x).hypot(ph.points[i].y - ph.points[j].y);
                assert!(d > 0.01, "Points {i} and {j} overlap: d={d}");
            }
        }
    }

    #[test]
    fn test_packing_density() {
        let mut ph = Phyllotaxis::golden(1.0);
        ph.generate(200);
        let density = ph.packing_density();
        assert!(density > 0.0 && density < 2.0);
    }

    #[test]
    fn test_custom_angle() {
        let mut ph = Phyllotaxis::new(0.5, 1.0);
        ph.generate(50);
        assert_eq!(ph.len(), 50);
        assert_abs_diff_eq!(ph.divergence_angle, 0.5);
    }
}
