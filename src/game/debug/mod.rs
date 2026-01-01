//! Debug overlay system
//!
//! Provides toggle-able debug displays for development:
//! - FPS counter with average and minimum
//! - Entity count
//! - Network statistics
//! - Nutrient values
//! - Game state
//! - Cursor world position
//! - Network graph visualization (F4)
//!
//! Toggle with F3 key.

use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub use components::*;
pub use resources::*;

/// Plugin for the debug overlay system
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<DebugSettings>()
            .init_resource::<FrameTimeTracker>()
            // Spawn debug overlay on startup
            .add_systems(Startup, systems::spawn_debug_overlay)
            // Input handling (always active)
            .add_systems(
                Update,
                (
                    systems::toggle_debug_overlay,
                    systems::toggle_network_graph,
                    systems::track_frame_time,
                ),
            )
            // UI updates (always run, but systems check settings internally)
            .add_systems(
                Update,
                (
                    systems::update_overlay_visibility,
                    systems::update_fps_display,
                    systems::update_entity_count_display,
                    systems::update_network_stats_display,
                    systems::update_nutrients_display,
                    systems::update_game_state_display,
                    systems::update_cursor_position_display,
                ),
            )
            // Gizmo rendering (only when GizmoConfigStore available)
            .add_systems(
                Update,
                systems::render_network_graph_debug
                    .run_if(resource_exists::<bevy::gizmos::config::GizmoConfigStore>),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::state::app::StatesPlugin;
    use crate::game::input::CursorWorldPosition;
    use crate::game::network::NetworkStats;
    use crate::game::progression::Nutrients;

    /// Create a minimal test app with just the resources (no UI or input systems)
    fn create_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(StatesPlugin)
            .init_state::<crate::GameState>()
            .init_resource::<NetworkStats>()
            .init_resource::<Nutrients>()
            .init_resource::<CursorWorldPosition>()
            // Register resources manually (UI spawning requires full Bevy)
            .init_resource::<DebugSettings>()
            .init_resource::<FrameTimeTracker>()
            // Only add frame time tracking (toggle systems need ButtonInput from InputPlugin)
            .add_systems(Update, systems::track_frame_time);
        app
    }

    #[test]
    fn test_debug_settings_resource_exists() {
        let mut app = create_test_app();
        app.update();

        assert!(app.world().get_resource::<DebugSettings>().is_some());
    }

    #[test]
    fn test_frame_time_tracker_resource_exists() {
        let mut app = create_test_app();
        app.update();

        assert!(app.world().get_resource::<FrameTimeTracker>().is_some());
    }

    #[test]
    fn test_frame_time_tracker_records_frames() {
        let mut app = create_test_app();

        // Run several frames
        for _ in 0..10 {
            app.update();
        }

        let tracker = app.world().resource::<FrameTimeTracker>();
        assert!(tracker.sample_count() >= 10);
    }

    #[test]
    fn test_debug_overlay_starts_disabled() {
        let mut app = create_test_app();
        app.update();

        let settings = app.world().resource::<DebugSettings>();
        assert!(!settings.enabled);
    }

    #[test]
    fn test_debug_settings_toggle_method() {
        let mut settings = DebugSettings::default();
        assert!(!settings.enabled);

        settings.toggle();
        assert!(settings.enabled);

        settings.toggle();
        assert!(!settings.enabled);
    }

    #[test]
    fn test_frame_time_tracker_calculates_fps() {
        let mut tracker = FrameTimeTracker::new(10);

        // Simulate 60 FPS frames
        for _ in 0..10 {
            tracker.record(1.0 / 60.0);
        }

        let fps = tracker.average_fps();
        assert!((fps - 60.0).abs() < 1.0, "Expected ~60 FPS, got {}", fps);
    }
}
