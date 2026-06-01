//! # Lau Geometric Growth
//!
//! Shell growth as computation — logarithmic spirals, golden ratio growth,
//! and temporal recording in spatial geometry.
//!
//! The conch shell is a logarithmic spiral. It records its entire growth history
//! in its spatial geometry. Each growth increment is determined by the curvature
//! of what already exists. The growth law IS the constructor.

pub mod spiral;
pub mod curvature;
pub mod temporal;
pub mod fibonacci;
pub mod nautilus;
pub mod fractal;
pub mod growth_rate;
pub mod quasicrystal;
pub mod agent_shell;
pub mod ricci_flow;

pub use spiral::{LogarithmicSpiral, SpiralPoint};
pub use curvature::CurvatureInheritance;
pub use temporal::{TemporalRecorder, TimeSlice};
pub use fibonacci::{Phyllotaxis, GoldenAngle};
pub use nautilus::NautilusChamber;
pub use fractal::FractalZoom;
pub use growth_rate::GoldenGrowthRate;
pub use quasicrystal::QuasicrystalGrowth;
pub use agent_shell::AgentShell;
pub use ricci_flow::RicciFlowGrowth;
