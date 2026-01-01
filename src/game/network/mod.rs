//! Fungal network system
//!
//! Handles the core network mechanics:
//! - Tendril growth and pathfinding
//! - Network connectivity and severance
//! - Specialized tendril types
//! - Core node management
//! - Visual rendering of the network

use bevy::prelude::*;

use crate::GameState;

mod components;
pub mod graph;
mod rendering;
mod resources;
mod systems;

pub use components::*;
pub use rendering::{lerp_color, segment_color, TendrilAnimationState, TendrilStyle};
pub use resources::*;

/// Plugin for the fungal network system
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<NetworkStats>()
            .init_resource::<ActiveGrowthTip>()
            .init_resource::<TendrilAnimationState>()
            // Spawn core node when entering Playing state
            .add_systems(OnEnter(GameState::Playing), systems::spawn_core_node)
            // Despawn core node when entering Menu (cleanup)
            .add_systems(OnEnter(GameState::Menu), systems::despawn_core_node)
            // Update systems during gameplay
            .add_systems(
                Update,
                (
                    systems::check_core_death,
                    systems::select_growth_tip,
                    rendering::update_tendril_animation,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            // Rendering systems (run during Playing and Paused so visuals remain)
            // Only run when Gizmos are available (requires full Bevy plugins, not MinimalPlugins)
            .add_systems(
                Update,
                (
                    rendering::render_tendrils,
                    rendering::render_growth_tips,
                    rendering::render_core,
                )
                    .run_if(
                        resource_exists::<bevy::gizmos::config::GizmoConfigStore>
                            .and(in_state(GameState::Playing).or(in_state(GameState::Paused))),
                    ),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        game::input::{CursorWorldPosition, InputActions},
        GameState,
    };
    use bevy::state::app::StatesPlugin;

    /// Helper to create test app with network plugin
    fn create_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(StatesPlugin)
            .init_state::<GameState>()
            .add_plugins(NetworkPlugin)
            .init_resource::<InputActions>()
            .init_resource::<CursorWorldPosition>();
        app
    }

    #[test]
    fn test_network_plugin_builds() {
        let mut app = create_test_app();
        app.update();
    }

    #[test]
    fn test_core_node_spawns_on_playing_enter() {
        let mut app = create_test_app();
        app.update();

        // Transition to Playing state
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update(); // Extra update for state transition

        // Assert CoreNode entity exists
        let core_count = app
            .world_mut()
            .query_filtered::<Entity, With<CoreNode>>()
            .iter(&app.world())
            .count();
        assert_eq!(core_count, 1);
    }

    #[test]
    fn test_core_node_entity_resource_set() {
        let mut app = create_test_app();
        app.update();

        // Transition to Playing state
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        // Assert CoreNodeEntity resource exists and points to valid entity
        let core_entity = app.world().resource::<CoreNodeEntity>();
        assert!(app.world().get_entity(core_entity.0).is_ok());
    }

    #[test]
    fn test_core_node_has_required_components() {
        let mut app = create_test_app();
        app.update();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        // Check core node has all required components
        let core_entity = app.world().resource::<CoreNodeEntity>().0;
        let world = app.world();

        assert!(world.get::<CoreNode>(core_entity).is_some());
        assert!(world.get::<Health>(core_entity).is_some());
        assert!(world.get::<NetworkMember>(core_entity).is_some());
        assert!(world.get::<Transform>(core_entity).is_some());
        assert!(world.get::<Sprite>(core_entity).is_some());
    }

    #[test]
    fn test_core_node_spawns_at_origin() {
        let mut app = create_test_app();
        app.update();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        let core_entity = app.world().resource::<CoreNodeEntity>().0;
        let transform = app.world().get::<Transform>(core_entity).unwrap();

        assert_eq!(transform.translation.x, 0.0);
        assert_eq!(transform.translation.y, 0.0);
    }

    #[test]
    fn test_core_death_triggers_game_over() {
        let mut app = create_test_app();
        app.update();

        // Transition to Playing state
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        // Get core node and set health to 0
        let core_entity = app.world().resource::<CoreNodeEntity>().0;
        app.world_mut()
            .get_mut::<Health>(core_entity)
            .unwrap()
            .current = 0.0;
        app.update();
        app.update();

        // Assert state is now GameOver
        assert_eq!(
            *app.world().resource::<State<GameState>>().get(),
            GameState::GameOver
        );
    }

    #[test]
    fn test_core_node_despawns_on_menu_return() {
        let mut app = create_test_app();
        app.update();

        // Go to Playing
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        // Return to Menu
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Menu);
        app.update();
        app.update();

        // Assert no CoreNode entities exist
        let core_count = app
            .world_mut()
            .query_filtered::<Entity, With<CoreNode>>()
            .iter(&app.world())
            .count();
        assert_eq!(core_count, 0);
    }

    #[test]
    /// Spawn tip, select it: tip selected.
    fn test_select_tip_with_click() {
        // Create app and init input + cursor
        let mut app = create_test_app();
        app.update();

        // Put the app in playing state
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        // Spawn tendril/tip entity
        let tip_position = Vec2::new(10.0, 20.0);
        let tip_entity = app
            .world_mut()
            .spawn((
                GrowthTip::default(),
                TendrilPosition::new(tip_position, Vec2::X),
            ))
            .id();

        // Set cursor and press primary button
        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(Vec2::new(10.0, 20.0));

        app.world_mut()
            .resource_mut::<InputActions>()
            .primary_just_pressed = true;

        app.update();

        // Get the tip entity and the active growth tip resource and check state
        let tip = app.world().get::<GrowthTip>(tip_entity).unwrap();
        assert!(tip.selected);

        let active = app.world().resource::<ActiveGrowthTip>();
        assert_eq!(active.0, Some(tip_entity));
    }

    #[test]
    /// Spawn selected tip, click far away: no tip selected.
    fn test_clicking_empty_space_deselects() {
        let mut app = create_test_app();
        app.update();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        let tip_position = Vec2::new(10.0, 20.0);
        let tip_entity = app
            .world_mut()
            .spawn((
                GrowthTip { selected: true },
                TendrilPosition::new(tip_position, Vec2::X),
            ))
            .id();

        app.world_mut().resource_mut::<ActiveGrowthTip>().0 = Some(tip_entity);

        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(Vec2::new(100.0, 100.0));

        app.world_mut()
            .resource_mut::<InputActions>()
            .primary_just_pressed = true;

        app.update();

        let tip = app.world().get::<GrowthTip>(tip_entity).unwrap();
        assert!(!tip.selected);

        let active = app.world().resource::<ActiveGrowthTip>();
        assert!(active.0.is_none());
    }

    #[test]
    /// Spawn selected tip, click it again: same tip selected.
    fn test_clicking_already_selected_tip_keeps_selection() {
        let mut app = create_test_app();
        app.update();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        let tip_position = Vec2::new(10.0, 20.0);
        let tip_entity = app
            .world_mut()
            .spawn((
                GrowthTip { selected: true },
                TendrilPosition::new(tip_position, Vec2::X),
            ))
            .id();

        app.world_mut().resource_mut::<ActiveGrowthTip>().0 = Some(tip_entity);

        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(Vec2::new(10.0, 20.0));

        app.world_mut()
            .resource_mut::<InputActions>()
            .primary_just_pressed = true;

        app.update();

        let tip = app.world().get::<GrowthTip>(tip_entity).unwrap();
        assert!(tip.selected);

        let active = app.world().resource::<ActiveGrowthTip>();
        assert_eq!(active.0, Some(tip_entity));
    }

    #[test]
    /// Spawn two tips, one selected, click the other: other is selected.
    fn test_selecting_new_tip_deselects_old() {
        let mut app = create_test_app();
        app.update();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        let tip_1_position = Vec2::new(10.0, 20.0);
        let tip_1_entity = app
            .world_mut()
            .spawn((
                GrowthTip { selected: true },
                TendrilPosition::new(tip_1_position, Vec2::X),
            ))
            .id();

        app.world_mut().resource_mut::<ActiveGrowthTip>().0 = Some(tip_1_entity);

        let tip_2_position = Vec2::new(20.0, 40.0);
        let tip_2_entity = app
            .world_mut()
            .spawn((
                GrowthTip::default(),
                TendrilPosition::new(tip_2_position, Vec2::X),
            ))
            .id();

        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(Vec2::new(20.0, 40.0));

        app.world_mut()
            .resource_mut::<InputActions>()
            .primary_just_pressed = true;

        app.update();

        let tip_1 = app.world().get::<GrowthTip>(tip_1_entity).unwrap();
        assert!(!tip_1.selected);

        let tip_2 = app.world().get::<GrowthTip>(tip_2_entity).unwrap();
        assert!(tip_2.selected);

        let active = app.world().resource::<ActiveGrowthTip>();
        assert_eq!(active.0, Some(tip_2_entity));
    }

    #[test]
    /// Spawn two tips within selection radius, click between them: closest is selected
    fn test_selects_closest_tip_when_multiple_in_radius() {
        let mut app = create_test_app();
        app.update();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        let cursor_pos = Vec2::new(50.0, 50.0);

        let tip_1_position = Vec2::new(50.0, 58.0);
        let tip_1_entity = app
            .world_mut()
            .spawn((
                GrowthTip::default(),
                TendrilPosition::new(tip_1_position, Vec2::X),
            ))
            .id();

        let tip_2_position = Vec2::new(50.0, 61.0);
        let _tip_2_entity = app
            .world_mut()
            .spawn((
                GrowthTip::default(),
                TendrilPosition::new(tip_2_position, Vec2::X),
            ))
            .id();

        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(cursor_pos);

        app.world_mut()
            .resource_mut::<InputActions>()
            .primary_just_pressed = true;

        app.update();

        let tip_1 = app.world().get::<GrowthTip>(tip_1_entity).unwrap();
        assert!(tip_1.selected);

        let active = app.world().resource::<ActiveGrowthTip>();
        assert_eq!(active.0, Some(tip_1_entity));
    }

    #[test]
    /// Spawn tip, set cursor: no tip selected.
    fn test_no_selection_without_click() {
        // Create app and init input + cursor
        let mut app = create_test_app();
        app.update();

        // Put the app in playing state
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        // Spawn tendril/tip entity
        let tip_position = Vec2::new(10.0, 20.0);
        let tip_entity = app
            .world_mut()
            .spawn((
                GrowthTip::default(),
                TendrilPosition::new(tip_position, Vec2::X),
            ))
            .id();

        // Set cursor
        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(Vec2::new(10.0, 20.0));

        app.update();

        // Get the tip entity and the active growth tip resource and check state
        let tip = app.world().get::<GrowthTip>(tip_entity).unwrap();
        assert!(!tip.selected);

        let active = app.world().resource::<ActiveGrowthTip>();
        assert!(active.0.is_none());
    }

    #[test]
    /// Spawn tip, set cursor outside window: no tip selected, no crash.
    fn test_no_crash_when_cursor_outside_window() {
        let mut app = create_test_app();
        app.update();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        let tip_position = Vec2::new(10.0, 20.0);
        let tip_entity = app
            .world_mut()
            .spawn((
                GrowthTip::default(),
                TendrilPosition::new(tip_position, Vec2::X),
            ))
            .id();

        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = None;

        app.world_mut()
            .resource_mut::<InputActions>()
            .primary_just_pressed = true;

        app.update();

        let tip = app.world().get::<GrowthTip>(tip_entity).unwrap();
        assert!(!tip.selected);

        let active = app.world().resource::<ActiveGrowthTip>();
        assert!(active.0.is_none());
    }

    #[test]
    /// Spawn no tip, set cursor and press: no tip selected, no crash.
    fn test_clicking_with_no_tips_does_nothing() {
        let mut app = create_test_app();
        app.update();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(Vec2::new(20.0, 10.0));

        app.world_mut()
            .resource_mut::<InputActions>()
            .primary_just_pressed = true;

        app.update();

        let active = app.world().resource::<ActiveGrowthTip>();
        assert!(active.0.is_none());
    }
}
