//! Game state management
//!
//! Handles state transitions, pause functionality, and run statistics.

use bevy::prelude::*;

use crate::GameState;

pub mod components;
pub mod systems;

pub use components::*;

/// Plugin for game state management
pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RunStats>()
            .init_resource::<PauseState>()
            // Pause input works in Playing and Paused states
            .add_systems(
                Update,
                systems::handle_pause_input
                    .run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))),
            )
            // Update run time only in Playing state
            .add_systems(
                Update,
                systems::update_run_time.run_if(in_state(GameState::Playing)),
            )
            // Reset stats when entering Playing (new run)
            .add_systems(OnEnter(GameState::Playing), systems::reset_run_stats);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GameState;
    use bevy::input::InputPlugin;
    use bevy::state::app::StatesPlugin;

    /// Helper to create test app with state plugin
    fn create_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(InputPlugin)
            .add_plugins(StatesPlugin)
            .init_state::<GameState>()
            .add_plugins(StatePlugin);
        app
    }

    #[test]
    fn test_state_plugin_builds() {
        let mut app = create_test_app();
        app.update();
    }

    #[test]
    fn test_run_stats_resource_exists() {
        let mut app = create_test_app();
        app.update();

        assert!(app.world().get_resource::<RunStats>().is_some());
    }

    #[test]
    fn test_game_starts_in_menu() {
        let mut app = create_test_app();
        app.update();

        let state = app.world().resource::<State<GameState>>();
        assert_eq!(**state, GameState::Menu);
    }

    #[test]
    fn test_state_can_transition_to_playing() {
        let mut app = create_test_app();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        let state = app.world().resource::<State<GameState>>();
        assert_eq!(**state, GameState::Playing);
    }

    #[test]
    fn test_state_can_transition_to_paused() {
        let mut app = create_test_app();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Paused);
        app.update();
        app.update();

        let state = app.world().resource::<State<GameState>>();
        assert_eq!(**state, GameState::Paused);
    }

    #[test]
    fn test_pause_state_tracks_upgrade_pause() {
        let mut app = create_test_app();
        app.update();

        // Verify we can track if pause was from upgrade
        {
            let mut pause_state = app.world_mut().resource_mut::<PauseState>();
            pause_state.was_paused_by_upgrade = true;
        }

        let pause_state = app.world().resource::<PauseState>();
        assert!(pause_state.was_paused_by_upgrade);
    }

    #[test]
    fn test_run_stats_reset_on_playing_enter() {
        let mut app = create_test_app();

        // Modify run stats
        {
            let mut stats = app.world_mut().resource_mut::<RunStats>();
            stats.enemies_killed = 100;
            stats.elapsed_time = 500.0;
        }

        // Transition to Playing (should reset)
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();

        let stats = app.world().resource::<RunStats>();
        assert_eq!(stats.enemies_killed, 0);
        // elapsed_time may have small delta from update, check it's close to 0
        assert!(stats.elapsed_time < 0.1);
    }
}
