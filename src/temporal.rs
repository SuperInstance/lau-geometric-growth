//! Temporal recording in spatial geometry
//!
//! Time maps to space through growth rings. Each ring records the state
//! of the organism at a moment in time. You can read the entire temporal
//! history by examining the spatial structure.

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// A time slice recorded in spatial geometry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlice {
    /// Spatial position (ring index)
    pub ring_index: usize,
    /// Physical time (seconds since epoch)
    pub physical_time: f64,
    /// Spatial radius at this time
    pub radius: f64,
    /// Curvature at this time
    pub curvature: f64,
    /// Growth rate at this time
    pub growth_rate: f64,
    /// Arbitrary data payload
    pub data: Option<serde_json::Value>,
}

/// Temporal recorder: maps time → space through growth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalRecorder {
    pub slices: Vec<TimeSlice>,
    /// Base growth rate
    pub growth_rate: f64,
    /// Current radius
    pub current_radius: f64,
    /// Accumulated angle
    pub accumulated_angle: f64,
}

impl TemporalRecorder {
    pub fn new(growth_rate: f64) -> Self {
        Self {
            slices: Vec::new(),
            growth_rate,
            current_radius: 0.0,
            accumulated_angle: 0.0,
        }
    }

    /// Record a moment in time as a spatial growth increment
    pub fn record(&mut self, data: Option<serde_json::Value>) -> &TimeSlice {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        let curvature = if self.current_radius > 0.0 {
            1.0 / self.current_radius
        } else {
            f64::INFINITY
        };

        let growth_increment = self.growth_rate * (1.0 + 0.01 * self.accumulated_angle);
        self.current_radius += growth_increment;
        self.accumulated_angle += curvature * growth_increment;

        self.slices.push(TimeSlice {
            ring_index: self.slices.len(),
            physical_time: now,
            radius: self.current_radius,
            curvature: if curvature.is_finite() { curvature } else { 1e10 },
            growth_rate: growth_increment,
            data,
        });

        self.slices.last().unwrap()
    }

    /// Read the temporal history from spatial geometry
    pub fn read_history(&self) -> &[TimeSlice] {
        &self.slices
    }

    /// Extract time series of radii
    pub fn radius_time_series(&self) -> Vec<(f64, f64)> {
        self.slices.iter().map(|s| (s.physical_time, s.radius)).collect()
    }

    /// Extract time series of curvatures
    pub fn curvature_time_series(&self) -> Vec<(f64, f64)> {
        self.slices.iter().map(|s| (s.physical_time, s.curvature)).collect()
    }

    /// Zoom into a temporal window (spatial region)
    pub fn zoom_time(&self, start_idx: usize, end_idx: usize) -> &[TimeSlice] {
        if start_idx >= self.slices.len() || end_idx > self.slices.len() {
            return &[];
        }
        &self.slices[start_idx..end_idx]
    }

    /// Compute growth acceleration (second derivative of radius)
    pub fn growth_acceleration(&self) -> Vec<f64> {
        if self.slices.len() < 3 {
            return Vec::new();
        }
        let rates: Vec<f64> = self.slices.iter().map(|s| s.growth_rate).collect();
        rates.windows(3)
            .map(|w| w[2] - 2.0 * w[1] + w[0])
            .collect()
    }

    /// Time span covered
    pub fn time_span(&self) -> f64 {
        if self.slices.len() < 2 {
            return 0.0;
        }
        self.slices.last().unwrap().physical_time - self.slices.first().unwrap().physical_time
    }

    /// Find the slice closest to a given radius (spatial lookup → temporal)
    pub fn time_at_radius(&self, target_radius: f64) -> Option<&TimeSlice> {
        self.slices.iter().min_by(|a, b| {
            (a.radius - target_radius).abs().partial_cmp(&(b.radius - target_radius).abs()).unwrap()
        })
    }

    /// Self-similar temporal pattern detection
    /// Compare early growth pattern to later growth pattern
    pub fn temporal_self_similarity(&self) -> f64 {
        if self.slices.len() < 20 {
            return 0.0;
        }
        let quarter = self.slices.len() / 4;
        let first: Vec<f64> = self.slices[..quarter]
            .windows(2)
            .map(|w| w[1].radius - w[0].radius)
            .collect();
        let third: Vec<f64> = self.slices[2 * quarter..3 * quarter]
            .windows(2)
            .map(|w| w[1].radius - w[0].radius)
            .collect();

        if first.len().min(third.len()) == 0 {
            return 0.0;
        }
        let len = first.len().min(third.len());
        let mut dot = 0.0;
        let mut n1 = 0.0;
        let mut n2 = 0.0;
        for i in 0..len {
            dot += first[i] * third[i];
            n1 += first[i] * first[i];
            n2 += third[i] * third[i];
        }
        if n1 == 0.0 || n2 == 0.0 {
            return 0.0;
        }
        dot / (n1.sqrt() * n2.sqrt())
    }

    /// Number of recorded time slices
    pub fn slice_count(&self) -> usize {
        self.slices.len()
    }

    /// Record n time slices with optional data
    pub fn record_n(&mut self, n: usize) {
        for i in 0..n {
            self.record(Some(serde_json::json!({"step": i})));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_single() {
        let mut tr = TemporalRecorder::new(0.1);
        tr.record(None);
        assert_eq!(tr.slices.len(), 1);
    }

    #[test]
    fn test_record_n() {
        let mut tr = TemporalRecorder::new(0.1);
        tr.record_n(50);
        assert_eq!(tr.slices.len(), 50);
    }

    #[test]
    fn test_radius_grows() {
        let mut tr = TemporalRecorder::new(0.5);
        tr.record_n(10);
        for w in tr.slices.windows(2) {
            assert!(w[1].radius > w[0].radius);
        }
    }

    #[test]
    fn test_read_history() {
        let mut tr = TemporalRecorder::new(0.1);
        tr.record_n(5);
        let h = tr.read_history();
        assert_eq!(h.len(), 5);
    }

    #[test]
    fn test_radius_time_series() {
        let mut tr = TemporalRecorder::new(0.1);
        tr.record_n(5);
        let ts = tr.radius_time_series();
        assert_eq!(ts.len(), 5);
    }

    #[test]
    fn test_curvature_time_series() {
        let mut tr = TemporalRecorder::new(0.1);
        tr.record_n(5);
        let ts = tr.curvature_time_series();
        assert_eq!(ts.len(), 5);
    }

    #[test]
    fn test_zoom_time() {
        let mut tr = TemporalRecorder::new(0.1);
        tr.record_n(20);
        let zoomed = tr.zoom_time(5, 15);
        assert_eq!(zoomed.len(), 10);
        assert_eq!(zoomed[0].ring_index, 5);
    }

    #[test]
    fn test_growth_acceleration() {
        let mut tr = TemporalRecorder::new(0.1);
        tr.record_n(10);
        let accel = tr.growth_acceleration();
        assert_eq!(accel.len(), 8); // n - 2
    }

    #[test]
    fn test_time_at_radius() {
        let mut tr = TemporalRecorder::new(0.1);
        tr.record_n(20);
        let target = tr.slices[10].radius;
        let found = tr.time_at_radius(target);
        assert!(found.is_some());
        assert!((found.unwrap().radius - target).abs() < 0.01);
    }

    #[test]
    fn test_temporal_self_similarity() {
        let mut tr = TemporalRecorder::new(0.1);
        tr.record_n(100);
        let sim = tr.temporal_self_similarity();
        // Growth rate increases slightly with accumulated angle, so similarity should be positive
        assert!(sim > 0.0);
    }

    #[test]
    fn test_slice_count() {
        let mut tr = TemporalRecorder::new(0.1);
        assert_eq!(tr.slice_count(), 0);
        tr.record_n(10);
        assert_eq!(tr.slice_count(), 10);
    }
}
