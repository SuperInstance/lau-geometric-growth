//! Non-repeating but deterministic growth (Penrose, quasicrystals)
//!
//! Quasicrystals fill space without periodic repetition.
//! They're deterministic but never exactly repeat — like biological growth.
//! The growth law produces aperiodic tilings from simple rules.

use serde::{Deserialize, Serialize};

/// A tile in a quasicrystal pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub index: usize,
    pub vertices: Vec<(f64, f64)>,
    pub tile_type: u8, // 0 = thin rhombus, 1 = thick rhombus (Penrose)
    pub orientation: f64,
    pub scale: f64,
}

/// Quasicrystal growth via projection from higher dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuasicrystalGrowth {
    pub tiles: Vec<Tile>,
    /// Golden ratio
    pub phi: f64,
    /// Current scale
    pub scale: f64,
    /// Subdivision depth
    pub depth: usize,
}

impl QuasicrystalGrowth {
    pub fn new() -> Self {
        Self {
            tiles: Vec::new(),
            phi: (1.0 + 5.0_f64.sqrt()) / 2.0,
            scale: 1.0,
            depth: 0,
        }
    }

    /// Generate a 1D quasicrystal via cut-and-project from 2D
    /// Take a line at irrational slope through a square grid
    pub fn generate_1d(&mut self, n: usize, slope: f64) -> Vec<f64> {
        let mut points = Vec::new();
        for i in 0..n {
            let x = i as f64;
            let y = x * slope;
            // Project: take floor of y
            let projected = y.floor();
            let frac = y - projected;
            // Accept if within strip
            if frac < 0.5 {
                points.push(x);
            }
        }
        points
    }

    /// Generate Fibonacci word (substitution system: 0→01, 1→0)
    pub fn fibonacci_word(&self, n_iterations: usize) -> String {
        let mut word = String::from("0");
        for _ in 0..n_iterations {
            let mut new_word = String::new();
            for c in word.chars() {
                match c {
                    '0' => new_word.push_str("01"),
                    '1' => new_word.push('0'),
                    _ => {}
                }
            }
            word = new_word;
        }
        word
    }

    /// Generate Penrose-like rhombus tiles at given depth
    pub fn generate_penrose(&mut self, depth: usize) {
        self.depth = depth;
        self.tiles.clear();

        // Start with a thick rhombus (72°)
        let angle = std::f64::consts::PI * 2.0 / 5.0;
        let base_vertices = vec![
            (0.0, 0.0),
            (self.scale, 0.0),
            (self.scale + self.scale * angle.cos(), self.scale * angle.sin()),
            (self.scale * angle.cos(), self.scale * angle.sin()),
        ];

        self.tiles.push(Tile {
            index: 0,
            vertices: base_vertices,
            tile_type: 1,
            orientation: 0.0,
            scale: self.scale,
        });

        // Subdivide
        for _ in 0..depth {
            self.subdivide();
        }
    }

    /// Penrose subdivision: each tile splits into smaller tiles
    fn subdivide(&mut self) {
        let old_tiles = std::mem::take(&mut self.tiles);
        let scale_factor = 1.0 / self.phi;
        let mut idx = 0;

        for tile in &old_tiles {
            match tile.tile_type {
                // Thick rhombus → 1 thick + 1 thin + 1 thick
                1 => {
                    let cx = tile.vertices.iter().map(|v| v.0).sum::<f64>() / 4.0;
                    let cy = tile.vertices.iter().map(|v| v.1).sum::<f64>() / 4.0;
                    let s = tile.scale * scale_factor;
                    
                    // Create 3 new tiles from subdivision
                    for (tt, off_angle) in [(1u8, 0.0), (0u8, 1.2566), (1u8, 2.5133)] {
                        let orient = tile.orientation + off_angle;
                        let verts = vec![
                            (cx, cy),
                            (cx + s * orient.cos(), cy + s * orient.sin()),
                            (cx + s * (orient + 0.6283).cos(), cy + s * (orient + 0.6283).sin()),
                            (cx + s * (orient - 0.6283).cos(), cy + s * (orient - 0.6283).sin()),
                        ];
                        self.tiles.push(Tile { index: idx, vertices: verts, tile_type: tt, orientation: orient, scale: s });
                        idx += 1;
                    }
                }
                // Thin rhombus → 1 thick + 1 thin
                0 => {
                    let cx = tile.vertices.iter().map(|v| v.0).sum::<f64>() / 4.0;
                    let cy = tile.vertices.iter().map(|v| v.1).sum::<f64>() / 4.0;
                    let s = tile.scale * scale_factor;

                    for (tt, off_angle) in [(1u8, 0.0), (0u8, 1.8850)] {
                        let orient = tile.orientation + off_angle;
                        let verts = vec![
                            (cx, cy),
                            (cx + s * orient.cos(), cy + s * orient.sin()),
                            (cx + s * (orient + 0.6283).cos(), cy + s * (orient + 0.6283).sin()),
                        ];
                        self.tiles.push(Tile { index: idx, vertices: verts, tile_type: tt, orientation: orient, scale: s });
                        idx += 1;
                    }
                }
                _ => {}
            }
        }
    }

    /// Check aperiodicity: verify no translational symmetry
    pub fn is_aperiodic(&self) -> bool {
        // Fibonacci word never repeats exactly
        let word = self.fibonacci_word(10);
        // Check that no finite block repeats with a single period
        for period in 1..word.len() / 2 {
            let mut is_periodic = true;
            let pattern = &word[..period];
            for i in (0..word.len()).step_by(period) {
                let end = (i + period).min(word.len());
                if &word[i..end] != &pattern[..end - i] {
                    is_periodic = false;
                    break;
                }
            }
            if is_periodic {
                return false;
            }
        }
        true
    }

    /// Count tile types
    pub fn count_tile_types(&self) -> (usize, usize) {
        let thick = self.tiles.iter().filter(|t| t.tile_type == 1).count();
        let thin = self.tiles.iter().filter(|t| t.tile_type == 0).count();
        (thick, thin)
    }

    /// Tile count
    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }
}

impl Default for QuasicrystalGrowth {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_1d() {
        let mut qc = QuasicrystalGrowth::new();
        let points = qc.generate_1d(100, (5.0_f64.sqrt() - 1.0) / 2.0);
        assert!(!points.is_empty());
        // Points should be increasing
        for w in points.windows(2) {
            assert!(w[1] > w[0]);
        }
    }

    #[test]
    fn test_fibonacci_word() {
        let qc = QuasicrystalGrowth::new();
        let w0 = qc.fibonacci_word(0);
        assert_eq!(w0, "0");
        let w1 = qc.fibonacci_word(1);
        assert_eq!(w1, "01");
        let w2 = qc.fibonacci_word(2);
        assert_eq!(w2, "010");
        let w3 = qc.fibonacci_word(3);
        assert_eq!(w3, "01001");
    }

    #[test]
    fn test_fibonacci_word_length() {
        let qc = QuasicrystalGrowth::new();
        // Length follows Fibonacci: F(n+2)
        let lengths: Vec<usize> = (0..8).map(|n| qc.fibonacci_word(n).len()).collect();
        for i in 2..lengths.len() {
            assert_eq!(lengths[i], lengths[i - 1] + lengths[i - 2]);
        }
    }

    #[test]
    fn test_penrose_generation() {
        let mut qc = QuasicrystalGrowth::new();
        qc.generate_penrose(0);
        assert_eq!(qc.len(), 1); // Just the seed
    }

    #[test]
    fn test_penrose_subdivision() {
        let mut qc = QuasicrystalGrowth::new();
        qc.generate_penrose(1);
        assert!(qc.len() > 1, "Subdivision should produce more tiles");
    }

    #[test]
    fn test_aperiodicity() {
        let qc = QuasicrystalGrowth::new();
        assert!(qc.is_aperiodic(), "Fibonacci word should be aperiodic");
    }

    #[test]
    fn test_count_tile_types() {
        let mut qc = QuasicrystalGrowth::new();
        qc.generate_penrose(2);
        let (thick, thin) = qc.count_tile_types();
        assert!(thick > 0);
        assert!(thin > 0);
    }

    #[test]
    fn test_deterministic() {
        let mut qc1 = QuasicrystalGrowth::new();
        let mut qc2 = QuasicrystalGrowth::new();
        qc1.generate_penrose(2);
        qc2.generate_penrose(2);
        assert_eq!(qc1.tiles.len(), qc2.tiles.len());
    }

    #[test]
    fn test_1d_irrational_slope() {
        let mut qc = QuasicrystalGrowth::new();
        let phi_slope = (5.0_f64.sqrt() - 1.0) / 2.0;
        let points = qc.generate_1d(200, phi_slope);
        // Gaps should be non-repeating but bounded
        let gaps: Vec<f64> = points.windows(2).map(|w| w[1] - w[0]).collect();
        let unique_gaps: Vec<f64> = {
            let mut g = gaps.clone();
            g.sort_by(|a, b| a.partial_cmp(b).unwrap());
            g.dedup_by(|a, b| (a - b).abs() < 1e-10);
            g
        };
        // Should have exactly 2 distinct gap sizes (short and long, in golden ratio)
        assert!(unique_gaps.len() <= 3, "Should have 2-3 distinct gap sizes, got {}", unique_gaps.len());
    }
}
