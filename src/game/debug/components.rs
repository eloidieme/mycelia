//! Debug UI components
//!
//! Marker components for debug overlay UI elements.

use bevy::prelude::*;

/// Marker for the debug UI root container
#[derive(Component, Debug, Default)]
pub struct DebugOverlay;

/// Marker for FPS text element
#[derive(Component, Debug, Default)]
pub struct FpsText;

/// Marker for entity count text element
#[derive(Component, Debug, Default)]
pub struct EntityCountText;

/// Marker for network stats text element
#[derive(Component, Debug, Default)]
pub struct NetworkStatsText;

/// Marker for nutrients text element
#[derive(Component, Debug, Default)]
pub struct NutrientsText;

/// Marker for game state text element
#[derive(Component, Debug, Default)]
pub struct GameStateText;

/// Marker for cursor position text element
#[derive(Component, Debug, Default)]
pub struct CursorPositionText;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_overlay_is_component() {
        fn assert_component<T: Component>() {}
        assert_component::<DebugOverlay>();
    }

    #[test]
    fn test_fps_text_is_component() {
        fn assert_component<T: Component>() {}
        assert_component::<FpsText>();
    }

    #[test]
    fn test_entity_count_text_is_component() {
        fn assert_component<T: Component>() {}
        assert_component::<EntityCountText>();
    }

    #[test]
    fn test_network_stats_text_is_component() {
        fn assert_component<T: Component>() {}
        assert_component::<NetworkStatsText>();
    }

    #[test]
    fn test_nutrients_text_is_component() {
        fn assert_component<T: Component>() {}
        assert_component::<NutrientsText>();
    }

    #[test]
    fn test_game_state_text_is_component() {
        fn assert_component<T: Component>() {}
        assert_component::<GameStateText>();
    }

    #[test]
    fn test_cursor_position_text_is_component() {
        fn assert_component<T: Component>() {}
        assert_component::<CursorPositionText>();
    }
}
