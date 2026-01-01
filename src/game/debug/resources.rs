//! Debug resources
//!
//! Resources for controlling and tracking debug overlay state.

use bevy::prelude::*;

/// Controls debug overlay visibility and feature toggles
#[derive(Resource, Debug, Default)]
pub struct DebugSettings {
    /// Master toggle for all debug displays
    pub enabled: bool,
    /// Show FPS counter
    pub show_fps: bool,
    /// Show entity count
    pub show_entity_count: bool,
    /// Show network statistics
    pub show_network_stats: bool,
    /// Show nutrient values
    pub show_nutrients: bool,
    /// Show game state
    pub show_game_state: bool,
    /// Visualize network graph edges
    pub show_network_graph: bool,
    /// Show cursor world position
    pub show_cursor_position: bool,
}

impl DebugSettings {
    /// Create settings with all debug features enabled
    #[must_use]
    pub fn all_enabled() -> Self {
        Self {
            enabled: true,
            show_fps: true,
            show_entity_count: true,
            show_network_stats: true,
            show_nutrients: true,
            show_game_state: true,
            show_network_graph: false, // Off by default, can be heavy
            show_cursor_position: true,
        }
    }

    /// Toggle the master enabled flag
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    /// Toggle network graph visualization
    pub fn toggle_network_graph(&mut self) {
        self.show_network_graph = !self.show_network_graph;
    }
}

/// Tracks frame timing for FPS calculation
#[derive(Resource, Debug)]
pub struct FrameTimeTracker {
    /// Recent frame times in seconds
    frame_times: Vec<f32>,
    /// Maximum number of samples to keep
    max_samples: usize,
}

impl Default for FrameTimeTracker {
    fn default() -> Self {
        Self::new(60) // Default to 60 samples (1 second at 60 FPS)
    }
}

impl FrameTimeTracker {
    /// Create a new tracker with specified sample count
    #[must_use]
    pub fn new(max_samples: usize) -> Self {
        Self {
            frame_times: Vec::with_capacity(max_samples),
            max_samples,
        }
    }

    /// Record a frame time
    pub fn record(&mut self, delta: f32) {
        if self.frame_times.len() >= self.max_samples {
            self.frame_times.remove(0);
        }
        self.frame_times.push(delta);
    }

    /// Get the average FPS over the sample window
    #[must_use]
    pub fn average_fps(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        let avg_delta: f32 = self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32;
        if avg_delta > 0.0 {
            1.0 / avg_delta
        } else {
            0.0
        }
    }

    /// Get the minimum FPS (worst frame) over the sample window
    #[must_use]
    pub fn min_fps(&self) -> f32 {
        self.frame_times
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|max_delta| {
                if *max_delta > 0.0 {
                    1.0 / max_delta
                } else {
                    0.0
                }
            })
            .unwrap_or(0.0)
    }

    /// Get the number of recorded samples
    #[must_use]
    pub fn sample_count(&self) -> usize {
        self.frame_times.len()
    }

    /// Clear all recorded frame times
    pub fn clear(&mut self) {
        self.frame_times.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // DebugSettings tests
    #[test]
    fn test_debug_settings_default_disabled() {
        let settings = DebugSettings::default();
        assert!(!settings.enabled);
    }

    #[test]
    fn test_debug_settings_all_enabled() {
        let settings = DebugSettings::all_enabled();
        assert!(settings.enabled);
        assert!(settings.show_fps);
        assert!(settings.show_entity_count);
        assert!(settings.show_network_stats);
        assert!(settings.show_nutrients);
        assert!(settings.show_game_state);
        assert!(settings.show_cursor_position);
        // Network graph off by default even in all_enabled
        assert!(!settings.show_network_graph);
    }

    #[test]
    fn test_debug_settings_toggle() {
        let mut settings = DebugSettings::default();
        assert!(!settings.enabled);

        settings.toggle();
        assert!(settings.enabled);

        settings.toggle();
        assert!(!settings.enabled);
    }

    #[test]
    fn test_debug_settings_toggle_network_graph() {
        let mut settings = DebugSettings::default();
        assert!(!settings.show_network_graph);

        settings.toggle_network_graph();
        assert!(settings.show_network_graph);

        settings.toggle_network_graph();
        assert!(!settings.show_network_graph);
    }

    #[test]
    fn test_debug_settings_is_resource() {
        fn assert_resource<T: Resource>() {}
        assert_resource::<DebugSettings>();
    }

    // FrameTimeTracker tests
    #[test]
    fn test_frame_time_tracker_default() {
        let tracker = FrameTimeTracker::default();
        assert_eq!(tracker.sample_count(), 0);
        assert_eq!(tracker.max_samples, 60);
    }

    #[test]
    fn test_frame_time_tracker_new() {
        let tracker = FrameTimeTracker::new(100);
        assert_eq!(tracker.max_samples, 100);
    }

    #[test]
    fn test_frame_time_tracker_record() {
        let mut tracker = FrameTimeTracker::new(5);
        tracker.record(0.016);
        assert_eq!(tracker.sample_count(), 1);

        tracker.record(0.017);
        tracker.record(0.015);
        assert_eq!(tracker.sample_count(), 3);
    }

    #[test]
    fn test_frame_time_tracker_average_fps() {
        let mut tracker = FrameTimeTracker::new(5);
        tracker.record(0.016); // ~62.5 FPS
        tracker.record(0.016);
        tracker.record(0.016);

        let fps = tracker.average_fps();
        assert!((fps - 62.5).abs() < 1.0, "Expected ~62.5 FPS, got {}", fps);
    }

    #[test]
    fn test_frame_time_tracker_min_fps() {
        let mut tracker = FrameTimeTracker::new(5);
        tracker.record(0.016); // ~62 FPS
        tracker.record(0.033); // ~30 FPS (worst frame)
        tracker.record(0.016); // ~62 FPS

        let min_fps = tracker.min_fps();
        assert!(
            (min_fps - 30.3).abs() < 1.0,
            "Expected ~30 FPS, got {}",
            min_fps
        );
    }

    #[test]
    fn test_frame_time_tracker_max_samples() {
        let mut tracker = FrameTimeTracker::new(3);
        tracker.record(0.1);
        tracker.record(0.2);
        tracker.record(0.3);
        tracker.record(0.4); // Should push out 0.1

        assert_eq!(tracker.sample_count(), 3);
    }

    #[test]
    fn test_frame_time_tracker_empty() {
        let tracker = FrameTimeTracker::new(5);
        assert_eq!(tracker.average_fps(), 0.0);
        assert_eq!(tracker.min_fps(), 0.0);
    }

    #[test]
    fn test_frame_time_tracker_zero_delta() {
        let mut tracker = FrameTimeTracker::new(5);
        tracker.record(0.0);

        // Should handle zero gracefully
        assert_eq!(tracker.average_fps(), 0.0);
    }

    #[test]
    fn test_frame_time_tracker_clear() {
        let mut tracker = FrameTimeTracker::new(5);
        tracker.record(0.016);
        tracker.record(0.016);
        assert_eq!(tracker.sample_count(), 2);

        tracker.clear();
        assert_eq!(tracker.sample_count(), 0);
    }

    #[test]
    fn test_frame_time_tracker_is_resource() {
        fn assert_resource<T: Resource>() {}
        assert_resource::<FrameTimeTracker>();
    }
}
