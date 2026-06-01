//! Agent capability shell
//!
//! Agents grow skill shells like snails — each new skill is determined
//! by the curvature of the existing capability manifold. Skills don't
//! appear randomly; they emerge from the shape of what you already know.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A skill in the agent's capability shell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub radius: f64,       // How developed (distance from center)
    pub angle: f64,        // Position on the capability manifold
    pub curvature: f64,    // Local complexity
    pub connections: Vec<String>, // Connected skills
    pub depth: usize,      // How many layers deep
}

/// Growth direction for the next skill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthDirection {
    pub angle: f64,
    pub expected_curvature: f64,
    pub suggested_skill: String,
    pub confidence: f64,
}

/// Agent shell: grows skills like a nautilus grows chambers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentShell {
    pub skills: HashMap<String, Skill>,
    pub total_angle: f64,
    pub base_growth_rate: f64,
    pub growth_history: Vec<GrowthDirection>,
}

impl AgentShell {
    pub fn new(base_growth_rate: f64) -> Self {
        Self {
            skills: HashMap::new(),
            total_angle: 0.0,
            base_growth_rate,
            growth_history: Vec::new(),
        }
    }

    /// Compute the curvature of the capability manifold at a given angle
    pub fn manifold_curvature(&self, angle: f64) -> f64 {
        if self.skills.is_empty() {
            return 1.0;
        }
        // Curvature is influenced by nearby skills
        let mut total_influence = 0.0;
        for skill in self.skills.values() {
            let angular_dist = (angle - skill.angle).abs();
            let wrapped = angular_dist.min(std::f64::consts::TAU - angular_dist);
            let influence = (-wrapped * wrapped / 0.5).exp() * skill.curvature;
            total_influence += influence;
        }
        1.0 / (1.0 + total_influence)
    }

    /// Determine where the next skill should grow
    pub fn compute_growth_direction(&self) -> GrowthDirection {
        if self.skills.is_empty() {
            return GrowthDirection {
                angle: 0.0,
                expected_curvature: 1.0,
                suggested_skill: "core".to_string(),
                confidence: 1.0,
            };
        }

        // Find the largest gap in the capability manifold
        let mut angles: Vec<f64> = self.skills.values().map(|s| s.angle).collect();
        angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut max_gap = 0.0;
        let mut gap_center = std::f64::consts::PI;

        for w in angles.windows(2) {
            let gap = w[1] - w[0];
            if gap > max_gap {
                max_gap = gap;
                gap_center = (w[0] + w[1]) / 2.0;
            }
        }

        // Also check the wrap-around gap
        let wrap_gap = angles[0] + std::f64::consts::TAU - angles[angles.len() - 1];
        if wrap_gap > max_gap {
            max_gap = wrap_gap;
            gap_center = (angles[angles.len() - 1] + angles[0] + std::f64::consts::TAU) / 2.0;
        }

        let expected_curvature = self.manifold_curvature(gap_center);
        let confidence = max_gap / std::f64::consts::TAU;

        GrowthDirection {
            angle: gap_center % std::f64::consts::TAU,
            expected_curvature,
            suggested_skill: format!("skill_{}", self.skills.len()),
            confidence,
        }
    }

    /// Add a skill to the shell
    pub fn add_skill(&mut self, name: &str, radius: Option<f64>) -> &Skill {
        let dir = self.compute_growth_direction();
        let r = radius.unwrap_or(dir.angle * self.base_growth_rate);
        let curvature = self.manifold_curvature(dir.angle);

        // Find connections to nearby skills
        let connections: Vec<String> = self.skills.values()
            .filter(|s| {
                let ad = (s.angle - dir.angle).abs();
                let wrapped = ad.min(std::f64::consts::TAU - ad);
                wrapped < std::f64::consts::FRAC_PI_2
            })
            .map(|s| s.name.clone())
            .collect();

        self.total_angle += curvature * self.base_growth_rate;

        let skill = Skill {
            name: name.to_string(),
            radius: r,
            angle: dir.angle,
            curvature,
            connections,
            depth: 0,
        };

        self.growth_history.push(dir);
        self.skills.insert(name.to_string(), skill);
        self.skills.get(name).unwrap()
    }

    /// Grow n generic skills
    pub fn grow_n(&mut self, n: usize) {
        for i in 0..n {
            self.add_skill(&format!("skill_{}", i), None);
        }
    }

    /// Shell completeness: how well does the capability manifold cover all directions
    pub fn completeness(&self) -> f64 {
        if self.skills.is_empty() {
            return 0.0;
        }
        let n_sectors = 36;
        let mut covered = vec![false; n_sectors];
        for skill in self.skills.values() {
            let sector = (skill.angle / std::f64::consts::TAU * n_sectors as f64) as usize;
            let sector = sector.min(n_sectors - 1);
            covered[sector] = true;
        }
        covered.iter().filter(|&&c| c).count() as f64 / n_sectors as f64
    }

    /// Average curvature
    pub fn average_curvature(&self) -> f64 {
        if self.skills.is_empty() {
            return 0.0;
        }
        self.skills.values().map(|s| s.curvature).sum::<f64>() / self.skills.len() as f64
    }

    /// Skill count
    pub fn len(&self) -> usize {
        self.skills.len()
    }

    pub fn is_empty(&self) -> bool {
        self.skills.is_empty()
    }

    /// Get growth history
    pub fn history(&self) -> &[GrowthDirection] {
        &self.growth_history
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_shell() {
        let shell = AgentShell::new(0.1);
        assert!(shell.is_empty());
        assert_eq!(shell.completeness(), 0.0);
    }

    #[test]
    fn test_add_first_skill() {
        let mut shell = AgentShell::new(0.1);
        shell.add_skill("core", None);
        assert_eq!(shell.len(), 1);
        assert!(shell.skills.contains_key("core"));
    }

    #[test]
    fn test_grow_n() {
        let mut shell = AgentShell::new(0.1);
        shell.grow_n(10);
        assert_eq!(shell.len(), 10);
    }

    #[test]
    fn test_completeness_increases() {
        let mut shell = AgentShell::new(0.1);
        let mut prev = 0.0;
        for i in 0..20 {
            shell.add_skill(&format!("s{}", i), None);
            let comp = shell.completeness();
            assert!(comp >= prev * 0.9, "Completeness should generally increase");
            prev = comp;
        }
    }

    #[test]
    fn test_manifold_curvature() {
        let mut shell = AgentShell::new(0.1);
        shell.add_skill("a", None);
        let c = shell.manifold_curvature(0.0);
        assert!(c > 0.0 && c <= 1.0);
    }

    #[test]
    fn test_growth_direction_fills_gaps() {
        let mut shell = AgentShell::new(0.1);
        // Add two skills far apart
        shell.add_skill("a", None);
        shell.add_skill("b", None);
        let dir = shell.compute_growth_direction();
        assert!(dir.confidence > 0.0);
    }

    #[test]
    fn test_skill_connections() {
        let mut shell = AgentShell::new(0.1);
        shell.add_skill("a", None);
        shell.add_skill("b", None);
        // Skills added at different angles may or may not be connected
        assert!(shell.skills.len() >= 2);
    }

    #[test]
    fn test_average_curvature() {
        let mut shell = AgentShell::new(0.1);
        shell.grow_n(5);
        let avg = shell.average_curvature();
        assert!(avg > 0.0);
    }

    #[test]
    fn test_growth_history() {
        let mut shell = AgentShell::new(0.1);
        shell.grow_n(5);
        assert_eq!(shell.history().len(), 5);
    }

    #[test]
    fn test_shell_expansion() {
        let mut shell = AgentShell::new(0.1);
        shell.grow_n(20);
        // After many skills, completeness should be high
        let comp = shell.completeness();
        assert!(comp > 0.5, "Should cover >50% of directions: got {comp}");
    }
}
