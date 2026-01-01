//! Input handling system
//!
//! Provides centralized input handling:
//! - Mouse position in world coordinates
//! - Device-agnostic input actions
//! - Keyboard and mouse support (gamepad planned)

use bevy::prelude::*;

pub mod resources;
pub mod systems;

pub use resources::*;

/// Plugin for the input handling system
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorWorldPosition>()
            .init_resource::<InputActions>()
            // Clear actions first, then read new input
            .add_systems(PreUpdate, systems::clear_input_actions)
            .add_systems(
                PreUpdate,
                (
                    systems::update_keyboard_input,
                    systems::update_mouse_input,
                    systems::update_scroll_input,
                    systems::update_cursor_world_position,
                )
                    .chain() // Ensure keyboard runs before mouse for correct OR semantics
                    .after(systems::clear_input_actions),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::InputPlugin as BevyInputPlugin;

    /// Helper to create test app with input plugin
    fn create_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(BevyInputPlugin)
            .add_plugins(InputPlugin);
        app
    }

    #[test]
    fn test_input_plugin_builds() {
        let mut app = create_test_app();
        app.update();
    }

    #[test]
    fn test_cursor_world_position_resource_exists() {
        let mut app = create_test_app();
        app.update();

        assert!(app.world().get_resource::<CursorWorldPosition>().is_some());
    }

    #[test]
    fn test_input_actions_resource_exists() {
        let mut app = create_test_app();
        app.update();

        assert!(app.world().get_resource::<InputActions>().is_some());
    }

    #[test]
    fn test_keyboard_updates_move_direction() {
        let mut app = create_test_app();
        app.update();

        // Press W key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        app.update();

        let actions = app.world().resource::<InputActions>();
        assert!(actions.move_direction.y > 0.0);
    }

    #[test]
    fn test_wasd_all_directions() {
        let mut app = create_test_app();
        app.update();

        // Test W (up)
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        app.update();
        assert!(app.world().resource::<InputActions>().move_direction.y > 0.0);

        // Clear and test S (down)
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .release(KeyCode::KeyW);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyS);
        app.update();
        assert!(app.world().resource::<InputActions>().move_direction.y < 0.0);

        // Clear and test A (left)
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .release(KeyCode::KeyS);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyA);
        app.update();
        assert!(app.world().resource::<InputActions>().move_direction.x < 0.0);

        // Clear and test D (right)
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .release(KeyCode::KeyA);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyD);
        app.update();
        assert!(app.world().resource::<InputActions>().move_direction.x > 0.0);
    }

    #[test]
    fn test_escape_sets_pause_pressed() {
        let mut app = create_test_app();
        app.update();

        // Send keyboard event for Escape press (events work with just_pressed)
        app.world_mut()
            .send_event(bevy::input::keyboard::KeyboardInput {
                key_code: KeyCode::Escape,
                logical_key: bevy::input::keyboard::Key::Escape,
                state: bevy::input::ButtonState::Pressed,
                repeat: false,
                window: Entity::PLACEHOLDER,
            });
        app.update();

        let actions = app.world().resource::<InputActions>();
        assert!(actions.pause_just_pressed);
    }

    #[test]
    fn test_mouse_scroll_updates_zoom_delta() {
        let mut app = create_test_app();
        app.update();

        // Send scroll event
        app.world_mut().send_event(bevy::input::mouse::MouseWheel {
            unit: bevy::input::mouse::MouseScrollUnit::Line,
            x: 0.0,
            y: 1.0,
            window: Entity::PLACEHOLDER,
        });
        app.update();

        let actions = app.world().resource::<InputActions>();
        assert!(actions.zoom_delta != 0.0);
    }

    #[test]
    fn test_mouse_left_click_sets_primary() {
        let mut app = create_test_app();
        app.update();

        // Press left mouse button
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Left);
        app.update();

        let actions = app.world().resource::<InputActions>();
        assert!(actions.primary_held);
    }

    #[test]
    fn test_mouse_right_click_sets_secondary() {
        let mut app = create_test_app();
        app.update();

        // Press right mouse button
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Right);
        app.update();

        let actions = app.world().resource::<InputActions>();
        assert!(actions.secondary_held);
    }
}
