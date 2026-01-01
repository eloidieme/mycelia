# [FEATURE] Basic Tendril Segment Data Structure

## Summary

Implement the data model for tendril segments including network graph structure, parent-child relationships, and connectivity tracking. This ticket focuses on the data layer; rendering is handled separately.

## Motivation

The fungal network is a graph structure where:
- Each tendril segment is a node
- Connections between segments are edges
- The core is the root of the tree
- Severance cuts edges and orphans subtrees

A robust data structure enables:
- Efficient network traversal
- Severance detection and handling
- Growth pathfinding
- Territory calculation

## Spec Reference

From `spec.md`:
- **Network Vulnerability:** Thin/spread tendrils can be severed, cutting off parts
- **Slow Decay:** Severed parts wither over time, giving chance to reconnect
- **Free Overlap:** Tendrils can layer and cross freely
- **Growth Tips:** Active tips that can grow

## Proposed Implementation

### Components

```rust
// src/game/network/components.rs

/// Identifies a tendril segment in the network
#[derive(Component, Debug, Clone)]
pub struct TendrilSegment {
    pub tendril_type: TendrilType,
    pub health: f32,
    pub max_health: f32,
    pub corrupted: bool,
    pub corruption_level: f32,
}

/// Connection to parent segment (toward core)
#[derive(Component, Debug)]
pub struct NetworkParent(pub Entity);

/// Connection to child segments (away from core)
#[derive(Component, Debug, Default)]
pub struct NetworkChildren(pub Vec<Entity>);

/// Marker for segments that are growth tips (can extend)
#[derive(Component, Debug, Default)]
pub struct GrowthTip {
    pub selected: bool,
}

/// Marker for segments disconnected from core
#[derive(Component, Debug)]
pub struct Severed {
    pub time_since_severance: f32,
    pub decay_rate: f32,
}

/// Position along the tendril (for rendering)
#[derive(Component, Debug, Clone)]
pub struct TendrilPosition {
    /// World position of this segment
    pub position: Vec2,
    /// Direction this segment is facing
    pub direction: Vec2,
}
```

### Resources

```rust
// src/game/network/resources.rs

/// Tracks overall network statistics
#[derive(Resource, Debug, Default)]
pub struct NetworkStats {
    pub total_mass: f32,
    pub max_mass: f32,
    pub segment_count: u32,
    pub tip_count: u32,
    pub territory_coverage: f32,
    pub connected_segments: u32,
    pub severed_segments: u32,
}

/// Configuration for network behavior
#[derive(Resource, Debug)]
pub struct NetworkConfig {
    pub segment_length: f32,
    pub segment_health: f32,
    pub decay_rate: f32,
    pub decay_start_delay: f32,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            segment_length: 16.0,
            segment_health: 50.0,
            decay_rate: 10.0,        // HP per second
            decay_start_delay: 2.0,  // Seconds before decay starts
        }
    }
}
```

### Graph Operations

```rust
// src/game/network/graph.rs

/// Check if a segment is connected to the core
pub fn is_connected_to_core(
    entity: Entity,
    parents: &Query<&NetworkParent>,
    core: Entity,
) -> bool {
    let mut current = entity;
    loop {
        if current == core {
            return true;
        }
        match parents.get(current) {
            Ok(parent) => current = parent.0,
            Err(_) => return false,
        }
    }
}

/// Find all segments that would be severed if this segment is destroyed
pub fn find_downstream_segments(
    entity: Entity,
    children: &Query<&NetworkChildren>,
) -> Vec<Entity> {
    let mut result = Vec::new();
    let mut stack = vec![entity];

    while let Some(current) = stack.pop() {
        result.push(current);
        if let Ok(kids) = children.get(current) {
            stack.extend(kids.0.iter());
        }
    }

    result
}

/// Calculate distance from core (hop count)
pub fn distance_from_core(
    entity: Entity,
    parents: &Query<&NetworkParent>,
    core: Entity,
) -> Option<u32> {
    let mut current = entity;
    let mut distance = 0;

    loop {
        if current == core {
            return Some(distance);
        }
        match parents.get(current) {
            Ok(parent) => {
                current = parent.0;
                distance += 1;
            }
            Err(_) => return None,
        }
    }
}
```

### File Structure

```
src/game/network/
├── mod.rs          # Plugin, exports
├── components.rs   # All network components
├── resources.rs    # NetworkStats, NetworkConfig
├── graph.rs        # Graph traversal utilities
└── systems.rs      # Systems (next tickets)
```

## Test Plan (TDD)

### Unit Tests

```rust
#[test]
fn test_tendril_segment_default() {
    let segment = TendrilSegment::default();
    assert_eq!(segment.tendril_type, TendrilType::Basic);
    assert!(!segment.corrupted);
}

#[test]
fn test_network_children_default_empty() {
    let children = NetworkChildren::default();
    assert!(children.0.is_empty());
}

#[test]
fn test_growth_tip_default_not_selected() {
    let tip = GrowthTip::default();
    assert!(!tip.selected);
}

#[test]
fn test_network_config_default_values() {
    let config = NetworkConfig::default();
    assert!(config.segment_length > 0.0);
    assert!(config.decay_rate > 0.0);
}

#[test]
fn test_tendril_position_direction_normalized() {
    let pos = TendrilPosition {
        position: Vec2::ZERO,
        direction: Vec2::new(3.0, 4.0).normalize(),
    };
    assert!((pos.direction.length() - 1.0).abs() < 0.001);
}
```

### Graph Operation Tests

```rust
#[test]
fn test_is_connected_to_core_direct_child() {
    let mut app = App::new();
    // Spawn core
    let core = app.world.spawn(CoreNode).id();
    // Spawn segment with core as parent
    let segment = app.world.spawn(NetworkParent(core)).id();

    // Use graph utility
    // Assert segment is connected
}

#[test]
fn test_is_connected_to_core_deep_chain() {
    // Core -> A -> B -> C
    // Assert C is connected to core
}

#[test]
fn test_is_connected_to_core_severed() {
    // Create segment with no parent
    // Assert not connected
}

#[test]
fn test_find_downstream_segments() {
    // Core -> A -> B
    //           -> C -> D
    // Find downstream from A should return [A, B, C, D]
}

#[test]
fn test_distance_from_core() {
    // Core -> A -> B -> C
    // distance(A) = 1, distance(B) = 2, distance(C) = 3
}
```

### Integration Tests

```rust
#[test]
fn test_spawn_segment_connected_to_core() {
    let mut app = App::new();
    // Setup, spawn core, spawn segment as child

    // Verify parent-child relationships
    // Verify NetworkStats updated
}

#[test]
fn test_network_stats_updates_on_segment_add() {
    let mut app = App::new();
    // Spawn core, verify segment_count = 1
    // Spawn child segment, verify segment_count = 2
}
```

## Acceptance Criteria

- [ ] `TendrilSegment` component with type, health, corruption state
- [ ] `NetworkParent` component links to parent segment
- [ ] `NetworkChildren` component tracks child segments
- [ ] `GrowthTip` marker identifies extendable tips
- [ ] `Severed` component marks disconnected segments
- [ ] `TendrilPosition` tracks world position and direction
- [ ] `NetworkStats` resource tracks network-wide statistics
- [ ] `NetworkConfig` resource defines behavior parameters
- [ ] Graph utilities: `is_connected_to_core`, `find_downstream_segments`, `distance_from_core`
- [ ] All unit tests pass
- [ ] All integration tests pass

## Dependencies

- **Ticket #4:** Core node (parent of all segments)

## Out of Scope

- Tendril rendering (Ticket #6)
- Growth mechanics (future ticket)
- Severance system implementation (future ticket)
- Corruption spread logic (future ticket)
