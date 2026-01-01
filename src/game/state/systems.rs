//! State systems

use bevy::prelude::*;

use super::components::{PauseState, RunStats};
use crate::GameState;

/// Handle pause input (Escape key)
pub fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut pause_state: ResMut<PauseState>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match **current_state {
            GameState::Playing => {
                pause_state.was_paused_by_upgrade = false;
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                // Only unpause if not paused for upgrade
                if !pause_state.was_paused_by_upgrade {
                    next_state.set(GameState::Playing);
                }
            }
            _ => {}
        }
    }
}

/// Reset run stats when entering Playing state
pub fn reset_run_stats(mut run_stats: ResMut<RunStats>) {
    run_stats.reset();
}

/// Update elapsed time during gameplay
pub fn update_run_time(time: Res<Time>, mut run_stats: ResMut<RunStats>) {
    run_stats.elapsed_time += time.delta_secs();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_run_stats_system() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(RunStats {
                elapsed_time: 100.0,
                enemies_killed: 50,
                max_territory: 0.5,
                nutrients_collected: 1000.0,
            })
            .add_systems(Update, reset_run_stats);

        app.update();

        let stats = app.world().resource::<RunStats>();
        assert_eq!(stats.elapsed_time, 0.0);
        assert_eq!(stats.enemies_killed, 0);
    }
}
