//! Camera system
//!
//! Handles camera behavior:
//! - Dynamic zoom based on network size
//! - Panning controls
//! - Tip-centered lock option
//! - Minimap viewport

use bevy::prelude::*;

pub mod components;
pub mod systems;

pub use components::*;

/// Plugin for the camera system
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraSettings>()
            .add_systems(Startup, systems::spawn_camera)
            .add_systems(
                Update,
                (
                    systems::camera_zoom,
                    systems::camera_pan,
                    systems::camera_follow_target,
                ),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::{mouse::MouseWheel, InputPlugin};

    /// Helper to create a test app with required plugins
    fn create_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(InputPlugin)
            .add_plugins(CameraPlugin);
        app
    }

    #[test]
    fn test_camera_plugin_builds() {
        // CameraPlugin should build without panicking
        let mut app = create_test_app();
        app.update();
    }

    #[test]
    fn test_camera_spawns_on_startup() {
        let mut app = create_test_app();
        app.update();

        // Should have exactly one MainCamera
        let mut query = app.world_mut().query_filtered::<Entity, With<MainCamera>>();
        assert_eq!(query.iter(&app.world()).count(), 1);
    }

    #[test]
    fn test_camera_has_controller() {
        let mut app = create_test_app();
        app.update();

        // MainCamera should have CameraController
        let mut query = app
            .world_mut()
            .query_filtered::<&CameraController, With<MainCamera>>();
        assert_eq!(query.iter(&app.world()).count(), 1);
    }

    #[test]
    fn test_camera_settings_resource_exists() {
        let mut app = create_test_app();
        app.update();

        // CameraSettings resource should exist
        assert!(app.world().get_resource::<CameraSettings>().is_some());
    }
}
