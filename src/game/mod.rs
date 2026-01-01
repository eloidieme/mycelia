//! Game systems module
//!
//! Contains all gameplay-related systems organized by domain.

use bevy::prelude::*;

pub mod camera;
pub mod combat;
pub mod enemies;
pub mod input;
pub mod map;
pub mod network;
pub mod progression;
pub mod ui;

/// Plugin that registers all game systems
pub struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            input::InputPlugin,
            network::NetworkPlugin,
            combat::CombatPlugin,
            enemies::EnemiesPlugin,
            progression::ProgressionPlugin,
            map::MapPlugin,
            ui::UiPlugin,
            camera::CameraPlugin,
        ));
    }
}
