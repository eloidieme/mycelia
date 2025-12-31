//! Fungal network system
//!
//! Handles the core network mechanics:
//! - Tendril growth and pathfinding
//! - Network connectivity and severance
//! - Specialized tendril types
//! - Core node management

use bevy::prelude::*;

mod components;
mod systems;

pub use components::*;

/// Plugin for the fungal network system
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, _app: &mut App) {
        // TODO: Register network systems
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // Network tests will go here
    }
}
