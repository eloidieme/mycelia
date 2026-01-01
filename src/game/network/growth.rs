//! Growth tip selection and control systems.
//!
//! # Configuration
//! - `TIP_SELECTION_RADIUS`: click detection radius

use bevy::prelude::*;

use crate::game::{
    input::{CursorWorldPosition, InputActions},
    network::{ActiveGrowthTip, GrowthTip, TendrilPosition},
};

const TIP_SELECTION_RADIUS: f32 = 12.0;

pub fn is_cursor_near_tip(cursor_pos: Vec2, tip_pos: Vec2, radius: f32) -> bool {
    tip_pos.distance_squared(cursor_pos) <= radius * radius
}

/// Select a growth tip if the primary input was just pressed near one
/// and update the ActiveGrowthTip resource accordingly.
pub fn select_growth_tip(
    input: Res<InputActions>,
    cursor_position: Res<CursorWorldPosition>,
    mut active_tip: ResMut<ActiveGrowthTip>,
    mut tips_query: Query<(Entity, &TendrilPosition, &mut GrowthTip)>,
) {
    if !input.primary_just_pressed {
        return;
    }

    let Some(cursor) = cursor_position.position else {
        return;
    };

    // Deselect old tip first (if it exists and hasn't been despawned)
    if let Some(old_entity) = active_tip.0 {
        if let Ok((_entity, _pos, mut old_tip)) = tips_query.get_mut(old_entity) {
            old_tip.selected = false;
        }
    }

    let closest_tip = tips_query
        .iter_mut()
        .filter(|(_entity, pos, _tip)| {
            is_cursor_near_tip(cursor, pos.position, TIP_SELECTION_RADIUS)
        })
        .min_by(|(_, pos_a, _), (_, pos_b, _)| {
            let dist_a = cursor.distance_squared(pos_a.position);
            let dist_b = cursor.distance_squared(pos_b.position);
            dist_a.total_cmp(&dist_b)
        });

    match closest_tip {
        Some((entity, _pos, mut tip)) => {
            tip.selected = true;
            active_tip.0 = Some(entity);
        }
        None => active_tip.0 = None,
    }
}

pub fn update_selected_tip_direction(
    cursor_position: Res<CursorWorldPosition>,
    active_tip: Res<ActiveGrowthTip>,
    mut tips_query: Query<(Entity, &mut TendrilPosition, &mut GrowthTip)>,
) {
    let Some(cursor) = cursor_position.position else {
        return;
    };
    let Some(active_tip) = active_tip.0 else {
        return;
    };
    let Ok((_entity, mut tip_pos, _tip)) = tips_query.get_mut(active_tip) else {
        return;
    };

    tip_pos.direction = (cursor - tip_pos.position).normalize_or_zero();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        game::{
            input::{CursorWorldPosition, InputActions},
            network::test_utils::create_test_app,
        },
        GameState,
    };

    #[test]
    /// Spawn tip, select it: tip selected.
    fn test_select_tip_with_click() {
        // Create app and init input + cursor
        let mut app = create_test_app();
        app.update();

        // Put the app in playing state
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        // Spawn tendril/tip entity
        let tip_position = Vec2::new(10.0, 20.0);
        let tip_entity = app
            .world_mut()
            .spawn((
                GrowthTip::default(),
                TendrilPosition::new(tip_position, Vec2::X),
            ))
            .id();

        // Set cursor and press primary button
        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(Vec2::new(10.0, 20.0));

        app.world_mut()
            .resource_mut::<InputActions>()
            .primary_just_pressed = true;

        app.update();

        // Get the tip entity and the active growth tip resource and check state
        let tip = app.world().get::<GrowthTip>(tip_entity).unwrap();
        assert!(tip.selected);

        let active = app.world().resource::<ActiveGrowthTip>();
        assert_eq!(active.0, Some(tip_entity));
    }

    #[test]
    /// Spawn selected tip, click far away: no tip selected.
    fn test_clicking_empty_space_deselects() {
        let mut app = create_test_app();
        app.update();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        let tip_position = Vec2::new(10.0, 20.0);
        let tip_entity = app
            .world_mut()
            .spawn((
                GrowthTip { selected: true },
                TendrilPosition::new(tip_position, Vec2::X),
            ))
            .id();

        app.world_mut().resource_mut::<ActiveGrowthTip>().0 = Some(tip_entity);

        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(Vec2::new(100.0, 100.0));

        app.world_mut()
            .resource_mut::<InputActions>()
            .primary_just_pressed = true;

        app.update();

        let tip = app.world().get::<GrowthTip>(tip_entity).unwrap();
        assert!(!tip.selected);

        let active = app.world().resource::<ActiveGrowthTip>();
        assert!(active.0.is_none());
    }

    #[test]
    /// Spawn selected tip, click it again: same tip selected.
    fn test_clicking_already_selected_tip_keeps_selection() {
        let mut app = create_test_app();
        app.update();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        let tip_position = Vec2::new(10.0, 20.0);
        let tip_entity = app
            .world_mut()
            .spawn((
                GrowthTip { selected: true },
                TendrilPosition::new(tip_position, Vec2::X),
            ))
            .id();

        app.world_mut().resource_mut::<ActiveGrowthTip>().0 = Some(tip_entity);

        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(Vec2::new(10.0, 20.0));

        app.world_mut()
            .resource_mut::<InputActions>()
            .primary_just_pressed = true;

        app.update();

        let tip = app.world().get::<GrowthTip>(tip_entity).unwrap();
        assert!(tip.selected);

        let active = app.world().resource::<ActiveGrowthTip>();
        assert_eq!(active.0, Some(tip_entity));
    }

    #[test]
    /// Spawn two tips, one selected, click the other: other is selected.
    fn test_selecting_new_tip_deselects_old() {
        let mut app = create_test_app();
        app.update();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        let tip_1_position = Vec2::new(10.0, 20.0);
        let tip_1_entity = app
            .world_mut()
            .spawn((
                GrowthTip { selected: true },
                TendrilPosition::new(tip_1_position, Vec2::X),
            ))
            .id();

        app.world_mut().resource_mut::<ActiveGrowthTip>().0 = Some(tip_1_entity);

        let tip_2_position = Vec2::new(20.0, 40.0);
        let tip_2_entity = app
            .world_mut()
            .spawn((
                GrowthTip::default(),
                TendrilPosition::new(tip_2_position, Vec2::X),
            ))
            .id();

        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(Vec2::new(20.0, 40.0));

        app.world_mut()
            .resource_mut::<InputActions>()
            .primary_just_pressed = true;

        app.update();

        let tip_1 = app.world().get::<GrowthTip>(tip_1_entity).unwrap();
        assert!(!tip_1.selected);

        let tip_2 = app.world().get::<GrowthTip>(tip_2_entity).unwrap();
        assert!(tip_2.selected);

        let active = app.world().resource::<ActiveGrowthTip>();
        assert_eq!(active.0, Some(tip_2_entity));
    }

    #[test]
    /// Spawn two tips within selection radius, click between them: closest is selected
    fn test_selects_closest_tip_when_multiple_in_radius() {
        let mut app = create_test_app();
        app.update();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        let cursor_pos = Vec2::new(50.0, 50.0);

        let tip_1_position = Vec2::new(50.0, 58.0);
        let tip_1_entity = app
            .world_mut()
            .spawn((
                GrowthTip::default(),
                TendrilPosition::new(tip_1_position, Vec2::X),
            ))
            .id();

        let tip_2_position = Vec2::new(50.0, 61.0);
        let _tip_2_entity = app
            .world_mut()
            .spawn((
                GrowthTip::default(),
                TendrilPosition::new(tip_2_position, Vec2::X),
            ))
            .id();

        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(cursor_pos);

        app.world_mut()
            .resource_mut::<InputActions>()
            .primary_just_pressed = true;

        app.update();

        let tip_1 = app.world().get::<GrowthTip>(tip_1_entity).unwrap();
        assert!(tip_1.selected);

        let active = app.world().resource::<ActiveGrowthTip>();
        assert_eq!(active.0, Some(tip_1_entity));
    }

    #[test]
    /// Spawn tip, set cursor: no tip selected.
    fn test_no_selection_without_click() {
        // Create app and init input + cursor
        let mut app = create_test_app();
        app.update();

        // Put the app in playing state
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        // Spawn tendril/tip entity
        let tip_position = Vec2::new(10.0, 20.0);
        let tip_entity = app
            .world_mut()
            .spawn((
                GrowthTip::default(),
                TendrilPosition::new(tip_position, Vec2::X),
            ))
            .id();

        // Set cursor
        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(Vec2::new(10.0, 20.0));

        app.update();

        // Get the tip entity and the active growth tip resource and check state
        let tip = app.world().get::<GrowthTip>(tip_entity).unwrap();
        assert!(!tip.selected);

        let active = app.world().resource::<ActiveGrowthTip>();
        assert!(active.0.is_none());
    }

    #[test]
    /// Spawn tip, set cursor outside window: no tip selected, no crash.
    fn test_no_crash_when_cursor_outside_window() {
        let mut app = create_test_app();
        app.update();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        let tip_position = Vec2::new(10.0, 20.0);
        let tip_entity = app
            .world_mut()
            .spawn((
                GrowthTip::default(),
                TendrilPosition::new(tip_position, Vec2::X),
            ))
            .id();

        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = None;

        app.world_mut()
            .resource_mut::<InputActions>()
            .primary_just_pressed = true;

        app.update();

        let tip = app.world().get::<GrowthTip>(tip_entity).unwrap();
        assert!(!tip.selected);

        let active = app.world().resource::<ActiveGrowthTip>();
        assert!(active.0.is_none());
    }

    #[test]
    /// Spawn no tip, set cursor and press: no tip selected, no crash.
    fn test_clicking_with_no_tips_does_nothing() {
        let mut app = create_test_app();
        app.update();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(Vec2::new(20.0, 10.0));

        app.world_mut()
            .resource_mut::<InputActions>()
            .primary_just_pressed = true;

        app.update();

        let active = app.world().resource::<ActiveGrowthTip>();
        assert!(active.0.is_none());
    }

    #[test]
    fn test_active_growth_tip_direction_updates_toward_cursor() {
        let mut app = create_test_app();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        let tip_position = Vec2::new(0.0, 0.0);
        let tip_entity = app
            .world_mut()
            .spawn((
                GrowthTip { selected: true },
                TendrilPosition::new(tip_position, Vec2::X),
            ))
            .id();

        app.world_mut().resource_mut::<ActiveGrowthTip>().0 = Some(tip_entity);

        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(Vec2::new(20.0, 0.0));

        app.update();
        app.update();

        let tip_direction = app
            .world()
            .get::<TendrilPosition>(tip_entity)
            .unwrap()
            .direction;
        assert_eq!(tip_direction, Vec2::X);

        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(Vec2::new(20.0, 20.0));

        app.update();
        app.update();

        let tip_direction = app
            .world()
            .get::<TendrilPosition>(tip_entity)
            .unwrap()
            .direction;
        assert_eq!(tip_direction, Vec2::new(20.0, 20.0).normalize());
    }

    #[test]
    fn test_direction_doesnt_change_for_unselected_tip() {
        let mut app = create_test_app();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        let tip_entity = app
            .world_mut()
            .spawn((
                GrowthTip::default(),
                TendrilPosition::new(Vec2::ZERO, Vec2::X),
            ))
            .id();

        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(Vec2::new(20.0, 0.0));

        app.update();
        app.update();

        let tip_direction = app
            .world()
            .get::<TendrilPosition>(tip_entity)
            .unwrap()
            .direction;
        assert_eq!(tip_direction, Vec2::X);

        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(Vec2::new(20.0, 20.0));

        app.update();
        app.update();

        let tip = app.world().get::<TendrilPosition>(tip_entity).unwrap();
        assert_eq!(tip.direction, Vec2::X);
    }

    #[test]
    fn test_no_crash_when_cursor_none_during_direction_update() {
        let mut app = create_test_app();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        let tip_entity = app
            .world_mut()
            .spawn((
                GrowthTip { selected: true },
                TendrilPosition::new(Vec2::ZERO, Vec2::X),
            ))
            .id();

        app.world_mut().resource_mut::<ActiveGrowthTip>().0 = Some(tip_entity);

        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = None;

        app.update();
        app.update();

        let tip = app.world().get::<TendrilPosition>(tip_entity).unwrap();
        assert_eq!(tip.direction, Vec2::X);
    }

    #[test]
    fn test_no_crash_when_active_tip_despawned() {
        let mut app = create_test_app();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        let tip_entity = app
            .world_mut()
            .spawn((
                GrowthTip { selected: true },
                TendrilPosition::new(Vec2::ZERO, Vec2::X),
            ))
            .id();

        app.world_mut().resource_mut::<ActiveGrowthTip>().0 = Some(tip_entity);

        app.world_mut().despawn(tip_entity);

        app.world_mut()
            .resource_mut::<CursorWorldPosition>()
            .position = Some(Vec2::new(20.0, 10.0));

        app.update();

        let active = app.world().resource::<ActiveGrowthTip>();
        assert_eq!(active.0, Some(tip_entity)); // Still set, but entity is gone, expected behavior.
    }

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
