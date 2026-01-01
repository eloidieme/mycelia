//! Fungal network core node handling systems.
//!
//! # Configuration
//! - `CORE_NODE_HEALTH`
//! - `CORE_NODE_SIZE`

use bevy::prelude::*;

use crate::{
    game::network::{CoreNode, CoreNodeEntity, Health, NetworkMember, NetworkVisuals},
    GameState,
};

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
    use crate::{game::network::test_utils::create_test_app, GameState};

    #[test]
    fn test_core_node_spawns_on_playing_enter() {
        let mut app = create_test_app();
        app.update();

        // Transition to Playing state
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update(); // Extra update for state transition

        // Assert CoreNode entity exists
        let core_count = app
            .world_mut()
            .query_filtered::<Entity, With<CoreNode>>()
            .iter(&app.world())
            .count();
        assert_eq!(core_count, 1);
    }

    #[test]
    fn test_core_node_entity_resource_set() {
        let mut app = create_test_app();
        app.update();

        // Transition to Playing state
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        // Assert CoreNodeEntity resource exists and points to valid entity
        let core_entity = app.world().resource::<CoreNodeEntity>();
        assert!(app.world().get_entity(core_entity.0).is_ok());
    }

    #[test]
    fn test_core_node_has_required_components() {
        let mut app = create_test_app();
        app.update();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        // Check core node has all required components
        let core_entity = app.world().resource::<CoreNodeEntity>().0;
        let world = app.world();

        assert!(world.get::<CoreNode>(core_entity).is_some());
        assert!(world.get::<Health>(core_entity).is_some());
        assert!(world.get::<NetworkMember>(core_entity).is_some());
        assert!(world.get::<Transform>(core_entity).is_some());
        assert!(world.get::<Sprite>(core_entity).is_some());
    }

    #[test]
    fn test_core_node_spawns_at_origin() {
        let mut app = create_test_app();
        app.update();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        let core_entity = app.world().resource::<CoreNodeEntity>().0;
        let transform = app.world().get::<Transform>(core_entity).unwrap();

        assert_eq!(transform.translation.x, 0.0);
        assert_eq!(transform.translation.y, 0.0);
    }

    #[test]
    fn test_core_death_triggers_game_over() {
        let mut app = create_test_app();
        app.update();

        // Transition to Playing state
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        // Get core node and set health to 0
        let core_entity = app.world().resource::<CoreNodeEntity>().0;
        app.world_mut()
            .get_mut::<Health>(core_entity)
            .unwrap()
            .current = 0.0;
        app.update();
        app.update();

        // Assert state is now GameOver
        assert_eq!(
            *app.world().resource::<State<GameState>>().get(),
            GameState::GameOver
        );
    }

    #[test]
    fn test_core_node_despawns_on_menu_return() {
        let mut app = create_test_app();
        app.update();

        // Go to Playing
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        // Return to Menu
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Menu);
        app.update();
        app.update();

        // Assert no CoreNode entities exist
        let core_count = app
            .world_mut()
            .query_filtered::<Entity, With<CoreNode>>()
            .iter(&app.world())
            .count();
        assert_eq!(core_count, 0);
    }
}
