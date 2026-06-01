//! Shell curvature inheritance
//!
//! Each growth ring's curvature is a function of the previous ring's curvature.
//! The shell doesn't "compute" — it grows according to local curvature rules.

use serde::{Deserialize, Serialize};

/// A growth ring with inherited curvature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthRing {
    pub index: usize,
    pub radius: f64,
    pub curvature: f64,
    pub thickness: f64,
    pub growth_angle: f64,
}

/// Curvature inheritance engine: each ring inherits curvature from its parent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurvatureInheritance {
    pub rings: Vec<GrowthRing>,
    /// Damping factor for curvature inheritance
    pub damping: f64,
    /// Base growth rate
    pub base_rate: f64,
    /// Minimum curvature (prevents singularity)
    pub min_curvature: f64,
}

impl CurvatureInheritance {
    pub fn new(damping: f64, base_rate: f64) -> Self {
        Self {
            rings: Vec::new(),
            damping,
            base_rate,
            min_curvature: 1e-10,
        }
    }

    /// Seed with an initial ring
    pub fn seed(&mut self, curvature: f64, radius: f64) {
        self.rings.push(GrowthRing {
            index: 0,
            radius,
            curvature,
            thickness: 1.0,
            growth_angle: 0.0,
        });
    }

    /// Compute curvature for next ring based on inheritance law
    /// κ_{n+1} = damping · κ_n + base_rate / (1 + κ_n)
    pub fn next_curvature(&self) -> f64 {
        if self.rings.is_empty() {
            return self.base_rate;
        }
        let prev = self.rings.last().unwrap();
        let inherited = self.damping * prev.curvature;
        let growth_correction = self.base_rate / (1.0 + prev.curvature);
        let k = inherited + growth_correction;
        k.max(self.min_curvature)
    }

    /// Grow one ring
    pub fn grow_ring(&mut self) -> &GrowthRing {
        let idx = self.rings.len();
        let (prev_r, prev_angle) = if idx == 0 {
            (0.0, 0.0)
        } else {
            let prev = &self.rings[idx - 1];
            (prev.radius, prev.growth_angle)
        };
        let curvature = self.next_curvature();
        // Radius grows as function of curvature
        let delta_r = self.base_rate / curvature;
        let radius = prev_r + delta_r;
        // Growth angle rotates
        let growth_angle = prev_angle + curvature;
        // Thickness is proportional to radius
        let thickness = delta_r * 0.1;

        self.rings.push(GrowthRing {
            index: idx,
            radius,
            curvature,
            thickness,
            growth_angle,
        });
        self.rings.last().unwrap()
    }

    /// Grow n rings
    pub fn grow_n(&mut self, n: usize) {
        for _ in 0..n {
            self.grow_ring();
        }
    }

    /// Compute total curvature (integral)
    pub fn total_curvature(&self) -> f64 {
        self.rings.iter().map(|r| r.curvature).sum()
    }

    /// Check curvature monotonicity (should decrease for proper shell)
    pub fn curvature_decreasing(&self) -> bool {
        self.rings.windows(2).all(|w| w[1].curvature <= w[0].curvature || w[0].curvature < 1e-8)
    }

    /// Curvature ratio between first and last ring
    pub fn curvature_ratio(&self) -> f64 {
        if self.rings.len() < 2 {
            return 1.0;
        }
        self.rings.first().unwrap().curvature / self.rings.last().unwrap().curvature
    }

    /// Get curvature time series
    pub fn curvature_series(&self) -> Vec<f64> {
        self.rings.iter().map(|r| r.curvature).collect()
    }

    /// Self-similarity of curvature: compare first half curvature pattern to second half
    pub fn curvature_self_similarity(&self) -> f64 {
        if self.rings.len() < 10 {
            return 0.0;
        }
        let mid = self.rings.len() / 2;
        let first: Vec<f64> = self.rings[..mid].iter().map(|r| r.curvature).collect();
        let second: Vec<f64> = self.rings[mid..2 * mid].iter().map(|r| r.curvature).collect();
        
        // Normalize and compare
        let s1: f64 = first.iter().sum();
        let s2: f64 = second.iter().sum();
        if s1 == 0.0 || s2 == 0.0 {
            return 0.0;
        }
        let mut corr = 0.0;
        for (a, b) in first.iter().zip(second.iter()) {
            corr += (a / s1) * (b / s2);
        }
        corr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn test_seed() {
        let mut ci = CurvatureInheritance::new(0.95, 0.1);
        ci.seed(1.0, 0.1);
        assert_eq!(ci.rings.len(), 1);
        assert_abs_diff_eq!(ci.rings[0].curvature, 1.0);
    }

    #[test]
    fn test_grow_ring() {
        let mut ci = CurvatureInheritance::new(0.95, 0.1);
        ci.seed(1.0, 0.1);
        ci.grow_ring();
        assert_eq!(ci.rings.len(), 2);
        let k = ci.rings[1].curvature;
        assert!(k > 0.0);
    }

    #[test]
    fn test_curvature_inheritance_formula() {
        let mut ci = CurvatureInheritance::new(0.9, 0.2);
        ci.seed(0.5, 0.1);
        let expected = 0.9 * 0.5 + 0.2 / (1.0 + 0.5);
        ci.grow_ring();
        assert_abs_diff_eq!(ci.rings[1].curvature, expected, epsilon = 1e-12);
    }

    #[test]
    fn test_grow_n() {
        let mut ci = CurvatureInheritance::new(0.95, 0.1);
        ci.seed(1.0, 0.1);
        ci.grow_n(99);
        assert_eq!(ci.rings.len(), 100);
    }

    #[test]
    fn test_total_curvature_positive() {
        let mut ci = CurvatureInheritance::new(0.95, 0.1);
        ci.seed(1.0, 0.1);
        ci.grow_n(10);
        assert!(ci.total_curvature() > 0.0);
    }

    #[test]
    fn test_curvature_ratio() {
        let mut ci = CurvatureInheritance::new(0.9, 0.1);
        ci.seed(2.0, 0.1);
        ci.grow_n(20);
        // With damping < 1, curvature should decrease
        assert!(ci.curvature_ratio() > 1.0);
    }

    #[test]
    fn test_curvature_series_length() {
        let mut ci = CurvatureInheritance::new(0.95, 0.1);
        ci.seed(1.0, 0.1);
        ci.grow_n(50);
        let series = ci.curvature_series();
        assert_eq!(series.len(), 51);
    }

    #[test]
    fn test_self_similarity() {
        let mut ci = CurvatureInheritance::new(0.95, 0.1);
        ci.seed(1.0, 0.1);
        ci.grow_n(100);
        let sim = ci.curvature_self_similarity();
        assert!(sim > 0.0);
    }
}
