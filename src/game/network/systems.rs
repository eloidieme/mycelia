//! Network systems

use bevy::prelude::*;

use super::components::{CoreNode, Health, NetworkMember, NetworkVisuals};
use super::resources::CoreNodeEntity;
use crate::GameState;

/// Core node configuration
const CORE_NODE_HEALTH: f32 = 100.0;
const CORE_NODE_SIZE: f32 = 32.0;

/// Spawn the core node when entering Playing state
pub fn spawn_core_node(mut commands: Commands) {
    let visuals = NetworkVisuals::default();

    let entity = commands
        .spawn((
            CoreNode,
            NetworkMember,
            Health::new(CORE_NODE_HEALTH),
            visuals,
            Sprite {
                color: Color::srgb(0.4, 0.8, 0.4),
                custom_size: Some(Vec2::splat(CORE_NODE_SIZE)),
                ..default()
            },
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
    for health in core_query.iter() {
        if health.is_dead() {
            next_state.set(GameState::GameOver);
        }
    }
}
