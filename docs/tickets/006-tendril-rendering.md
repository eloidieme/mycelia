# [FEATURE] Tendril Rendering with Animated Lines

## Summary

Implement visual rendering of tendril segments as continuous, animated pixel-art lines connecting network nodes. This brings the fungal network to life visually.

## Motivation

The network needs clear, readable visuals that:
- Show organic, flowing tendril growth
- Distinguish different tendril types by color/pattern
- Indicate health/corruption state
- Support the pixel-art aesthetic
- Maintain 60 FPS with hundreds of segments

## Spec Reference

From `spec.md`:
- **Pixel Art:** Retro aesthetic with clear readability
- **Continuous Animated Lines:** Smooth, organic growth animation along tendril paths
- **Visual degradation:** Tendrils visually wilt/darken when damaged
- **60 FPS minimum:** Performance is non-negotiable

## Proposed Implementation

### Rendering Approach

Use Bevy's `Gizmos` for initial prototype, then optimize with custom mesh batching if needed.

```rust
// Option A: Gizmos (simple, good for prototype)
fn render_tendrils_gizmos(
    mut gizmos: Gizmos,
    segments: Query<(&TendrilPosition, &TendrilSegment, &NetworkParent)>,
    positions: Query<&TendrilPosition>,
) {
    for (pos, segment, parent) in segments.iter() {
        if let Ok(parent_pos) = positions.get(parent.0) {
            let color = segment_color(segment);
            gizmos.line_2d(parent_pos.position, pos.position, color);
        }
    }
}

// Option B: Custom mesh batching (optimized)
// Build a single mesh with all line segments for minimal draw calls
```

### Components

```rust
// src/game/network/rendering.rs

/// Visual style configuration for a tendril
#[derive(Component, Debug)]
pub struct TendrilStyle {
    /// Base color for this tendril type
    pub color: Color,
    /// Line thickness in pixels
    pub thickness: f32,
    /// Animation phase offset (for flowing effect)
    pub anim_offset: f32,
}

impl TendrilStyle {
    pub fn for_type(tendril_type: TendrilType) -> Self {
        match tendril_type {
            TendrilType::Basic => Self {
                color: Color::srgb(0.4, 0.7, 0.3),  // Green
                thickness: 3.0,
                anim_offset: 0.0,
            },
            TendrilType::Toxic => Self {
                color: Color::srgb(0.6, 0.2, 0.7),  // Purple
                thickness: 3.0,
                anim_offset: 0.0,
            },
            TendrilType::Sticky => Self {
                color: Color::srgb(0.8, 0.6, 0.2),  // Amber
                thickness: 4.0,
                anim_offset: 0.0,
            },
            TendrilType::Explosive => Self {
                color: Color::srgb(0.9, 0.3, 0.2),  // Red-orange
                thickness: 3.0,
                anim_offset: 0.0,
            },
        }
    }
}

/// Tracks animation state for flowing effect
#[derive(Resource, Default)]
pub struct TendrilAnimationState {
    pub time: f32,
    pub flow_speed: f32,
}
```

### Color Calculations

```rust
fn segment_color(segment: &TendrilSegment, style: &TendrilStyle) -> Color {
    let base = style.color;

    // Darken based on damage
    let health_factor = segment.health / segment.max_health;
    let damaged_color = base * health_factor.max(0.3);

    // Blend toward corruption color if corrupted
    if segment.corrupted {
        let corruption_color = Color::srgb(0.5, 0.1, 0.4);
        lerp_color(damaged_color, corruption_color, segment.corruption_level)
    } else {
        damaged_color
    }
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let a_rgba = a.to_srgba();
    let b_rgba = b.to_srgba();
    Color::srgba(
        a_rgba.red + (b_rgba.red - a_rgba.red) * t,
        a_rgba.green + (b_rgba.green - a_rgba.green) * t,
        a_rgba.blue + (b_rgba.blue - a_rgba.blue) * t,
        a_rgba.alpha + (b_rgba.alpha - a_rgba.alpha) * t,
    )
}
```

### Systems

1. `update_tendril_animation` - Advance animation time
2. `render_tendrils` - Draw all tendril segments
3. `render_growth_tips` - Highlight growth tips (pulsing)
4. `render_selected_tip` - Extra highlight for selected tip

### Rendering Order

```rust
// Z-ordering for proper layering
const Z_TENDRIL_BASE: f32 = 0.0;
const Z_TENDRIL_SELECTED: f32 = 1.0;
const Z_GROWTH_TIP: f32 = 2.0;
const Z_CORE: f32 = 3.0;
```

### File Structure

```
src/game/network/
├── mod.rs
├── components.rs
├── resources.rs
├── graph.rs
├── rendering.rs    # TendrilStyle, render systems
└── systems.rs
```

## Test Plan (TDD)

### Unit Tests

```rust
#[test]
fn test_tendril_style_for_each_type() {
    let basic = TendrilStyle::for_type(TendrilType::Basic);
    let toxic = TendrilStyle::for_type(TendrilType::Toxic);

    // Different types have different colors
    assert_ne!(basic.color, toxic.color);
    // All have positive thickness
    assert!(basic.thickness > 0.0);
}

#[test]
fn test_segment_color_healthy() {
    let segment = TendrilSegment {
        health: 100.0,
        max_health: 100.0,
        corrupted: false,
        ..default()
    };
    let style = TendrilStyle::for_type(TendrilType::Basic);

    let color = segment_color(&segment, &style);
    // Should be close to base color
    assert_eq!(color, style.color);
}

#[test]
fn test_segment_color_damaged() {
    let segment = TendrilSegment {
        health: 50.0,
        max_health: 100.0,
        corrupted: false,
        ..default()
    };
    let style = TendrilStyle::for_type(TendrilType::Basic);

    let color = segment_color(&segment, &style);
    // Should be darker than base
    let base_luminance = luminance(style.color);
    let result_luminance = luminance(color);
    assert!(result_luminance < base_luminance);
}

#[test]
fn test_segment_color_corrupted() {
    let segment = TendrilSegment {
        health: 100.0,
        max_health: 100.0,
        corrupted: true,
        corruption_level: 0.5,
        ..default()
    };
    let style = TendrilStyle::for_type(TendrilType::Basic);

    let color = segment_color(&segment, &style);
    // Should be shifted toward purple/corruption color
    let color_rgba = color.to_srgba();
    assert!(color_rgba.red > 0.3); // Has some red from corruption
}

#[test]
fn test_lerp_color() {
    let a = Color::srgb(0.0, 0.0, 0.0);
    let b = Color::srgb(1.0, 1.0, 1.0);

    let mid = lerp_color(a, b, 0.5);
    let mid_rgba = mid.to_srgba();

    assert!((mid_rgba.red - 0.5).abs() < 0.01);
    assert!((mid_rgba.green - 0.5).abs() < 0.01);
}
```

### Integration Tests

```rust
#[test]
fn test_tendrils_render_without_panic() {
    let mut app = App::new();
    // Setup with rendering plugin
    // Spawn core + some segments

    // Run multiple frames
    for _ in 0..10 {
        app.update();
    }
    // If we get here without panic, rendering works
}

#[test]
fn test_rendering_performance_many_segments() {
    let mut app = App::new();
    // Setup
    // Spawn 500 segments

    let start = std::time::Instant::now();
    for _ in 0..60 {
        app.update();
    }
    let elapsed = start.elapsed();

    // 60 frames should take ~1 second at 60 FPS
    // Allow 2 seconds for test overhead
    assert!(elapsed.as_secs_f32() < 2.0);
}
```

## Acceptance Criteria

- [ ] Tendril segments render as connected lines
- [ ] Lines connect segment positions to parent positions
- [ ] Each tendril type has distinct color
- [ ] Damaged segments appear darker
- [ ] Corrupted segments blend toward purple
- [ ] Growth tips have visual indicator (pulse/glow)
- [ ] Selected tip has additional highlight
- [ ] Rendering maintains 60 FPS with 200+ segments
- [ ] All unit tests pass
- [ ] All integration tests pass

## Dependencies

- **Ticket #5:** Tendril data structure (positions, types, health)
- **Ticket #4:** Core node (root of render tree)
- **Ticket #1:** Camera system (for proper projection)

## Out of Scope

- Animated growth (showing segment extending)
- Particle effects (spores, etc.)
- Tendril junction visuals
- Post-processing effects
- Custom shaders
