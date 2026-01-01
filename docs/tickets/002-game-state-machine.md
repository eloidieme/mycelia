# [FEATURE] Game State Machine Implementation

## Summary

Implement the full game state machine with proper transitions, true pause functionality, and state-dependent system scheduling.

## Motivation

The game needs clear separation between different modes of operation:
- **Menu:** Title screen, strain selection (future)
- **Playing:** Active gameplay with all systems running
- **Paused:** True pause - everything frozen, can review network
- **Upgrading:** Upgrade selection screen (pauses gameplay)
- **GameOver:** End screen with stats

This enables proper game flow and the "true pause" requirement from the spec.

## Spec Reference

From `spec.md`:
- **True Pause:** Everything freezes, can review network and plan
- **Pauses during upgrade selection**

## Proposed Implementation

### State Definition

Already defined in `src/lib.rs`:
```rust
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    Upgrading,
    GameOver,
}
```

### New Components/Systems

```rust
// src/game/state/mod.rs

/// Resource tracking run statistics
#[derive(Resource, Default)]
pub struct RunStats {
    pub elapsed_time: f32,
    pub enemies_killed: u32,
    pub max_territory: f32,
    pub nutrients_collected: f32,
}

/// Event for requesting state transitions
#[derive(Event)]
pub struct StateTransitionRequest {
    pub target: GameState,
}
```

### Systems

1. `handle_pause_input` - Toggle pause on Escape key (Playing <-> Paused)
2. `handle_state_transitions` - Process StateTransitionRequest events
3. `reset_run_stats` - Reset stats when entering Playing from Menu
4. `pause_game_time` - Pause Bevy's Time when in Paused/Upgrading states
5. `resume_game_time` - Resume Time when returning to Playing

### System Scheduling

```rust
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RunStats>()
            .add_event::<StateTransitionRequest>()
            // Only run gameplay systems in Playing state
            .configure_sets(Update, GameplaySet.run_if(in_state(GameState::Playing)))
            // Pause input works in Playing and Paused
            .add_systems(Update, handle_pause_input
                .run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))))
            // State entry/exit
            .add_systems(OnEnter(GameState::Playing), reset_run_stats)
            .add_systems(OnEnter(GameState::Paused), pause_game_time)
            .add_systems(OnExit(GameState::Paused), resume_game_time)
            .add_systems(OnEnter(GameState::Upgrading), pause_game_time)
            .add_systems(OnExit(GameState::Upgrading), resume_game_time);
    }
}
```

### File Structure

```
src/game/
├── state/
│   ├── mod.rs          # Plugin, StateTransitionRequest
│   ├── components.rs   # RunStats
│   └── systems.rs      # State handling systems
```

## Test Plan (TDD)

### Unit Tests

```rust
#[test]
fn test_run_stats_default() {
    let stats = RunStats::default();
    assert_eq!(stats.elapsed_time, 0.0);
    assert_eq!(stats.enemies_killed, 0);
}

#[test]
fn test_state_transition_request_creation() {
    let request = StateTransitionRequest { target: GameState::Paused };
    assert_eq!(request.target, GameState::Paused);
}
```

### Integration Tests

```rust
#[test]
fn test_game_starts_in_menu_state() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .init_state::<GameState>();

    app.update();

    assert_eq!(*app.world.resource::<State<GameState>>(), GameState::Menu);
}

#[test]
fn test_pause_toggle_from_playing() {
    let mut app = App::new();
    // Setup with StatePlugin
    // Set state to Playing
    // Simulate Escape key press
    // Assert state is now Paused
}

#[test]
fn test_pause_freezes_game_time() {
    let mut app = App::new();
    // Setup app
    // Transition to Paused
    // Assert Time is paused (virtual time not advancing)
}

#[test]
fn test_gameplay_systems_only_run_in_playing() {
    // Setup app with a dummy gameplay system that increments a counter
    // Run update in Menu state - counter should not increment
    // Transition to Playing - counter should increment
    // Transition to Paused - counter should not increment
}
```

## Acceptance Criteria

- [ ] GameState enum has all required states (Menu, Playing, Paused, Upgrading, GameOver)
- [ ] Escape key toggles between Playing and Paused states
- [ ] Game time freezes in Paused and Upgrading states (true pause)
- [ ] Game time resumes when returning to Playing
- [ ] RunStats resource tracks run statistics
- [ ] Gameplay systems only run in Playing state
- [ ] State transitions are clean with proper OnEnter/OnExit hooks
- [ ] All tests pass

## Dependencies

None - builds on existing GameState in lib.rs

## Out of Scope

- Menu UI rendering
- GameOver screen UI
- Upgrade selection UI (separate ticket)
- Strain/character selection
