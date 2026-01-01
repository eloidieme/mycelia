# [FEATURE] Input Handling System

## Summary

Implement a centralized input handling system that tracks mouse position in world coordinates, mouse clicks, keyboard input, and provides gamepad support foundation.

## Motivation

Mycelia's core mechanic requires precise input handling:
- **Mouse position** drives tendril growth direction (tip follows cursor)
- **Click** selects active growth tips
- **Keyboard** handles pause, camera pan, abilities
- **Gamepad** support is required for full release

A centralized input system provides:
- World-space cursor position (accounting for camera zoom/pan)
- Clean abstraction over input devices
- Easy testing of input-dependent systems

## Spec Reference

From `spec.md`:
- **Mouse-from-Tip Control:** Active tendril tip chases mouse cursor position
- **Explicit Tip Selection:** Click on a tendril tip to select it
- **Input Support:** Mouse + Keyboard (primary), Gamepad (full support)
- **Camera Controls:** Keyboard pan + scroll wheel zoom

## Proposed Implementation

### Resources

```rust
// src/game/input/resources.rs

/// Tracks cursor position in world coordinates
#[derive(Resource, Default)]
pub struct CursorWorldPosition {
    /// Position in world space, None if cursor outside window
    pub position: Option<Vec2>,
}

/// Tracks current input actions (abstracted from device)
#[derive(Resource, Default)]
pub struct InputActions {
    /// Direction for growth/camera movement
    pub move_direction: Vec2,
    /// Primary action (select tip, confirm)
    pub primary_action: ActionState,
    /// Secondary action (cancel, alternative)
    pub secondary_action: ActionState,
    /// Pause toggle
    pub pause_pressed: bool,
    /// Camera zoom delta (-1.0 to 1.0)
    pub zoom_delta: f32,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ActionState {
    #[default]
    Released,
    JustPressed,
    Held,
    JustReleased,
}
```

### Systems

1. `update_cursor_world_position` - Convert screen cursor to world coords using camera
2. `update_keyboard_input` - Read keyboard state into InputActions
3. `update_mouse_input` - Read mouse buttons into InputActions
4. `update_gamepad_input` - Read gamepad state into InputActions (additive)
5. `reset_input_actions` - Reset "just pressed" states at frame end

### Input Mapping

| Action | Keyboard | Mouse | Gamepad |
|--------|----------|-------|---------|
| Move/Aim | WASD/Arrows | Cursor position | Left stick |
| Primary | Space | Left click | A/Cross |
| Secondary | Shift | Right click | B/Circle |
| Pause | Escape | - | Start |
| Zoom In | + / = | Scroll up | RB |
| Zoom Out | - | Scroll down | LB |

### File Structure

```
src/game/input/
├── mod.rs          # Plugin registration
├── resources.rs    # CursorWorldPosition, InputActions
└── systems.rs      # Input handling systems
```

## Test Plan (TDD)

### Unit Tests

```rust
#[test]
fn test_cursor_world_position_default_is_none() {
    let cursor = CursorWorldPosition::default();
    assert!(cursor.position.is_none());
}

#[test]
fn test_input_actions_default() {
    let actions = InputActions::default();
    assert_eq!(actions.move_direction, Vec2::ZERO);
    assert_eq!(actions.primary_action, ActionState::Released);
    assert!(!actions.pause_pressed);
}

#[test]
fn test_action_state_transitions() {
    // Test JustPressed -> Held -> JustReleased -> Released cycle
}

#[test]
fn test_screen_to_world_conversion() {
    // Given camera at position (100, 50) with zoom 2.0
    // And screen position (640, 360) (center of 1280x720)
    // World position should be (100, 50)

    // Given screen position (0, 0) (top-left)
    // World position should be offset by half screen / zoom
}
```

### Integration Tests

```rust
#[test]
fn test_cursor_position_updates_each_frame() {
    let mut app = App::new();
    // Setup with InputPlugin and mock window
    // Simulate cursor movement
    // Assert CursorWorldPosition updates
}

#[test]
fn test_keyboard_input_maps_to_actions() {
    let mut app = App::new();
    // Setup
    // Simulate WASD key press
    // Assert move_direction is non-zero
}

#[test]
fn test_input_works_with_camera_zoom() {
    // Cursor at screen center with zoomed out camera
    // Should still map to correct world position
}
```

## Acceptance Criteria

- [ ] `CursorWorldPosition` resource tracks mouse in world coordinates
- [ ] World position correctly accounts for camera position and zoom
- [ ] `InputActions` resource provides device-agnostic input state
- [ ] WASD/Arrows update `move_direction`
- [ ] Mouse left click updates `primary_action`
- [ ] Scroll wheel updates `zoom_delta`
- [ ] Escape key sets `pause_pressed`
- [ ] Gamepad input additively updates `InputActions`
- [ ] All tests pass
- [ ] Input lag is imperceptible (< 1 frame)

## Dependencies

- **Ticket #1:** Camera system (needed for screen-to-world conversion)

## Out of Scope

- Key rebinding/configuration
- Input recording/replay
- Touch/mobile input
- Complex gesture recognition
