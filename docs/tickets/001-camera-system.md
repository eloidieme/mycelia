# [FEATURE] Basic 2D Camera with Dynamic Zoom Foundation

## Summary

Implement the foundational camera system with orthographic 2D projection, basic controls, and infrastructure for future dynamic zoom based on network size.

## Motivation

The camera is the player's window into the game world. Mycelia requires a camera that can:
- Display the 2D pixel-art game world
- Zoom out as the fungal network grows (dynamic zoom)
- Support manual pan and zoom for large networks
- Center on points of interest (core, active tip)

This ticket establishes the foundation; dynamic zoom based on network bounds comes later.

## Spec Reference

From `spec.md`:
- **Dynamic Zoom:** Camera auto-zooms to show the entire network
- **Multi-Screen Sprawl:** Network can extend beyond visible area
- **Camera Controls:** Minimap click, tip-centered lock, keyboard pan + scroll wheel zoom

## Proposed Implementation

### Components

```rust
// src/game/camera/components.rs

/// Marker for the main game camera
#[derive(Component)]
pub struct MainCamera;

/// Camera configuration and state
#[derive(Component)]
pub struct CameraController {
    /// Current zoom level (1.0 = default)
    pub zoom: f32,
    /// Minimum zoom (zoomed in)
    pub min_zoom: f32,
    /// Maximum zoom (zoomed out)
    pub max_zoom: f32,
    /// Pan speed in world units per second
    pub pan_speed: f32,
    /// Whether camera is locked to a target
    pub locked_target: Option<Entity>,
}

/// Resource for camera settings
#[derive(Resource)]
pub struct CameraSettings {
    pub default_zoom: f32,
    pub zoom_speed: f32,
}
```

### Systems

1. `spawn_camera` - Spawns Camera2d with MainCamera marker on game start
2. `camera_zoom` - Handle scroll wheel zoom input
3. `camera_pan` - Handle keyboard panning (WASD or arrows when not locked)
4. `camera_follow_target` - Follow locked target entity if set

### File Structure

```
src/game/camera/
├── mod.rs          # Plugin registration
├── components.rs   # Camera components
└── systems.rs      # Camera systems
```

## Test Plan (TDD)

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_controller_default_values() {
        let controller = CameraController::default();
        assert_eq!(controller.zoom, 1.0);
        assert!(controller.min_zoom < controller.max_zoom);
        assert!(controller.locked_target.is_none());
    }

    #[test]
    fn test_zoom_clamps_to_bounds() {
        let mut controller = CameraController {
            zoom: 1.0,
            min_zoom: 0.5,
            max_zoom: 3.0,
            ..default()
        };

        // Test clamping
        controller.zoom = 0.1;
        let clamped = controller.zoom.clamp(controller.min_zoom, controller.max_zoom);
        assert_eq!(clamped, 0.5);
    }

    #[test]
    fn test_camera_settings_resource() {
        let settings = CameraSettings::default();
        assert!(settings.zoom_speed > 0.0);
    }
}
```

### Integration Tests

```rust
#[test]
fn test_camera_spawns_on_startup() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugins(CameraPlugin)
       .init_state::<GameState>();

    app.update();

    let camera_query = app.world.query_filtered::<Entity, With<MainCamera>>();
    assert_eq!(camera_query.iter(&app.world).count(), 1);
}

#[test]
fn test_camera_zoom_changes_projection() {
    // Setup app with camera
    // Simulate scroll input
    // Assert projection scale changed
}
```

## Acceptance Criteria

- [ ] Camera spawns with orthographic 2D projection at game start
- [ ] `MainCamera` marker component identifies the game camera
- [ ] `CameraController` component tracks zoom level and pan state
- [ ] Scroll wheel zooms in/out within min/max bounds
- [ ] WASD/Arrow keys pan the camera when not locked
- [ ] Camera can lock to follow a target entity
- [ ] All unit tests pass
- [ ] Integration test confirms camera spawns correctly
- [ ] 60 FPS maintained

## Dependencies

None - this is foundational infrastructure.

## Out of Scope

- Dynamic zoom based on network bounds (future ticket)
- Minimap viewport
- Smooth zoom interpolation (can be added later)
- Screen shake effects
