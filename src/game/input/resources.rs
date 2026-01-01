//! Input resources

use bevy::prelude::*;

/// Tracks cursor position in world coordinates
#[derive(Resource, Debug, Default)]
pub struct CursorWorldPosition {
    /// Position in world space, None if cursor outside window
    pub position: Option<Vec2>,
}

/// Tracks current input actions (abstracted from device)
#[derive(Resource, Debug, Default)]
pub struct InputActions {
    /// Direction for growth/camera movement (normalized or zero)
    pub move_direction: Vec2,
    /// Primary action just pressed this frame (select tip, confirm)
    pub primary_just_pressed: bool,
    /// Primary action currently held
    pub primary_held: bool,
    /// Secondary action just pressed this frame (cancel, alternative)
    pub secondary_just_pressed: bool,
    /// Secondary action currently held
    pub secondary_held: bool,
    /// Pause toggle just pressed this frame
    pub pause_just_pressed: bool,
    /// Camera zoom delta (-1.0 to 1.0, negative = zoom out)
    pub zoom_delta: f32,
}

impl InputActions {
    /// Check if there's any movement input
    pub fn has_movement(&self) -> bool {
        self.move_direction != Vec2::ZERO
    }

    /// Clear all input state (called at frame start)
    pub fn clear(&mut self) {
        self.move_direction = Vec2::ZERO;
        self.primary_just_pressed = false;
        self.secondary_just_pressed = false;
        self.pause_just_pressed = false;
        self.zoom_delta = 0.0;
        // Note: held states are not cleared, they persist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_world_position_default_is_none() {
        let cursor = CursorWorldPosition::default();
        assert!(cursor.position.is_none());
    }

    #[test]
    fn test_cursor_world_position_is_resource() {
        fn assert_resource<T: Resource>() {}
        assert_resource::<CursorWorldPosition>();
    }

    #[test]
    fn test_input_actions_default() {
        let actions = InputActions::default();
        assert_eq!(actions.move_direction, Vec2::ZERO);
        assert!(!actions.primary_just_pressed);
        assert!(!actions.secondary_just_pressed);
        assert!(!actions.pause_just_pressed);
        assert_eq!(actions.zoom_delta, 0.0);
    }

    #[test]
    fn test_input_actions_is_resource() {
        fn assert_resource<T: Resource>() {}
        assert_resource::<InputActions>();
    }

    #[test]
    fn test_input_actions_has_movement() {
        let mut actions = InputActions::default();
        assert!(!actions.has_movement());

        actions.move_direction = Vec2::new(1.0, 0.0);
        assert!(actions.has_movement());
    }

    #[test]
    fn test_input_actions_clear() {
        let mut actions = InputActions {
            move_direction: Vec2::new(1.0, 1.0),
            primary_just_pressed: true,
            secondary_just_pressed: true,
            pause_just_pressed: true,
            zoom_delta: 0.5,
            ..default()
        };

        actions.clear();

        assert_eq!(actions.move_direction, Vec2::ZERO);
        assert!(!actions.primary_just_pressed);
        assert_eq!(actions.zoom_delta, 0.0);
    }
}
