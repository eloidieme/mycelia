//! Input systems

use bevy::{input::mouse::MouseWheel, prelude::*};

use super::resources::{CursorWorldPosition, InputActions};

/// Clear input actions at the start of each frame
pub fn clear_input_actions(mut actions: ResMut<InputActions>) {
    actions.clear();
}

/// Read keyboard input and update InputActions
pub fn update_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut actions: ResMut<InputActions>,
) {
    // Movement direction
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

    if direction != Vec2::ZERO {
        actions.move_direction = direction.normalize();
    }

    // Pause
    if keyboard.just_pressed(KeyCode::Escape) {
        actions.pause_just_pressed = true;
    }

    // Primary action (Space)
    if keyboard.just_pressed(KeyCode::Space) {
        actions.primary_just_pressed = true;
    }
    actions.primary_held = keyboard.pressed(KeyCode::Space);

    // Secondary action (Shift)
    if keyboard.just_pressed(KeyCode::ShiftLeft) || keyboard.just_pressed(KeyCode::ShiftRight) {
        actions.secondary_just_pressed = true;
    }
    actions.secondary_held =
        keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
}

/// Read mouse input and update InputActions
pub fn update_mouse_input(mouse: Res<ButtonInput<MouseButton>>, mut actions: ResMut<InputActions>) {
    // Primary action (Left click)
    if mouse.just_pressed(MouseButton::Left) {
        actions.primary_just_pressed = true;
    }
    if mouse.pressed(MouseButton::Left) {
        actions.primary_held = true;
    }

    // Secondary action (Right click)
    if mouse.just_pressed(MouseButton::Right) {
        actions.secondary_just_pressed = true;
    }
    if mouse.pressed(MouseButton::Right) {
        actions.secondary_held = true;
    }
}

/// Read mouse scroll and update zoom delta
pub fn update_scroll_input(
    mut scroll_events: EventReader<MouseWheel>,
    mut actions: ResMut<InputActions>,
) {
    for event in scroll_events.read() {
        actions.zoom_delta += event.y;
    }

    // Clamp zoom delta
    actions.zoom_delta = actions.zoom_delta.clamp(-1.0, 1.0);
}

/// Update cursor world position from window cursor position
/// Note: Requires camera to be set up for proper screen-to-world conversion
pub fn update_cursor_world_position(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut cursor_pos: ResMut<CursorWorldPosition>,
) {
    let Ok(window) = windows.get_single() else {
        cursor_pos.position = None;
        return;
    };

    let Some(cursor_screen_pos) = window.cursor_position() else {
        cursor_pos.position = None;
        return;
    };

    // Try to find a camera to convert screen to world coords
    if let Ok((camera, camera_transform)) = camera_query.get_single() {
        cursor_pos.position = camera
            .viewport_to_world_2d(camera_transform, cursor_screen_pos)
            .ok();
    } else {
        // No camera, just store screen position as-is
        cursor_pos.position = Some(cursor_screen_pos);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_input_actions() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(InputActions {
                move_direction: Vec2::new(1.0, 1.0),
                primary_just_pressed: true,
                pause_just_pressed: true,
                zoom_delta: 0.5,
                ..default()
            });
        app.add_systems(Update, clear_input_actions);

        app.update();

        let actions = app.world().resource::<InputActions>();
        assert_eq!(actions.move_direction, Vec2::ZERO);
        assert!(!actions.primary_just_pressed);
    }
}
