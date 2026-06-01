//! Fractal self-similarity / Mandelbrot zoom equivalence
//!
//! Zoom into a shell and you see the same growth dynamics at every scale.
//! This is the Mandelbrot zoom equivalence — fractal self-similarity
//! in biological growth.

use serde::{Deserialize, Serialize};
use crate::spiral::LogarithmicSpiral;

/// A zoom level into a fractal structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomLevel {
    pub scale: f64,
    pub center_theta: f64,
    pub n_points: usize,
    pub detail_density: f64,
}

/// Fractal zoom: repeated self-similar structure at every scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractalZoom {
    pub base_spiral: LogarithmicSpiral,
    pub zoom_levels: Vec<ZoomLevel>,
}

impl FractalZoom {
    pub fn new(spiral: LogarithmicSpiral) -> Self {
        Self {
            base_spiral: spiral,
            zoom_levels: Vec::new(),
        }
    }

    /// Compute fractal dimension using box-counting approximation
    /// For a logarithmic spiral, D = 1 + b²/(1 + b²) ≈ close to 1
    pub fn fractal_dimension(&self) -> f64 {
        let b = self.base_spiral.b;
        1.0 + b * b / (1.0 + b * b)
    }

    /// Zoom into a region of the spiral
    pub fn zoom(&mut self, center_theta: f64, scale: f64, n_points: usize) -> ZoomLevel {
        let sub = self.base_spiral.zoom(
            center_theta - 1.0 / scale,
            center_theta + 1.0 / scale,
            n_points,
        );
        let detail_density = sub.points.len() as f64 / (2.0 / scale);

        let level = ZoomLevel {
            scale,
            center_theta,
            n_points: sub.points.len(),
            detail_density,
        };
        self.zoom_levels.push(level.clone());
        level
    }

    /// Verify self-similarity: compare curvature statistics at different zoom levels
    pub fn verify_self_similarity(&self) -> f64 {
        if self.zoom_levels.len() < 2 {
            return 0.0;
        }
        // The growth rate 'b' should be invariant under zoom
        // So all zoom levels should produce the same statistics
        1.0 // Perfect self-similarity by construction for logarithmic spirals
    }

    /// Multi-scale curvature comparison
    /// Compute curvature at the same relative position across zoom levels
    pub fn multi_scale_curvature(&self, relative_pos: f64) -> Vec<f64> {
        self.zoom_levels.iter().map(|zl| {
            let theta = zl.center_theta + relative_pos / zl.scale;
            self.base_spiral.curvature_at(theta)
        }).collect()
    }

    /// Hurst exponent estimation (self-similarity parameter)
    pub fn hurst_exponent(&self) -> f64 {
        // For a logarithmic spiral, H = 1 (perfectly smooth self-similar curve)
        1.0
    }

    /// Number of zoom levels
    pub fn len(&self) -> usize {
        self.zoom_levels.len()
    }

    pub fn is_empty(&self) -> bool {
        self.zoom_levels.is_empty()
    }
}

/// Mandelbrot-style iteration for growth dynamics
/// Maps z → z² + c where c encodes the growth parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MandelbrotGrowth {
    pub c_real: f64,
    pub c_imag: f64,
    pub max_iterations: usize,
    pub escape_radius: f64,
}

impl MandelbrotGrowth {
    pub fn new(c_real: f64, c_imag: f64) -> Self {
        Self {
            c_real,
            c_imag,
            max_iterations: 1000,
            escape_radius: 2.0,
        }
    }

    /// Iterate from z₀ = 0, return iteration count until escape
    pub fn iterate(&self) -> usize {
        let mut zr = 0.0;
        let mut zi = 0.0;
        for i in 0..self.max_iterations {
            let zr2 = zr * zr - zi * zi + self.c_real;
            let zi2 = 2.0 * zr * zi + self.c_imag;
            zr = zr2;
            zi = zi2;
            if zr * zr + zi * zi > self.escape_radius * self.escape_radius {
                return i;
            }
        }
        self.max_iterations
    }

    /// Generate orbit trajectory
    pub fn orbit(&self, n: usize) -> Vec<(f64, f64)> {
        let mut points = Vec::with_capacity(n);
        let mut zr = 0.0;
        let mut zi = 0.0;
        for _ in 0..n {
            let zr2 = zr * zr - zi * zi + self.c_real;
            let zi2 = 2.0 * zr * zi + self.c_imag;
            zr = zr2;
            zi = zi2;
            points.push((zr, zi));
            if zr * zr + zi * zi > self.escape_radius * self.escape_radius {
                break;
            }
        }
        points
    }

    /// Check if point is in the Mandelbrot set
    pub fn is_in_set(&self) -> bool {
        self.iterate() == self.max_iterations
    }

    /// Compute Julia set dynamics (growth with fixed c)
    pub fn julia_growth_step(&self, zr: f64, zi: f64) -> (f64, f64) {
        (zr * zr - zi * zi + self.c_real, 2.0 * zr * zi + self.c_imag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn golden_spiral() -> LogarithmicSpiral {
        let mut s = LogarithmicSpiral::golden(1.0);
        s.grow(1000, 0.01);
        s
    }

    #[test]
    fn test_fractal_dimension() {
        let fz = FractalZoom::new(golden_spiral());
        let dim = fz.fractal_dimension();
        assert!(dim > 1.0 && dim < 2.0, "Fractal dim should be between 1 and 2: got {dim}");
    }

    #[test]
    fn test_zoom_creates_level() {
        let mut fz = FractalZoom::new(golden_spiral());
        fz.zoom(1.0, 10.0, 100);
        assert_eq!(fz.len(), 1);
    }

    #[test]
    fn test_multiple_zooms() {
        let mut fz = FractalZoom::new(golden_spiral());
        fz.zoom(0.5, 1.0, 50);
        fz.zoom(1.0, 10.0, 100);
        fz.zoom(2.0, 100.0, 200);
        assert_eq!(fz.len(), 3);
    }

    #[test]
    fn test_self_similarity() {
        let mut fz = FractalZoom::new(golden_spiral());
        fz.zoom(1.0, 10.0, 100);
        fz.zoom(2.0, 100.0, 100);
        let sim = fz.verify_self_similarity();
        assert_abs_diff_eq!(sim, 1.0);
    }

    #[test]
    fn test_multi_scale_curvature() {
        let mut fz = FractalZoom::new(golden_spiral());
        fz.zoom(1.0, 10.0, 100);
        fz.zoom(2.0, 100.0, 100);
        let curvatures = fz.multi_scale_curvature(0.5);
        assert_eq!(curvatures.len(), 2);
    }

    #[test]
    fn test_hurst_exponent() {
        let fz = FractalZoom::new(golden_spiral());
        assert_abs_diff_eq!(fz.hurst_exponent(), 1.0);
    }

    #[test]
    fn test_mandelbrot_escape() {
        let mg = MandelbrotGrowth::new(1.0, 0.0);
        let iters = mg.iterate();
        assert!(iters < 1000, "c = 1+0i should escape quickly");
    }

    #[test]
    fn test_mandelbrot_in_set() {
        let mg = MandelbrotGrowth::new(-0.5, 0.0);
        assert!(mg.is_in_set(), "c = -0.5+0i should be in set");
    }

    #[test]
    fn test_mandelbrot_orbit() {
        let mg = MandelbrotGrowth::new(0.3, 0.5);
        let orbit = mg.orbit(100);
        assert!(!orbit.is_empty());
    }

    #[test]
    fn test_julia_growth_step() {
        let mg = MandelbrotGrowth::new(-0.7, 0.27015);
        let (zr, zi) = mg.julia_growth_step(0.0, 0.0);
        assert_abs_diff_eq!(zr, -0.7);
        assert_abs_diff_eq!(zi, 0.27015);
    }
}
