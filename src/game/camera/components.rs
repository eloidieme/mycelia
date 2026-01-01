//! Camera components

use bevy::prelude::*;

/// Marker component for the main game camera
#[derive(Component, Debug, Default)]
pub struct MainCamera;

/// Camera configuration and state
#[derive(Component, Debug)]
pub struct CameraController {
    /// Current zoom level (1.0 = default)
    pub zoom: f32,
    /// Minimum zoom (zoomed in limit)
    pub min_zoom: f32,
    /// Maximum zoom (zoomed out limit)
    pub max_zoom: f32,
    /// Pan speed in world units per second
    pub pan_speed: f32,
    /// Whether camera is locked to a target entity
    pub locked_target: Option<Entity>,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            min_zoom: 0.25,
            max_zoom: 4.0,
            pan_speed: 500.0,
            locked_target: None,
        }
    }
}

impl CameraController {
    /// Clamp zoom to valid range
    pub fn clamp_zoom(&mut self) {
        self.zoom = self.zoom.clamp(self.min_zoom, self.max_zoom);
    }

    /// Apply zoom delta and clamp
    pub fn apply_zoom_delta(&mut self, delta: f32) {
        self.zoom += delta;
        self.clamp_zoom();
    }
}

/// Resource for camera settings
#[derive(Resource, Debug)]
pub struct CameraSettings {
    /// Default zoom level for new cameras
    pub default_zoom: f32,
    /// Speed of zoom changes
    pub zoom_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            default_zoom: 1.0,
            zoom_speed: 0.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_camera_is_component() {
        // MainCamera should be a marker component
        fn assert_component<T: Component>() {}
        assert_component::<MainCamera>();
    }

    #[test]
    fn test_camera_controller_default_values() {
        let controller = CameraController::default();
        assert_eq!(controller.zoom, 1.0);
        assert!(controller.min_zoom > 0.0);
        assert!(controller.min_zoom < controller.max_zoom);
        assert!(controller.pan_speed > 0.0);
        assert!(controller.locked_target.is_none());
    }

    #[test]
    fn test_camera_controller_zoom_clamping() {
        let controller = CameraController {
            zoom: 1.0,
            min_zoom: 0.5,
            max_zoom: 3.0,
            pan_speed: 500.0,
            locked_target: None,
        };

        // Test clamping logic
        let too_low = 0.1_f32.clamp(controller.min_zoom, controller.max_zoom);
        assert_eq!(too_low, 0.5);

        let too_high = 5.0_f32.clamp(controller.min_zoom, controller.max_zoom);
        assert_eq!(too_high, 3.0);

        let in_range = 2.0_f32.clamp(controller.min_zoom, controller.max_zoom);
        assert_eq!(in_range, 2.0);
    }

    #[test]
    fn test_camera_controller_clamp_zoom_method() {
        let mut controller = CameraController {
            zoom: 0.1, // Below min
            min_zoom: 0.5,
            max_zoom: 3.0,
            ..default()
        };
        controller.clamp_zoom();
        assert_eq!(controller.zoom, 0.5);

        controller.zoom = 5.0; // Above max
        controller.clamp_zoom();
        assert_eq!(controller.zoom, 3.0);
    }

    #[test]
    fn test_camera_controller_apply_zoom_delta() {
        let mut controller = CameraController {
            zoom: 1.0,
            min_zoom: 0.5,
            max_zoom: 3.0,
            ..default()
        };

        controller.apply_zoom_delta(0.5);
        assert_eq!(controller.zoom, 1.5);

        controller.apply_zoom_delta(10.0); // Would exceed max
        assert_eq!(controller.zoom, 3.0); // Clamped to max
    }

    #[test]
    fn test_camera_settings_default() {
        let settings = CameraSettings::default();
        assert!(settings.default_zoom > 0.0);
        assert!(settings.zoom_speed > 0.0);
    }

    #[test]
    fn test_camera_settings_is_resource() {
        fn assert_resource<T: Resource>() {}
        assert_resource::<CameraSettings>();
    }
}
