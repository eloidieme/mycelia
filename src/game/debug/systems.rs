//! Debug systems
//!
//! Systems for managing the debug overlay.

use bevy::prelude::*;

use super::components::*;
use super::resources::*;
use crate::game::input::CursorWorldPosition;
use crate::game::network::{NetworkParent, NetworkStats, TendrilPosition};
use crate::game::progression::Nutrients;
use crate::GameState;

/// Toggle debug overlay with F3 key
pub fn toggle_debug_overlay(input: Res<ButtonInput<KeyCode>>, mut settings: ResMut<DebugSettings>) {
    if input.just_pressed(KeyCode::F3) {
        settings.toggle();
    }
}

/// Toggle network graph visualization with F4 key
pub fn toggle_network_graph(input: Res<ButtonInput<KeyCode>>, mut settings: ResMut<DebugSettings>) {
    if input.just_pressed(KeyCode::F4) {
        settings.toggle_network_graph();
    }
}

/// Track frame times for FPS calculation
pub fn track_frame_time(time: Res<Time>, mut tracker: ResMut<FrameTimeTracker>) {
    tracker.record(time.delta_secs());
}

/// Update debug overlay visibility based on settings
pub fn update_overlay_visibility(
    settings: Res<DebugSettings>,
    mut query: Query<&mut Visibility, With<DebugOverlay>>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = if settings.enabled {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

/// Update FPS display text
pub fn update_fps_display(
    tracker: Res<FrameTimeTracker>,
    settings: Res<DebugSettings>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    if !settings.enabled || !settings.show_fps {
        return;
    }

    let fps = tracker.average_fps();
    let min_fps = tracker.min_fps();

    for mut text in query.iter_mut() {
        **text = format!("FPS: {:.0} (min: {:.0})", fps, min_fps);
    }
}

/// Update entity count display text
pub fn update_entity_count_display(
    settings: Res<DebugSettings>,
    entities: Query<Entity>,
    mut query: Query<&mut Text, With<EntityCountText>>,
) {
    if !settings.enabled || !settings.show_entity_count {
        return;
    }

    let count = entities.iter().count();

    for mut text in query.iter_mut() {
        **text = format!("Entities: {}", count);
    }
}

/// Update network stats display text
pub fn update_network_stats_display(
    settings: Res<DebugSettings>,
    network_stats: Res<NetworkStats>,
    mut query: Query<&mut Text, With<NetworkStatsText>>,
) {
    if !settings.enabled || !settings.show_network_stats {
        return;
    }

    for mut text in query.iter_mut() {
        **text = format!(
            "Network: {} segs, {:.0} mass, {:.1}% territory",
            network_stats.segment_count,
            network_stats.total_mass,
            network_stats.territory_coverage * 100.0
        );
    }
}

/// Update nutrients display text
pub fn update_nutrients_display(
    settings: Res<DebugSettings>,
    nutrients: Res<Nutrients>,
    mut query: Query<&mut Text, With<NutrientsText>>,
) {
    if !settings.enabled || !settings.show_nutrients {
        return;
    }

    for mut text in query.iter_mut() {
        **text = format!(
            "Nutrients: {:.0}/{:.0} ({:.0}%)",
            nutrients.current,
            nutrients.max,
            nutrients.percentage() * 100.0
        );
    }
}

/// Update game state display text
pub fn update_game_state_display(
    settings: Res<DebugSettings>,
    state: Res<State<GameState>>,
    mut query: Query<&mut Text, With<GameStateText>>,
) {
    if !settings.enabled || !settings.show_game_state {
        return;
    }

    let state_name = match state.get() {
        GameState::Menu => "Menu",
        GameState::Playing => "Playing",
        GameState::Paused => "Paused",
        GameState::GameOver => "GameOver",
        GameState::Upgrading => "Upgrading",
    };

    for mut text in query.iter_mut() {
        **text = format!("State: {}", state_name);
    }
}

/// Update cursor position display text
pub fn update_cursor_position_display(
    settings: Res<DebugSettings>,
    cursor_pos: Res<CursorWorldPosition>,
    mut query: Query<&mut Text, With<CursorPositionText>>,
) {
    if !settings.enabled || !settings.show_cursor_position {
        return;
    }

    for mut text in query.iter_mut() {
        **text = match cursor_pos.position {
            Some(pos) => format!("Cursor: ({:.0}, {:.0})", pos.x, pos.y),
            None => "Cursor: --".to_string(),
        };
    }
}

/// Visualize network graph edges with debug gizmos
pub fn render_network_graph_debug(
    settings: Res<DebugSettings>,
    mut gizmos: Gizmos,
    segments: Query<(&TendrilPosition, &NetworkParent)>,
    positions: Query<&TendrilPosition>,
) {
    if !settings.enabled || !settings.show_network_graph {
        return;
    }

    for (pos, parent) in segments.iter() {
        if let Ok(parent_pos) = positions.get(parent.0) {
            // Draw debug line in yellow, semi-transparent
            gizmos.line_2d(
                parent_pos.position,
                pos.position,
                Color::srgba(1.0, 1.0, 0.0, 0.5),
            );
        }
    }
}

/// Spawn the debug overlay UI
pub fn spawn_debug_overlay(mut commands: Commands) {
    commands
        .spawn((
            DebugOverlay,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                ..default()
            },
            Visibility::Hidden, // Start hidden
        ))
        .with_children(|parent| {
            let text_font = TextFont {
                font_size: 14.0,
                ..default()
            };
            let text_color = TextColor(Color::srgb(0.0, 1.0, 0.0)); // Green text

            // FPS
            parent.spawn((FpsText, Text::new("FPS: --"), text_font.clone(), text_color));

            // Entity count
            parent.spawn((
                EntityCountText,
                Text::new("Entities: --"),
                text_font.clone(),
                text_color,
            ));

            // Network stats
            parent.spawn((
                NetworkStatsText,
                Text::new("Network: --"),
                text_font.clone(),
                text_color,
            ));

            // Nutrients
            parent.spawn((
                NutrientsText,
                Text::new("Nutrients: --"),
                text_font.clone(),
                text_color,
            ));

            // Game state
            parent.spawn((
                GameStateText,
                Text::new("State: --"),
                text_font.clone(),
                text_color,
            ));

            // Cursor position
            parent.spawn((
                CursorPositionText,
                Text::new("Cursor: --"),
                text_font,
                text_color,
            ));
        });
}

/// Despawn the debug overlay UI
#[allow(dead_code)] // Available for future use (e.g., cleanup on app exit)
pub fn despawn_debug_overlay(mut commands: Commands, query: Query<Entity, With<DebugOverlay>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_debug_overlay_system_exists() {
        // Just verify the system compiles and has correct signature
        fn assert_system<T: bevy::ecs::system::System>(_: T) {}
        assert_system(IntoSystem::into_system(toggle_debug_overlay));
    }

    #[test]
    fn test_toggle_network_graph_system_exists() {
        fn assert_system<T: bevy::ecs::system::System>(_: T) {}
        assert_system(IntoSystem::into_system(toggle_network_graph));
    }

    #[test]
    fn test_track_frame_time_system_exists() {
        fn assert_system<T: bevy::ecs::system::System>(_: T) {}
        assert_system(IntoSystem::into_system(track_frame_time));
    }

    #[test]
    fn test_update_fps_display_system_exists() {
        fn assert_system<T: bevy::ecs::system::System>(_: T) {}
        assert_system(IntoSystem::into_system(update_fps_display));
    }
}
