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
mod core_node;
pub mod graph;
mod growth;
mod rendering;
mod resources;

// Re-exports
pub use components::*;
pub use rendering::{lerp_color, segment_color, TendrilAnimationState, TendrilStyle};
pub use resources::*;

/// Plugin for the fungal network system
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkStats>()
            .init_resource::<ActiveGrowthTip>()
            .init_resource::<TendrilAnimationState>()
            .add_systems(OnEnter(GameState::Playing), core_node::spawn_core_node)
            .add_systems(OnEnter(GameState::Menu), core_node::despawn_core_node)
            .add_systems(
                Update,
                (
                    core_node::check_core_death,
                    (
                        growth::select_growth_tip,
                        growth::update_selected_tip_direction,
                    )
                        .chain(),
                    rendering::update_tendril_animation,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            // Rendering systems (run during Playing and Paused so visuals remain)
            // Only run when Gizmos are available
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
mod test_utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_plugin_builds() {
        let mut app = test_utils::create_test_app();
        app.update();
    }
}
