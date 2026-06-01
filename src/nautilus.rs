//! Nautilus chamber growth
//!
//! Each chamber is determined by gas pressure, which is a function of
//! existing chambers. The nautilus builds its shell incrementally —
//! each new chamber's shape and size depends on what came before.

use serde::{Deserialize, Serialize};

/// A single nautilus chamber
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chamber {
    pub index: usize,
    pub inner_radius: f64,
    pub outer_radius: f64,
    pub volume: f64,
    pub gas_pressure: f64,
    pub wall_thickness: f64,
    pub angle_span: f64,
}

/// Nautilus chamber growth simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NautilusChamber {
    pub chambers: Vec<Chamber>,
    /// Growth rate factor
    pub growth_factor: f64,
    /// Target gas pressure
    pub target_pressure: f64,
    /// Wall thickness factor
    pub wall_factor: f64,
    /// Current total angle
    pub total_angle: f64,
}

impl NautilusChamber {
    pub fn new(growth_factor: f64) -> Self {
        Self {
            chambers: Vec::new(),
            growth_factor,
            target_pressure: 1.0,
            wall_factor: 0.05,
            total_angle: 0.0,
        }
    }

    /// Compute gas pressure for next chamber based on existing chambers
    /// Pressure = target × (1 + deviation from equilibrium)
    pub fn compute_pressure(&self) -> f64 {
        if self.chambers.is_empty() {
            return self.target_pressure;
        }
        let last = self.chambers.last().unwrap();
        // Pressure adjusts based on difference from target
        let deviation = last.gas_pressure - self.target_pressure;
        self.target_pressure * (1.0 - 0.3 * deviation)
    }

    /// Compute angle span based on growth rate and curvature
    pub fn compute_angle_span(&self) -> f64 {
        let base = std::f64::consts::FRAC_PI_2;
        if self.chambers.is_empty() {
            return base;
        }
        let last = self.chambers.last().unwrap();
        // Angle span decreases as shell grows (chambers get smaller relative to radius)
        base * (1.0 + 0.1 / (1.0 + last.outer_radius))
    }

    /// Grow the next chamber
    pub fn grow_chamber(&mut self) -> &Chamber {
        let idx = self.chambers.len();
        let pressure = self.compute_pressure();
        let angle_span = self.compute_angle_span();

        let (inner_r, outer_r) = if self.chambers.is_empty() {
            let outer = self.growth_factor;
            (0.0, outer)
        } else {
            let last = self.chambers.last().unwrap();
            let growth = self.growth_factor * pressure;
            let inner = last.outer_radius;
            let outer = inner + growth;
            (inner, outer)
        };

        // Volume of a toroidal sector
        let avg_r = (inner_r + outer_r) / 2.0;
        let cross_section = std::f64::consts::PI * ((outer_r - inner_r) / 2.0).powi(2);
        let volume = cross_section * avg_r * angle_span;

        // Wall thickness: thinner as shell grows (material efficiency)
        let wall_thickness = self.wall_factor * (1.0 + 0.5 / (1.0 + outer_r));

        self.total_angle += angle_span;

        self.chambers.push(Chamber {
            index: idx,
            inner_radius: inner_r,
            outer_radius: outer_r,
            volume,
            gas_pressure: pressure,
            wall_thickness,
            angle_span,
        });

        self.chambers.last().unwrap()
    }

    /// Grow n chambers
    pub fn grow_n(&mut self, n: usize) {
        for _ in 0..n {
            self.grow_chamber();
        }
    }

    /// Total shell volume
    pub fn total_volume(&self) -> f64 {
        self.chambers.iter().map(|c| c.volume).sum()
    }

    /// Check logarithmic spiral consistency
    /// In a nautilus, the ratio of consecutive chamber radii should be roughly constant
    pub fn growth_ratio_consistency(&self) -> f64 {
        if self.chambers.len() < 3 {
            return 0.0;
        }
        let ratios: Vec<f64> = self.chambers.windows(2)
            .map(|w| w[1].outer_radius / w[0].outer_radius)
            .collect();
        let mean = ratios.iter().sum::<f64>() / ratios.len() as f64;
        let variance = ratios.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / ratios.len() as f64;
        // Return coefficient of variation (lower = more consistent)
        if mean == 0.0 { return f64::INFINITY; }
        variance.sqrt() / mean
    }

    /// Pressure stability: how close pressures stay to target
    pub fn pressure_stability(&self) -> f64 {
        if self.chambers.is_empty() {
            return 1.0;
        }
        let deviations: Vec<f64> = self.chambers.iter()
            .map(|c| (c.gas_pressure - self.target_pressure).abs())
            .collect();
        let mean_dev = deviations.iter().sum::<f64>() / deviations.len() as f64;
        1.0 / (1.0 + mean_dev)
    }

    /// Chamber count
    pub fn len(&self) -> usize {
        self.chambers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chambers.is_empty()
    }

    /// Last chamber
    pub fn last(&self) -> Option<&Chamber> {
        self.chambers.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_chamber() {
        let mut nautilus = NautilusChamber::new(1.0);
        nautilus.grow_chamber();
        assert_eq!(nautilus.len(), 1);
        assert_eq!(nautilus.chambers[0].index, 0);
    }

    #[test]
    fn test_grow_n() {
        let mut nautilus = NautilusChamber::new(0.5);
        nautilus.grow_n(30);
        assert_eq!(nautilus.len(), 30);
    }

    #[test]
    fn test_radii_increase() {
        let mut nautilus = NautilusChamber::new(0.5);
        nautilus.grow_n(10);
        for w in nautilus.chambers.windows(2) {
            assert!(w[1].outer_radius > w[0].outer_radius);
        }
    }

    #[test]
    fn test_chambers_nested() {
        let mut nautilus = NautilusChamber::new(0.5);
        nautilus.grow_n(10);
        for w in nautilus.chambers.windows(2) {
            assert_abs_diff_eq!(w[1].inner_radius, w[0].outer_radius, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_total_volume_increases() {
        let mut nautilus = NautilusChamber::new(0.5);
        nautilus.grow_chamber();
        let v1 = nautilus.total_volume();
        nautilus.grow_chamber();
        let v2 = nautilus.total_volume();
        assert!(v2 > v1);
    }

    #[test]
    fn test_pressure_oscillation() {
        let mut nautilus = NautilusChamber::new(0.5);
        nautilus.grow_n(20);
        // Pressures should fluctuate around target
        for c in &nautilus.chambers {
            assert!(c.gas_pressure > 0.0);
        }
    }

    #[test]
    fn test_growth_ratio_consistency() {
        let mut nautilus = NautilusChamber::new(0.5);
        nautilus.grow_n(30);
        let cv = nautilus.growth_ratio_consistency();
        assert!(cv < 1.0, "Coefficient of variation should be < 1: got {cv}");
    }

    #[test]
    fn test_pressure_stability() {
        let mut nautilus = NautilusChamber::new(0.5);
        nautilus.grow_n(20);
        let stability = nautilus.pressure_stability();
        assert!(stability > 0.0 && stability <= 1.0);
    }

    #[test]
    fn test_last_chamber() {
        let mut nautilus = NautilusChamber::new(0.5);
        assert!(nautilus.last().is_none());
        nautilus.grow_chamber();
        assert!(nautilus.last().is_some());
        assert_eq!(nautilus.last().unwrap().index, 0);
    }
}
