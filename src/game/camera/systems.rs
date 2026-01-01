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

    #[test]
    fn test_spawn_camera_system() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<CameraSettings>()
            .add_systems(Startup, spawn_camera);

        app.update();

        // Verify camera was spawned
        let mut query = app.world_mut().query_filtered::<Entity, With<MainCamera>>();
        assert_eq!(query.iter(&app.world()).count(), 1);
    }
}
