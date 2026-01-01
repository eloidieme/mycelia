//! Network systems

use bevy::prelude::*;

use super::components::{CoreNode, Health, NetworkMember, NetworkVisuals};
use super::resources::CoreNodeEntity;
use crate::GameState;

/// Core node configuration
const CORE_NODE_HEALTH: f32 = 100.0;
const CORE_NODE_SIZE: f32 = 32.0;

/// Selection radius for growth tips (in world units)
const TIP_SELECTION_RADIUS: f32 = 12.0;

// Helpers
pub fn is_cursor_near_tip(cursor_pos: Vec2, tip_pos: Vec2, radius: f32) -> bool {
    tip_pos.distance_squared(cursor_pos) <= radius * radius
}

/// Spawn the core node when entering Playing state
pub fn spawn_core_node(mut commands: Commands) {
    let visuals = NetworkVisuals::default();

    let entity = commands
        .spawn((
            CoreNode,
            NetworkMember,
            Health::new(CORE_NODE_HEALTH),
            Sprite {
                color: visuals.base_color,
                custom_size: Some(Vec2::splat(CORE_NODE_SIZE)),
                ..default()
            },
            visuals,
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();

    commands.insert_resource(CoreNodeEntity(entity));
}

/// Despawn the core node when leaving Playing/GameOver states
pub fn despawn_core_node(mut commands: Commands, core_query: Query<Entity, With<CoreNode>>) {
    for entity in core_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<CoreNodeEntity>();
}

/// Check if core node is dead and trigger game over
pub fn check_core_death(
    core_query: Query<&Health, With<CoreNode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(health) = core_query.get_single() else {
        return;
    };

    if health.is_dead() {
        next_state.set(GameState::GameOver);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_directly_on_tip_is_near() {
        assert!(is_cursor_near_tip(
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 0.0),
            1.0
        ));
    }

    #[test]
    fn test_cursor_within_radius_is_near() {
        // cursor 5 units away, radius 10
        assert!(is_cursor_near_tip(
            Vec2::new(0.0, 5.0),
            Vec2::new(0.0, 0.0),
            10.0
        ));
    }

    #[test]
    fn test_cursor_outside_radius_is_not_near() {
        // cursor 15 units away, radius 10
        assert!(!is_cursor_near_tip(
            Vec2::new(0.0, 15.0),
            Vec2::new(0.0, 0.0),
            10.0
        ));
    }

    #[test]
    fn test_cursor_at_exact_radius_boundary() {
        // cursor exactly radius units away
        assert!(is_cursor_near_tip(
            Vec2::new(0.0, 10.0),
            Vec2::new(0.0, 0.0),
            10.0
        ));
    }
}
