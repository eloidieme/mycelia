//! Progression system
//!
//! Handles upgrades and meta-progression:
//! - Territory milestone tracking
//! - Upgrade selection
//! - Meta-progression unlocks
//! - Difficulty scaling

use bevy::prelude::*;

/// Plugin for the progression system
pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, _app: &mut App) {
        // TODO: Register progression systems
    }
}

/// Resource tracking nutrients
#[derive(Resource, Debug, Default)]
pub struct Nutrients {
    pub current: f32,
    pub max: f32,
}
