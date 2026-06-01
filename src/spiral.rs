//! Logarithmic spiral growth: r = a·e^(bθ)
//!
//! The fundamental growth law. When b = ln(φ)/(π/2), this produces
//! the golden spiral — the most self-similar curve in nature.

use nalgebra::{Point2, Vector2, Rotation2};
use serde::{Deserialize, Serialize};

/// A point on a spiral with associated metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiralPoint {
    pub theta: f64,
    pub r: f64,
    pub x: f64,
    pub y: f64,
    pub curvature: f64,
    pub time_index: usize,
}

/// Logarithmic spiral: r = a·e^(bθ)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogarithmicSpiral {
    /// Scale factor 'a'
    pub a: f64,
    /// Growth rate 'b' (ln(φ)/(π/2) for golden spiral)
    pub b: f64,
    /// Generated points
    pub points: Vec<SpiralPoint>,
}

impl LogarithmicSpiral {
    /// Create a golden spiral (b = ln(φ)/(π/2))
    pub fn golden(a: f64) -> Self {
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let b = phi.ln() / (std::f64::consts::FRAC_PI_2);
        Self { a, b, points: Vec::new() }
    }

    /// Create a spiral with custom growth rate
    pub fn new(a: f64, b: f64) -> Self {
        Self { a, b, points: Vec::new() }
    }

    /// Compute radius at angle theta
    pub fn radius_at(&self, theta: f64) -> f64 {
        self.a * (self.b * theta).exp()
    }

    /// Compute curvature at angle theta for logarithmic spiral
    /// κ = 1 / (r · √(1 + b²))
    pub fn curvature_at(&self, theta: f64) -> f64 {
        let r = self.radius_at(theta);
        1.0 / (r * (1.0 + self.b * self.b).sqrt())
    }

    /// Convert polar to cartesian
    pub fn to_cartesian(&self, theta: f64) -> (f64, f64) {
        let r = self.radius_at(theta);
        (r * theta.cos(), r * theta.sin())
    }

    /// Grow the spiral by n_points increments
    pub fn grow(&mut self, n_points: usize, d_theta: f64) {
        let start = self.points.len();
        for i in 0..n_points {
            let theta = (start + i) as f64 * d_theta;
            let r = self.radius_at(theta);
            let (x, y) = self.to_cartesian(theta);
            let curvature = self.curvature_at(theta);
            self.points.push(SpiralPoint {
                theta,
                r,
                x,
                y,
                curvature,
                time_index: start + i,
            });
        }
    }

    /// Check if this is a golden spiral
    pub fn is_golden(&self) -> bool {
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let golden_b = phi.ln() / std::f64::consts::FRAC_PI_2;
        (self.b - golden_b).abs() < 1e-10
    }

    /// Compute arc length between two angles
    pub fn arc_length(&self, theta1: f64, theta2: f64) -> f64 {
        // For logarithmic spiral: L = (r2 - r1) / sqrt(1 + b²)
        let r1 = self.radius_at(theta1);
        let r2 = self.radius_at(theta2);
        (r2 - r1) / (1.0 + self.b * self.b).sqrt()
    }

    /// Self-similarity ratio: ratio of radii at θ and θ + 2π
    pub fn self_similarity_ratio(&self) -> f64 {
        (self.b * 2.0 * std::f64::consts::PI).exp()
    }

    /// Get total angular span
    pub fn total_angle(&self) -> f64 {
        self.points.last().map(|p| p.theta).unwrap_or(0.0)
    }

    /// Interpolate point at arbitrary angle
    pub fn interpolate(&self, theta: f64) -> SpiralPoint {
        let r = self.radius_at(theta);
        let (x, y) = self.to_cartesian(theta);
        SpiralPoint {
            theta,
            r,
            x,
            y,
            curvature: self.curvature_at(theta),
            time_index: 0, // interpolated, no discrete index
        }
    }

    /// Tangent vector at a given angle
    pub fn tangent_at(&self, theta: f64) -> Vector2<f64> {
        // dr/dθ = a·b·e^(bθ), so derivative in polar → cartesian
        let r = self.radius_at(theta);
        let dr = self.a * self.b * (self.b * theta).exp();
        let ct = theta.cos();
        let st = theta.sin();
        let dx = dr * ct - r * st;
        let dy = dr * st + r * ct;
        let len = (dx * dx + dy * dy).sqrt();
        Vector2::new(dx / len, dy / len)
    }

    /// Normal vector (perpendicular to tangent, pointing inward)
    pub fn normal_at(&self, theta: f64) -> Vector2<f64> {
        let t = self.tangent_at(theta);
        Vector2::new(-t.y, t.x)
    }

    /// Mandelbrot zoom: extract a sub-spiral between two angles
    /// The sub-spiral is self-similar to the whole
    pub fn zoom(&self, theta_start: f64, theta_end: f64, n_points: usize) -> LogarithmicSpiral {
        let a_sub = self.radius_at(theta_start);
        let mut sub = LogarithmicSpiral::new(a_sub, self.b);
        let d_theta = (theta_end - theta_start) / n_points as f64;
        sub.grow(n_points, d_theta);
        sub
    }

    /// Number of growth rings
    pub fn ring_count(&self) -> usize {
        self.points.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn test_golden_spiral_creation() {
        let spiral = LogarithmicSpiral::golden(1.0);
        assert!(spiral.is_golden());
    }

    #[test]
    fn test_golden_spiral_b_value() {
        let spiral = LogarithmicSpiral::golden(1.0);
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let expected_b = phi.ln() / std::f64::consts::FRAC_PI_2;
        assert_abs_diff_eq!(spiral.b, expected_b, epsilon = 1e-12);
    }

    #[test]
    fn test_radius_at_zero() {
        let spiral = LogarithmicSpiral::new(2.0, 0.1);
        assert_abs_diff_eq!(spiral.radius_at(0.0), 2.0);
    }

    #[test]
    fn test_radius_growth() {
        let spiral = LogarithmicSpiral::new(1.0, 0.1);
        let r0 = spiral.radius_at(0.0);
        let r1 = spiral.radius_at(std::f64::consts::PI);
        assert!(r1 > r0, "Spiral should grow outward");
    }

    #[test]
    fn test_curvature_decreases() {
        let spiral = LogarithmicSpiral::new(1.0, 0.2);
        let c0 = spiral.curvature_at(0.0);
        let c1 = spiral.curvature_at(5.0);
        assert!(c1 < c0, "Curvature should decrease as spiral grows");
    }

    #[test]
    fn test_cartesian_origin() {
        let spiral = LogarithmicSpiral::new(3.0, 0.1);
        let (x, y) = spiral.to_cartesian(0.0);
        assert_abs_diff_eq!(x, 3.0);
        assert_abs_diff_eq!(y, 0.0);
    }

    #[test]
    fn test_grow_adds_points() {
        let mut spiral = LogarithmicSpiral::golden(1.0);
        spiral.grow(100, 0.1);
        assert_eq!(spiral.points.len(), 100);
    }

    #[test]
    fn test_grow_cumulative() {
        let mut spiral = LogarithmicSpiral::golden(1.0);
        spiral.grow(50, 0.1);
        spiral.grow(50, 0.1);
        assert_eq!(spiral.points.len(), 100);
    }

    #[test]
    fn test_arc_length_positive() {
        let spiral = LogarithmicSpiral::new(1.0, 0.2);
        let len = spiral.arc_length(0.0, std::f64::consts::TAU);
        assert!(len > 0.0);
    }

    #[test]
    fn test_self_similarity_ratio() {
        let spiral = LogarithmicSpiral::golden(1.0);
        let ratio = spiral.self_similarity_ratio();
        assert!(ratio > 1.0, "Golden spiral grows by φ⁴ per full turn");
    }

    #[test]
    fn test_tangent_unit_vector() {
        let spiral = LogarithmicSpiral::golden(1.0);
        let t = spiral.tangent_at(1.0);
        let len = (t.x * t.x + t.y * t.y).sqrt();
        assert_abs_diff_eq!(len, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_zoom_self_similarity() {
        let mut spiral = LogarithmicSpiral::golden(1.0);
        spiral.grow(1000, 0.01);
        let zoomed = spiral.zoom(1.0, 3.0, 200);
        // The zoomed spiral should have the same growth rate
        assert_abs_diff_eq!(zoomed.b, spiral.b, epsilon = 1e-10);
        assert_eq!(zoomed.points.len(), 200);
    }

    #[test]
    fn test_interpolate() {
        let spiral = LogarithmicSpiral::new(1.0, 0.2);
        let p = spiral.interpolate(2.5);
        assert_abs_diff_eq!(p.theta, 2.5);
        let expected_r = spiral.radius_at(2.5);
        assert_abs_diff_eq!(p.r, expected_r, epsilon = 1e-12);
    }

    #[test]
    fn test_total_angle() {
        let mut spiral = LogarithmicSpiral::golden(1.0);
        spiral.grow(10, 0.5);
        assert_abs_diff_eq!(spiral.total_angle(), 4.5, epsilon = 1e-10);
    }
}
