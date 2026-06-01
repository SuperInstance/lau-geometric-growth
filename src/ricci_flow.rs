//! Ricci flow as growth law
//!
//! Curvature drives growth direction, not external planning.
//! The Ricci flow smooths curvature → the shell evolves toward
//! uniform curvature distribution. Growth IS curvature equalization.

use serde::{Deserialize, Serialize};
use nalgebra::DVector;

/// A node in the growth mesh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshNode {
    pub index: usize,
    pub position: f64,
    pub curvature: f64,
    pub velocity: f64,
}

/// Ricci flow growth simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RicciFlowGrowth {
    pub nodes: Vec<MeshNode>,
    /// Time step for flow
    pub dt: f64,
    /// Smoothing parameter
    pub alpha: f64,
    /// Iteration count
    pub iterations: usize,
    /// Curvature history (for temporal recording)
    pub curvature_history: Vec<Vec<f64>>,
}

impl RicciFlowGrowth {
    pub fn new(dt: f64, alpha: f64) -> Self {
        Self {
            nodes: Vec::new(),
            dt,
            alpha,
            iterations: 0,
            curvature_history: Vec::new(),
        }
    }

    /// Initialize with a perturbed circle (seed shape)
    pub fn init_perturbed_circle(&mut self, n_nodes: usize, perturbation: f64) {
        self.nodes.clear();
        for i in 0..n_nodes {
            let angle = i as f64 * std::f64::consts::TAU / n_nodes as f64;
            let r = 1.0 + perturbation * (3.0 * angle).sin();
            let curv = if i > 0 && i < n_nodes - 1 {
                // Discrete curvature from neighbors
                let prev_angle = (i - 1) as f64 * std::f64::consts::TAU / n_nodes as f64;
                let next_angle = (i + 1) as f64 * std::f64::consts::TAU / n_nodes as f64;
                let prev_r = 1.0 + perturbation * (3.0 * prev_angle).sin();
                let next_r = 1.0 + perturbation * (3.0 * next_angle).sin();
                (prev_r + next_r - 2.0 * r) * n_nodes as f64 / (2.0 * std::f64::consts::TAU)
            } else {
                0.0
            };
            self.nodes.push(MeshNode {
                index: i,
                position: r,
                curvature: curv,
                velocity: 0.0,
            });
        }
        self.record_curvature();
    }

    /// Compute discrete curvature for all nodes
    pub fn compute_curvatures(&mut self) {
        let n = self.nodes.len();
        if n < 3 { return; }
        let positions: Vec<f64> = self.nodes.iter().map(|n| n.position).collect();
        for i in 0..n {
            let prev = positions[(i + n - 1) % n];
            let curr = positions[i];
            let next = positions[(i + 1) % n];
            self.nodes[i].curvature = prev + next - 2.0 * curr;
        }
    }

    /// One step of Ricci flow: ∂g/∂t = -2·Ric
    /// In 1D discrete: position += α · curvature · dt
    pub fn step(&mut self) {
        self.compute_curvatures();
        for node in &mut self.nodes {
            // Ricci flow: smooth toward uniform curvature
            node.velocity = -self.alpha * node.curvature;
            node.position += node.velocity * self.dt;
            // Prevent collapse
            if node.position < 0.01 {
                node.position = 0.01;
            }
        }
        self.iterations += 1;
        self.record_curvature();
    }

    /// Run n steps
    pub fn evolve(&mut self, n_steps: usize) {
        for _ in 0..n_steps {
            self.step();
        }
    }

    /// Record current curvature state
    fn record_curvature(&mut self) {
        let curvatures: Vec<f64> = self.nodes.iter().map(|n| n.curvature).collect();
        self.curvature_history.push(curvatures);
    }

    /// Compute total curvature energy (should decrease under Ricci flow)
    pub fn total_curvature_energy(&self) -> f64 {
        self.nodes.iter().map(|n| n.curvature * n.curvature).sum()
    }

    /// Curvature uniformity (standard deviation of curvatures)
    pub fn curvature_uniformity(&self) -> f64 {
        if self.nodes.is_empty() { return 0.0; }
        let mean = self.nodes.iter().map(|n| n.curvature).sum::<f64>() / self.nodes.len() as f64;
        let variance = self.nodes.iter()
            .map(|n| (n.curvature - mean).powi(2))
            .sum::<f64>() / self.nodes.len() as f64;
        variance.sqrt()
    }

    /// Check convergence: has the flow stabilized?
    pub fn has_converged(&self, threshold: f64) -> bool {
        if self.curvature_history.len() < 2 {
            return false;
        }
        let prev = &self.curvature_history[self.curvature_history.len() - 2];
        let curr = &self.curvature_history[self.curvature_history.len() - 1];
        let max_change = prev.iter().zip(curr.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0_f64, f64::max);
        max_change < threshold
    }

    /// The growth law: given current curvature, determine next growth increment
    /// This IS the computation — no separate step needed
    pub fn growth_law(&self, curvature: f64) -> f64 {
        -self.alpha * curvature * self.dt
    }

    /// Node count
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Get curvature history length
    pub fn history_len(&self) -> usize {
        self.curvature_history.len()
    }

    /// Compute the final shape (positions after flow)
    pub fn final_shape(&self) -> Vec<f64> {
        self.nodes.iter().map(|n| n.position).collect()
    }

    /// Compute the average radius
    pub fn average_radius(&self) -> f64 {
        if self.nodes.is_empty() { return 0.0; }
        self.nodes.iter().map(|n| n.position).sum::<f64>() / self.nodes.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn test_init_perturbed() {
        let mut rf = RicciFlowGrowth::new(0.01, 0.1);
        rf.init_perturbed_circle(50, 0.3);
        assert_eq!(rf.len(), 50);
    }

    #[test]
    fn test_curvature_computed() {
        let mut rf = RicciFlowGrowth::new(0.01, 0.1);
        rf.init_perturbed_circle(50, 0.3);
        // Curvatures should be non-zero due to perturbation
        let non_zero = rf.nodes.iter().filter(|n| n.curvature.abs() > 1e-10).count();
        assert!(non_zero > 0);
    }

    #[test]
    fn test_step_decreases_energy() {
        let mut rf = RicciFlowGrowth::new(0.01, 0.5);
        rf.init_perturbed_circle(50, 0.5);
        let e0 = rf.total_curvature_energy();
        rf.step();
        let e1 = rf.total_curvature_energy();
        assert!(e1 < e0, "Energy should decrease: {e1} >= {e0}");
    }

    #[test]
    fn test_evolve_converges() {
        let mut rf = RicciFlowGrowth::new(0.01, 0.5);
        rf.init_perturbed_circle(50, 0.3);
        rf.evolve(500);
        let uniformity = rf.curvature_uniformity();
        assert!(uniformity < 0.1, "Should converge toward uniform: got {uniformity}");
    }

    #[test]
    fn test_history_recorded() {
        let mut rf = RicciFlowGrowth::new(0.01, 0.1);
        rf.init_perturbed_circle(20, 0.1);
        let initial_history = rf.history_len();
        rf.evolve(10);
        assert_eq!(rf.history_len(), initial_history + 10);
    }

    #[test]
    fn test_growth_law() {
        let rf = RicciFlowGrowth::new(0.01, 0.5);
        let g = rf.growth_law(1.0);
        assert_abs_diff_eq!(g, -0.005, epsilon = 1e-10);
    }

    #[test]
    fn test_growth_law_positive_curvature() {
        let rf = RicciFlowGrowth::new(0.01, 0.5);
        let g = rf.growth_law(1.0);
        assert!(g < 0.0, "Positive curvature should shrink");
    }

    #[test]
    fn test_growth_law_negative_curvature() {
        let rf = RicciFlowGrowth::new(0.01, 0.5);
        let g = rf.growth_law(-1.0);
        assert!(g > 0.0, "Negative curvature should expand");
    }

    #[test]
    fn test_total_curvature_energy_positive() {
        let mut rf = RicciFlowGrowth::new(0.01, 0.1);
        rf.init_perturbed_circle(30, 0.5);
        assert!(rf.total_curvature_energy() > 0.0);
    }

    #[test]
    fn test_convergence_detection() {
        let mut rf = RicciFlowGrowth::new(0.01, 0.5);
        rf.init_perturbed_circle(30, 0.1);
        rf.evolve(1000);
        assert!(rf.has_converged(0.001));
    }

    #[test]
    fn test_final_shape_positive() {
        let mut rf = RicciFlowGrowth::new(0.01, 0.5);
        rf.init_perturbed_circle(30, 0.3);
        rf.evolve(100);
        let shape = rf.final_shape();
        assert!(shape.iter().all(|&r| r > 0.0));
    }

    #[test]
    fn test_average_radius() {
        let mut rf = RicciFlowGrowth::new(0.01, 0.1);
        rf.init_perturbed_circle(30, 0.0);
        let avg = rf.average_radius();
        assert_abs_diff_eq!(avg, 1.0, epsilon = 0.01);
    }
}
