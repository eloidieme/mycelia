//! Enemy system
//!
//! Handles enemy behavior:
//! - Enemy spawning
//! - AI and pathfinding
//! - Enemy types (insects, fungi, bacteria)
//! - Corruption mechanics

use bevy::prelude::*;

/// Plugin for the enemy system
pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, _app: &mut App) {
        // TODO: Register enemy systems
    }
}
