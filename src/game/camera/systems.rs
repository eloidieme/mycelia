//! Camera systems

use bevy::prelude::*;

use super::components::{CameraController, CameraSettings, MainCamera};

/// Spawns the main game camera
pub fn spawn_camera(mut commands: Commands, settings: Res<CameraSettings>) {
    commands.spawn((
        Name::new("Main Camera"),
        MainCamera,
        CameraController {
            zoom: settings.default_zoom,
            ..default()
        },
        Camera2d,
        Transform::default(),
    ));
}

/// Handle camera zoom from scroll wheel
pub fn camera_zoom(
    mut scroll_events: EventReader<bevy::input::mouse::MouseWheel>,
    settings: Res<CameraSettings>,
    mut camera_query: Query<(&mut CameraController, &mut OrthographicProjection), With<MainCamera>>,
) {
    let scroll_delta: f32 = scroll_events.read().map(|e| e.y).sum();

    if scroll_delta == 0.0 {
        return;
    }

    for (mut controller, mut projection) in camera_query.iter_mut() {
        // Zoom in = scroll up = positive delta = smaller scale
        controller.apply_zoom_delta(-scroll_delta * settings.zoom_speed);
        projection.scale = controller.zoom;
    }
}

/// Handle camera panning with keyboard
pub fn camera_pan(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&CameraController, &mut Transform), With<MainCamera>>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction == Vec2::ZERO {
        return;
    }

    let direction = direction.normalize();

    for (controller, mut transform) in camera_query.iter_mut() {
        // Don't pan if locked to a target
        if controller.locked_target.is_some() {
            continue;
        }

        let delta = direction * controller.pan_speed * time.delta_secs();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
    }
}

/// Follow locked target if set
pub fn camera_follow_target(
    mut camera_query: Query<(&CameraController, &mut Transform), With<MainCamera>>,
    target_query: Query<&Transform, Without<MainCamera>>,
) {
    for (controller, mut camera_transform) in camera_query.iter_mut() {
        if let Some(target_entity) = controller.locked_target {
            if let Ok(target_transform) = target_query.get(target_entity) {
                camera_transform.translation.x = target_transform.translation.x;
                camera_transform.translation.y = target_transform.translation.y;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
    use bevy::input::InputPlugin;

    /// Helper to create test app with camera spawned
    fn create_camera_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(InputPlugin)
            .init_resource::<CameraSettings>()
            .add_systems(Startup, spawn_camera);
        app.update(); // Run startup to spawn camera
        app
    }

    #[test]
    fn test_spawn_camera_system() {
        let mut app = create_camera_test_app();

        // Verify camera was spawned
        let mut query = app.world_mut().query_filtered::<Entity, With<MainCamera>>();
        assert_eq!(query.iter(&app.world()).count(), 1);
    }

    #[test]
    fn test_spawn_camera_has_orthographic_projection() {
        let mut app = create_camera_test_app();

        // Verify camera has OrthographicProjection (from Camera2d bundle)
        let mut query = app
            .world_mut()
            .query_filtered::<&OrthographicProjection, With<MainCamera>>();
        assert_eq!(query.iter(&app.world()).count(), 1);
    }

    // Camera zoom tests
    #[test]
    fn test_camera_zoom_scroll_up_zooms_in() {
        let mut app = create_camera_test_app();
        app.add_systems(Update, camera_zoom);

        // Get initial zoom
        let initial_zoom = app
            .world_mut()
            .query_filtered::<&CameraController, With<MainCamera>>()
            .single(&app.world())
            .zoom;

        // Send scroll up event (zoom in)
        app.world_mut().send_event(MouseWheel {
            unit: MouseScrollUnit::Line,
            x: 0.0,
            y: 1.0, // Scroll up
            window: Entity::PLACEHOLDER,
        });
        app.update();

        let new_zoom = app
            .world_mut()
            .query_filtered::<&CameraController, With<MainCamera>>()
            .single(&app.world())
            .zoom;

        // Zoom in = smaller scale value
        assert!(new_zoom < initial_zoom);
    }

    #[test]
    fn test_camera_zoom_scroll_down_zooms_out() {
        let mut app = create_camera_test_app();
        app.add_systems(Update, camera_zoom);

        let initial_zoom = app
            .world_mut()
            .query_filtered::<&CameraController, With<MainCamera>>()
            .single(&app.world())
            .zoom;

        // Send scroll down event (zoom out)
        app.world_mut().send_event(MouseWheel {
            unit: MouseScrollUnit::Line,
            x: 0.0,
            y: -1.0, // Scroll down
            window: Entity::PLACEHOLDER,
        });
        app.update();

        let new_zoom = app
            .world_mut()
            .query_filtered::<&CameraController, With<MainCamera>>()
            .single(&app.world())
            .zoom;

        // Zoom out = larger scale value
        assert!(new_zoom > initial_zoom);
    }

    #[test]
    fn test_camera_zoom_respects_min_limit() {
        let mut app = create_camera_test_app();
        app.add_systems(Update, camera_zoom);

        // Get min_zoom from controller
        let min_zoom = app
            .world_mut()
            .query_filtered::<&CameraController, With<MainCamera>>()
            .single(&app.world())
            .min_zoom;

        // Send many scroll up events to hit min limit
        for _ in 0..50 {
            app.world_mut().send_event(MouseWheel {
                unit: MouseScrollUnit::Line,
                x: 0.0,
                y: 10.0,
                window: Entity::PLACEHOLDER,
            });
            app.update();
        }

        let zoom = app
            .world_mut()
            .query_filtered::<&CameraController, With<MainCamera>>()
            .single(&app.world())
            .zoom;

        assert!(zoom >= min_zoom);
    }

    #[test]
    fn test_camera_zoom_respects_max_limit() {
        let mut app = create_camera_test_app();
        app.add_systems(Update, camera_zoom);

        // Get max_zoom from controller
        let max_zoom = app
            .world_mut()
            .query_filtered::<&CameraController, With<MainCamera>>()
            .single(&app.world())
            .max_zoom;

        // Send many scroll down events to hit max limit
        for _ in 0..50 {
            app.world_mut().send_event(MouseWheel {
                unit: MouseScrollUnit::Line,
                x: 0.0,
                y: -10.0,
                window: Entity::PLACEHOLDER,
            });
            app.update();
        }

        let zoom = app
            .world_mut()
            .query_filtered::<&CameraController, With<MainCamera>>()
            .single(&app.world())
            .zoom;

        assert!(zoom <= max_zoom);
    }

    // Camera pan tests
    #[test]
    fn test_camera_pan_moves_camera() {
        let mut app = create_camera_test_app();
        app.add_systems(Update, camera_pan);

        let initial_pos = app
            .world_mut()
            .query_filtered::<&Transform, With<MainCamera>>()
            .single(&app.world())
            .translation;

        // Press D key to pan right
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyD);
        app.update();

        let new_pos = app
            .world_mut()
            .query_filtered::<&Transform, With<MainCamera>>()
            .single(&app.world())
            .translation;

        assert!(new_pos.x > initial_pos.x);
    }

    #[test]
    fn test_camera_pan_blocked_when_locked() {
        let mut app = create_camera_test_app();
        app.add_systems(Update, camera_pan);

        // Create a target entity and lock camera to it
        let target = app.world_mut().spawn(Transform::default()).id();

        // Lock camera to target
        let mut query = app
            .world_mut()
            .query_filtered::<&mut CameraController, With<MainCamera>>();
        for mut controller in query.iter_mut(app.world_mut()) {
            controller.locked_target = Some(target);
        }

        let initial_pos = app
            .world_mut()
            .query_filtered::<&Transform, With<MainCamera>>()
            .single(&app.world())
            .translation;

        // Try to pan
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyD);
        app.update();

        let new_pos = app
            .world_mut()
            .query_filtered::<&Transform, With<MainCamera>>()
            .single(&app.world())
            .translation;

        // Position should not change when locked
        assert_eq!(new_pos.x, initial_pos.x);
    }

    // Camera follow target tests
    #[test]
    fn test_camera_follows_target() {
        let mut app = create_camera_test_app();
        app.add_systems(Update, camera_follow_target);

        // Create target at specific position
        let target = app
            .world_mut()
            .spawn(Transform::from_xyz(100.0, 50.0, 0.0))
            .id();

        // Lock camera to target
        let mut query = app
            .world_mut()
            .query_filtered::<&mut CameraController, With<MainCamera>>();
        for mut controller in query.iter_mut(app.world_mut()) {
            controller.locked_target = Some(target);
        }

        app.update();

        let camera_pos = app
            .world_mut()
            .query_filtered::<&Transform, With<MainCamera>>()
            .single(&app.world())
            .translation;

        assert_eq!(camera_pos.x, 100.0);
        assert_eq!(camera_pos.y, 50.0);
    }

    #[test]
    fn test_camera_handles_invalid_target() {
        let mut app = create_camera_test_app();
        app.add_systems(Update, camera_follow_target);

        // Set an invalid entity as target
        let invalid_entity = Entity::from_raw(99999);

        let mut query = app
            .world_mut()
            .query_filtered::<&mut CameraController, With<MainCamera>>();
        for mut controller in query.iter_mut(app.world_mut()) {
            controller.locked_target = Some(invalid_entity);
        }

        // Should not panic
        app.update();

        // Camera should stay at original position
        let camera_pos = app
            .world_mut()
            .query_filtered::<&Transform, With<MainCamera>>()
            .single(&app.world())
            .translation;

        assert_eq!(camera_pos.x, 0.0);
        assert_eq!(camera_pos.y, 0.0);
    }
}
