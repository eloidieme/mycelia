//! Mycelia - A fungal network survival roguelite
//!
//! This crate provides the core game logic for Mycelia, a Vampire Survivors-inspired
//! game where you control a spreading fungal network.

use bevy::prelude::*;

pub mod game;

/// Main game plugin that sets up all game systems
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(game::GameSystemsPlugin)
            .init_state::<GameState>();
    }
}

/// Core game states
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    /// Main menu
    #[default]
    Menu,
    /// Active gameplay
    Playing,
    /// Game is paused
    Paused,
    /// Upgrade selection screen
    Upgrading,
    /// Game over screen
    GameOver,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_default_is_menu() {
        assert_eq!(GameState::default(), GameState::Menu);
    }
}
