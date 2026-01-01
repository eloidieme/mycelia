# [FEATURE] Core Node Spawning and Rendering

## Summary

Implement the core node - the origin point of the fungal network and the player's "heart". If destroyed by corruption, the game ends.

## Motivation

The core node is central to Mycelia's design:
- **Origin point:** All tendrils ultimately connect back to the core
- **Lose condition:** Corruption reaching the core = game over
- **Visual anchor:** Player's reference point in the world
- **Starting point:** Where the network begins each run

This is the first actual game entity, building on the camera and input infrastructure.

## Spec Reference

From `spec.md`:
- **Lose Condition:** Corruption reaches core node = game over
- **Starting State:** Begin with a small established core network
- **Core health only:** Track/display core node's health (in addition to total mass)

## Proposed Implementation

### Components

```rust
// Already exists in src/game/network/components.rs
#[derive(Component, Debug, Default)]
pub struct CoreNode;

// New components
/// Health component for damageable entities
#[derive(Component, Debug)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn percentage(&self) -> f32 {
        self.current / self.max
    }
}

/// Marker that this entity is part of the fungal network
#[derive(Component, Debug, Default)]
pub struct NetworkMember;

/// Visual configuration for network entities
#[derive(Component, Debug)]
pub struct NetworkVisuals {
    pub base_color: Color,
    pub corruption_color: Color,
}
```

### Resources

```rust
/// Reference to the core node entity
#[derive(Resource)]
pub struct CoreNodeEntity(pub Entity);
```

### Systems

1. `spawn_core_node` - Spawns core node when entering Playing state
2. `despawn_core_node` - Cleans up when leaving Playing/GameOver
3. `check_core_death` - Triggers GameOver if core health reaches zero
4. `render_core_node` - Update visual based on health/corruption

### Visuals

For the prototype, the core node will be rendered as:
- A larger circular sprite (placeholder)
- Pulsing glow effect based on health
- Color shifts toward corruption color when infected

```rust
// Spawn bundle
commands.spawn((
    CoreNode,
    NetworkMember,
    Health::new(100.0),
    TendrilSegment::default(),
    NetworkVisuals {
        base_color: Color::srgb(0.4, 0.8, 0.4), // Healthy green
        corruption_color: Color::srgb(0.6, 0.1, 0.4), // Corruption purple
    },
    Sprite {
        color: Color::srgb(0.4, 0.8, 0.4),
        custom_size: Some(Vec2::splat(32.0)),
        ..default()
    },
    Transform::from_xyz(0.0, 0.0, 0.0),
));
```

### File Changes

```
src/game/network/
├── mod.rs          # Update plugin
├── components.rs   # Add Health, NetworkMember, NetworkVisuals
├── systems.rs      # Add core node systems
└── resources.rs    # Add CoreNodeEntity (new file)
```

## Test Plan (TDD)

### Unit Tests

```rust
#[test]
fn test_health_new() {
    let health = Health::new(100.0);
    assert_eq!(health.current, 100.0);
    assert_eq!(health.max, 100.0);
}

#[test]
fn test_health_damage() {
    let mut health = Health::new(100.0);
    health.damage(30.0);
    assert_eq!(health.current, 70.0);
}

#[test]
fn test_health_damage_does_not_go_negative() {
    let mut health = Health::new(50.0);
    health.damage(100.0);
    assert_eq!(health.current, 0.0);
}

#[test]
fn test_health_heal() {
    let mut health = Health::new(100.0);
    health.damage(50.0);
    health.heal(30.0);
    assert_eq!(health.current, 80.0);
}

#[test]
fn test_health_heal_does_not_exceed_max() {
    let mut health = Health::new(100.0);
    health.damage(10.0);
    health.heal(50.0);
    assert_eq!(health.current, 100.0);
}

#[test]
fn test_health_is_dead() {
    let mut health = Health::new(100.0);
    assert!(!health.is_dead());
    health.damage(100.0);
    assert!(health.is_dead());
}

#[test]
fn test_health_percentage() {
    let mut health = Health::new(100.0);
    assert_eq!(health.percentage(), 1.0);
    health.damage(25.0);
    assert_eq!(health.percentage(), 0.75);
}
```

### Integration Tests

```rust
#[test]
fn test_core_node_spawns_on_playing_enter() {
    let mut app = App::new();
    // Setup with NetworkPlugin
    // Transition to Playing state

    app.update();

    // Assert CoreNode entity exists
    let core_query = app.world.query_filtered::<Entity, With<CoreNode>>();
    assert_eq!(core_query.iter(&app.world).count(), 1);
}

#[test]
fn test_core_node_entity_resource_set() {
    let mut app = App::new();
    // Setup and transition to Playing

    app.update();

    // Assert CoreNodeEntity resource exists and points to valid entity
    let core_entity = app.world.resource::<CoreNodeEntity>();
    assert!(app.world.get_entity(core_entity.0).is_some());
}

#[test]
fn test_core_death_triggers_game_over() {
    let mut app = App::new();
    // Setup, transition to Playing
    // Get core node, set health to 0

    app.update();

    // Assert state is now GameOver
    assert_eq!(*app.world.resource::<State<GameState>>(), GameState::GameOver);
}

#[test]
fn test_core_node_despawns_on_menu_return() {
    let mut app = App::new();
    // Setup, play, then return to menu

    app.update();

    // Assert no CoreNode entities exist
    let core_query = app.world.query_filtered::<Entity, With<CoreNode>>();
    assert_eq!(core_query.iter(&app.world).count(), 0);
}
```

## Acceptance Criteria

- [ ] `Health` component with damage/heal/percentage methods
- [ ] `CoreNode` marker component identifies the core
- [ ] `CoreNodeEntity` resource provides quick access to core
- [ ] Core node spawns at (0, 0) when entering Playing state
- [ ] Core node has visible sprite rendering
- [ ] Core death (health = 0) triggers GameOver state
- [ ] Core node despawns when returning to Menu
- [ ] All unit tests pass
- [ ] All integration tests pass

## Dependencies

- **Ticket #2:** Game state machine (for Playing/GameOver states)

## Out of Scope

- Corruption spreading to core (future ticket)
- Core visual effects (pulse, glow)
- Core upgrades/abilities
- Multiple cores
