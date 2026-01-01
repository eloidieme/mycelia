# [FEATURE] Debug Overlay System

## Summary

Implement a toggle-able debug overlay displaying FPS, entity counts, network statistics, and visual debugging tools for development and testing.

## Motivation

A debug overlay is essential for:
- **Performance monitoring:** FPS counter to ensure 60 FPS target
- **Development insight:** Entity counts, resource values, game state
- **Visual debugging:** Network graph structure, collision boxes
- **TDD support:** Verify system behavior visually during development

This is developer tooling that can be compiled out for release builds.

## Spec Reference

From `spec.md`:
- **60 FPS minimum:** Need FPS counter to verify
- **Network Stats:** Total mass, segment count, territory coverage

## Proposed Implementation

### Resources

```rust
// src/game/debug/resources.rs

/// Controls debug overlay visibility
#[derive(Resource, Debug, Default)]
pub struct DebugSettings {
    /// Master toggle for all debug displays
    pub enabled: bool,
    /// Show FPS counter
    pub show_fps: bool,
    /// Show entity count
    pub show_entity_count: bool,
    /// Show network statistics
    pub show_network_stats: bool,
    /// Show nutrient values
    pub show_nutrients: bool,
    /// Show game state
    pub show_game_state: bool,
    /// Visualize network graph edges
    pub show_network_graph: bool,
    /// Show cursor world position
    pub show_cursor_position: bool,
}

impl DebugSettings {
    pub fn all_enabled() -> Self {
        Self {
            enabled: true,
            show_fps: true,
            show_entity_count: true,
            show_network_stats: true,
            show_nutrients: true,
            show_game_state: true,
            show_network_graph: false, // Off by default, can be heavy
            show_cursor_position: true,
        }
    }
}

/// Tracks frame timing for FPS calculation
#[derive(Resource, Debug, Default)]
pub struct FrameTimeTracker {
    pub frame_times: Vec<f32>,
    pub max_samples: usize,
}

impl FrameTimeTracker {
    pub fn new(max_samples: usize) -> Self {
        Self {
            frame_times: Vec::with_capacity(max_samples),
            max_samples,
        }
    }

    pub fn record(&mut self, delta: f32) {
        if self.frame_times.len() >= self.max_samples {
            self.frame_times.remove(0);
        }
        self.frame_times.push(delta);
    }

    pub fn average_fps(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        let avg_delta: f32 = self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32;
        if avg_delta > 0.0 { 1.0 / avg_delta } else { 0.0 }
    }

    pub fn min_fps(&self) -> f32 {
        self.frame_times.iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .map(|max_delta| if *max_delta > 0.0 { 1.0 / max_delta } else { 0.0 })
            .unwrap_or(0.0)
    }
}
```

### Components

```rust
// src/game/debug/components.rs

/// Marker for the debug UI root
#[derive(Component)]
pub struct DebugOverlay;

/// Marker for FPS text
#[derive(Component)]
pub struct FpsText;

/// Marker for entity count text
#[derive(Component)]
pub struct EntityCountText;

/// Marker for network stats text
#[derive(Component)]
pub struct NetworkStatsText;

/// Marker for nutrients text
#[derive(Component)]
pub struct NutrientsText;

/// Marker for game state text
#[derive(Component)]
pub struct GameStateText;

/// Marker for cursor position text
#[derive(Component)]
pub struct CursorPositionText;
```

### Systems

```rust
// src/game/debug/systems.rs

/// Toggle debug overlay with F3 key
pub fn toggle_debug_overlay(
    input: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<DebugSettings>,
) {
    if input.just_pressed(KeyCode::F3) {
        settings.enabled = !settings.enabled;
    }
    // F4 toggles network graph visualization
    if input.just_pressed(KeyCode::F4) {
        settings.show_network_graph = !settings.show_network_graph;
    }
}

/// Track frame times
pub fn track_frame_time(
    time: Res<Time>,
    mut tracker: ResMut<FrameTimeTracker>,
) {
    tracker.record(time.delta_secs());
}

/// Update FPS display
pub fn update_fps_display(
    tracker: Res<FrameTimeTracker>,
    settings: Res<DebugSettings>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    if !settings.enabled || !settings.show_fps {
        return;
    }
    for mut text in query.iter_mut() {
        let fps = tracker.average_fps();
        let min_fps = tracker.min_fps();
        *text = Text::from_section(
            format!("FPS: {:.0} (min: {:.0})", fps, min_fps),
            TextStyle::default(),
        );
    }
}

/// Update network stats display
pub fn update_network_stats_display(
    network_stats: Res<NetworkStats>,
    settings: Res<DebugSettings>,
    mut query: Query<&mut Text, With<NetworkStatsText>>,
) {
    if !settings.enabled || !settings.show_network_stats {
        return;
    }
    for mut text in query.iter_mut() {
        *text = Text::from_section(
            format!(
                "Network: {} segments, {:.0} mass, {:.1}% territory",
                network_stats.segment_count,
                network_stats.total_mass,
                network_stats.territory_coverage * 100.0,
            ),
            TextStyle::default(),
        );
    }
}

/// Visualize network graph edges
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
            // Draw debug line (different color from render)
            gizmos.line_2d(
                parent_pos.position,
                pos.position,
                Color::srgba(1.0, 1.0, 0.0, 0.5), // Yellow, semi-transparent
            );
        }
    }
}
```

### Spawn Debug UI

```rust
pub fn spawn_debug_overlay(mut commands: Commands) {
    commands.spawn((
        DebugOverlay,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            ..default()
        },
    )).with_children(|parent| {
        // FPS
        parent.spawn((
            FpsText,
            Text::new("FPS: --"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::WHITE),
        ));
        // Entity count
        parent.spawn((
            EntityCountText,
            Text::new("Entities: --"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::WHITE),
        ));
        // Network stats
        parent.spawn((
            NetworkStatsText,
            Text::new("Network: --"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::WHITE),
        ));
        // Nutrients
        parent.spawn((
            NutrientsText,
            Text::new("Nutrients: --"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::WHITE),
        ));
        // Game state
        parent.spawn((
            GameStateText,
            Text::new("State: --"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::WHITE),
        ));
        // Cursor position
        parent.spawn((
            CursorPositionText,
            Text::new("Cursor: --"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::WHITE),
        ));
    });
}
```

### File Structure

```
src/game/debug/
├── mod.rs          # Plugin registration
├── resources.rs    # DebugSettings, FrameTimeTracker
├── components.rs   # UI markers
└── systems.rs      # Toggle, update, render systems
```

## Test Plan (TDD)

### Unit Tests

```rust
#[test]
fn test_debug_settings_default_disabled() {
    let settings = DebugSettings::default();
    assert!(!settings.enabled);
}

#[test]
fn test_debug_settings_all_enabled() {
    let settings = DebugSettings::all_enabled();
    assert!(settings.enabled);
    assert!(settings.show_fps);
    assert!(settings.show_network_stats);
}

#[test]
fn test_frame_time_tracker_average() {
    let mut tracker = FrameTimeTracker::new(5);
    tracker.record(0.016); // ~60 FPS
    tracker.record(0.016);
    tracker.record(0.016);

    let fps = tracker.average_fps();
    assert!((fps - 60.0).abs() < 5.0); // Within 5 FPS
}

#[test]
fn test_frame_time_tracker_min_fps() {
    let mut tracker = FrameTimeTracker::new(5);
    tracker.record(0.016);  // 60 FPS
    tracker.record(0.033);  // ~30 FPS (worst frame)
    tracker.record(0.016);

    let min_fps = tracker.min_fps();
    assert!((min_fps - 30.0).abs() < 5.0);
}

#[test]
fn test_frame_time_tracker_max_samples() {
    let mut tracker = FrameTimeTracker::new(3);
    tracker.record(0.1);
    tracker.record(0.2);
    tracker.record(0.3);
    tracker.record(0.4); // Should push out 0.1

    assert_eq!(tracker.frame_times.len(), 3);
    assert!(!tracker.frame_times.contains(&0.1));
}

#[test]
fn test_frame_time_tracker_empty() {
    let tracker = FrameTimeTracker::new(5);
    assert_eq!(tracker.average_fps(), 0.0);
    assert_eq!(tracker.min_fps(), 0.0);
}
```

### Integration Tests

```rust
#[test]
fn test_f3_toggles_debug() {
    let mut app = App::new();
    // Setup with DebugPlugin
    // Simulate F3 key press

    let settings = app.world.resource::<DebugSettings>();
    assert!(settings.enabled);

    // Press F3 again
    // Assert disabled
}

#[test]
fn test_debug_overlay_spawns() {
    let mut app = App::new();
    // Setup

    app.update();

    // Check DebugOverlay entity exists
    let overlay_query = app.world.query_filtered::<Entity, With<DebugOverlay>>();
    assert_eq!(overlay_query.iter(&app.world).count(), 1);
}

#[test]
fn test_fps_updates_each_frame() {
    let mut app = App::new();
    // Setup with debug enabled

    for _ in 0..10 {
        app.update();
    }

    let tracker = app.world.resource::<FrameTimeTracker>();
    assert!(tracker.frame_times.len() >= 10);
}
```

## Acceptance Criteria

- [ ] `DebugSettings` resource controls visibility of debug elements
- [ ] F3 key toggles debug overlay on/off
- [ ] F4 key toggles network graph visualization
- [ ] FPS counter shows average and minimum FPS
- [ ] Entity count display shows total entities
- [ ] Network stats show segment count, mass, territory
- [ ] Nutrients display shows current/max
- [ ] Game state display shows current GameState
- [ ] Cursor position shows world coordinates
- [ ] Network graph visualization draws connections (optional toggle)
- [ ] Debug overlay positioned in top-left corner
- [ ] All text updates every frame when enabled
- [ ] Debug systems have minimal performance impact
- [ ] All unit tests pass
- [ ] All integration tests pass

## Dependencies

- **Ticket #1:** Camera (for cursor world position)
- **Ticket #2:** Game state (for state display)
- **Ticket #3:** Input (for cursor position)
- **Ticket #5:** Network stats (for network display)
- **Ticket #7:** Nutrients (for nutrient display)

## Out of Scope

- Imgui-style interactive debug UI
- In-game console
- Entity inspector
- Performance profiler integration
- Log viewer
