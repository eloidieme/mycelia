//! Shared test utilities for network module tests

#[cfg(test)]
use super::NetworkPlugin;
#[cfg(test)]
use crate::{
    game::input::{CursorWorldPosition, InputActions},
    GameState,
};
#[cfg(test)]
use bevy::prelude::*;
#[cfg(test)]
use bevy::state::app::StatesPlugin;

/// Helper to create test app with network plugin and all required dependencies
#[cfg(test)]
pub fn create_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(StatesPlugin)
        .init_state::<GameState>()
        .add_plugins(NetworkPlugin)
        .init_resource::<InputActions>()
        .init_resource::<CursorWorldPosition>();
    app
}
