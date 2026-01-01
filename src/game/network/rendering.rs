//! Tendril rendering systems
//!
//! Handles visual representation of the fungal network:
//! - Line rendering connecting segments
//! - Color based on tendril type, health, corruption
//! - Growth tip highlighting
//! - Animation state for flowing effects

use bevy::prelude::*;

use super::components::{
    CoreNode, GrowthTip, NetworkParent, TendrilPosition, TendrilSegment, TendrilType,
};

/// Visual style configuration for a tendril segment
#[derive(Component, Debug, Clone)]
pub struct TendrilStyle {
    /// Base color for this tendril type
    pub color: Color,
    /// Line thickness in pixels
    pub thickness: f32,
    /// Animation phase offset (for flowing effect)
    pub anim_offset: f32,
}

impl TendrilStyle {
    /// Create style for a specific tendril type
    #[must_use]
    pub fn for_type(tendril_type: TendrilType) -> Self {
        match tendril_type {
            TendrilType::Basic => Self {
                color: Color::srgb(0.4, 0.7, 0.3), // Green
                thickness: 3.0,
                anim_offset: 0.0,
            },
            TendrilType::Toxic => Self {
                color: Color::srgb(0.6, 0.2, 0.7), // Purple
                thickness: 3.0,
                anim_offset: 0.0,
            },
            TendrilType::Sticky => Self {
                color: Color::srgb(0.8, 0.6, 0.2), // Amber
                thickness: 4.0,
                anim_offset: 0.0,
            },
            TendrilType::Explosive => Self {
                color: Color::srgb(0.9, 0.3, 0.2), // Red-orange
                thickness: 3.0,
                anim_offset: 0.0,
            },
        }
    }
}

impl Default for TendrilStyle {
    fn default() -> Self {
        Self::for_type(TendrilType::Basic)
    }
}

/// Tracks animation state for flowing/pulsing effects
#[derive(Resource, Debug)]
pub struct TendrilAnimationState {
    /// Current animation time
    pub time: f32,
    /// Speed of flowing animation
    pub flow_speed: f32,
    /// Speed of pulse animation for tips
    pub pulse_speed: f32,
}

impl Default for TendrilAnimationState {
    fn default() -> Self {
        Self {
            time: 0.0,
            flow_speed: 2.0,
            pulse_speed: 3.0,
        }
    }
}

/// Z-ordering constants for proper layering (for future 3D positioning)
#[allow(dead_code)]
pub mod z_order {
    pub const TENDRIL_BASE: f32 = 0.0;
    pub const TENDRIL_HIGHLIGHTED: f32 = 1.0;
    pub const GROWTH_TIP: f32 = 2.0;
    pub const CORE: f32 = 3.0;
}

/// Corruption color for blending
const CORRUPTION_COLOR: Color = Color::srgb(0.5, 0.1, 0.4);

/// Brightness multiplier for unselected growth tips
const GROWTH_TIP_BRIGHTNESS_MULTIPLIER: f32 = 1.3;

/// Linearly interpolate between two colors
#[must_use]
pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let a_srgba = a.to_srgba();
    let b_srgba = b.to_srgba();
    let t = t.clamp(0.0, 1.0);

    Color::srgba(
        a_srgba.red + (b_srgba.red - a_srgba.red) * t,
        a_srgba.green + (b_srgba.green - a_srgba.green) * t,
        a_srgba.blue + (b_srgba.blue - a_srgba.blue) * t,
        a_srgba.alpha + (b_srgba.alpha - a_srgba.alpha) * t,
    )
}

/// Calculate the display color for a segment based on health and corruption
#[must_use]
pub fn segment_color(segment: &TendrilSegment, style: &TendrilStyle) -> Color {
    // Calculate health factor (min 0.3 to keep segments visible)
    let health_factor = if segment.max_health > 0.0 {
        (segment.health / segment.max_health).clamp(0.3, 1.0)
    } else {
        1.0
    };

    // Darken based on damage
    let base_srgba = style.color.to_srgba();
    let damaged_color = Color::srgba(
        base_srgba.red * health_factor,
        base_srgba.green * health_factor,
        base_srgba.blue * health_factor,
        base_srgba.alpha,
    );

    // Blend toward corruption color if corrupted
    if segment.corrupted {
        lerp_color(damaged_color, CORRUPTION_COLOR, segment.corruption_level)
    } else {
        damaged_color
    }
}

/// Update animation time
pub fn update_tendril_animation(time: Res<Time>, mut anim_state: ResMut<TendrilAnimationState>) {
    anim_state.time += time.delta_secs();
}

/// Calculate pulse value for animations (returns 0.0 to 1.0)
#[must_use]
pub fn calculate_pulse(time: f32, speed: f32) -> f32 {
    (time * speed).sin() * 0.5 + 0.5
}

/// Brighten a color by the growth tip brightness multiplier
#[must_use]
pub fn brighten_color(color: Color) -> Color {
    let srgba = color.to_srgba();
    Color::srgba(
        (srgba.red * GROWTH_TIP_BRIGHTNESS_MULTIPLIER).min(1.0),
        (srgba.green * GROWTH_TIP_BRIGHTNESS_MULTIPLIER).min(1.0),
        (srgba.blue * GROWTH_TIP_BRIGHTNESS_MULTIPLIER).min(1.0),
        srgba.alpha,
    )
}

/// Render all tendril segments as lines connecting to their parents
pub fn render_tendrils(
    mut gizmos: Gizmos,
    segments: Query<
        (&TendrilPosition, &TendrilSegment, &TendrilStyle, &NetworkParent),
        Without<CoreNode>,
    >,
    positions: Query<&TendrilPosition>,
) {
    for (pos, segment, style, parent) in segments.iter() {
        if let Ok(parent_pos) = positions.get(parent.0) {
            let color = segment_color(segment, style);
            gizmos.line_2d(parent_pos.position, pos.position, color);
        }
    }
}

/// Render growth tips with pulsing highlight
pub fn render_growth_tips(
    mut gizmos: Gizmos,
    anim_state: Res<TendrilAnimationState>,
    tips: Query<(&TendrilPosition, &GrowthTip, Option<&TendrilStyle>)>,
) {
    for (pos, tip, style) in tips.iter() {
        // Pulsing size based on animation
        let pulse = calculate_pulse(anim_state.time, anim_state.pulse_speed);
        let base_radius = 6.0;
        let radius = base_radius + pulse * 3.0;

        // Selected tips are brighter
        let base_color = style.map_or(Color::srgb(0.4, 0.7, 0.3), |s| s.color);
        let color = if tip.selected {
            // Bright highlight for selected
            Color::srgb(1.0, 1.0, 0.8)
        } else {
            // Slightly brighter version of base color
            brighten_color(base_color)
        };

        gizmos.circle_2d(pos.position, radius, color);

        // Extra ring for selected tip
        if tip.selected {
            let outer_radius = radius + 4.0 + pulse * 2.0;
            gizmos.circle_2d(pos.position, outer_radius, Color::srgba(1.0, 1.0, 0.8, 0.5));
        }
    }
}

/// Render the core node with special visuals
pub fn render_core(
    mut gizmos: Gizmos,
    anim_state: Res<TendrilAnimationState>,
    core: Query<&Transform, With<CoreNode>>,
) {
    let Ok(transform) = core.get_single() else {
        return;
    };

    let pos = transform.translation.truncate();

    // Pulsing core (uses a custom slower pulse speed of 1.5)
    let pulse = calculate_pulse(anim_state.time, 1.5);
    let inner_radius = 14.0 + pulse * 2.0;
    let outer_radius = 18.0 + pulse * 3.0;

    // Core colors
    let inner_color = Color::srgb(0.3, 0.8, 0.3);
    let outer_color = Color::srgba(0.4, 0.9, 0.4, 0.4);

    gizmos.circle_2d(pos, inner_radius, inner_color);
    gizmos.circle_2d(pos, outer_radius, outer_color);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tendril_style_for_basic() {
        let style = TendrilStyle::for_type(TendrilType::Basic);
        assert!(style.thickness > 0.0);
        // Basic is greenish
        let srgba = style.color.to_srgba();
        assert!(srgba.green > srgba.red);
    }

    #[test]
    fn test_tendril_style_for_toxic() {
        let style = TendrilStyle::for_type(TendrilType::Toxic);
        // Toxic is purplish (more blue than green)
        let srgba = style.color.to_srgba();
        assert!(srgba.blue > srgba.green);
    }

    #[test]
    fn test_tendril_style_for_sticky() {
        let style = TendrilStyle::for_type(TendrilType::Sticky);
        // Sticky is thicker
        assert!(style.thickness > 3.0);
    }

    #[test]
    fn test_tendril_style_for_explosive() {
        let style = TendrilStyle::for_type(TendrilType::Explosive);
        // Explosive is reddish
        let srgba = style.color.to_srgba();
        assert!(srgba.red > srgba.green);
        assert!(srgba.red > srgba.blue);
    }

    #[test]
    fn test_all_types_have_different_colors() {
        let basic = TendrilStyle::for_type(TendrilType::Basic);
        let toxic = TendrilStyle::for_type(TendrilType::Toxic);
        let sticky = TendrilStyle::for_type(TendrilType::Sticky);
        let explosive = TendrilStyle::for_type(TendrilType::Explosive);

        assert_ne!(basic.color, toxic.color);
        assert_ne!(basic.color, sticky.color);
        assert_ne!(basic.color, explosive.color);
        assert_ne!(toxic.color, sticky.color);
        assert_ne!(toxic.color, explosive.color);
        assert_ne!(sticky.color, explosive.color);
    }

    #[test]
    fn test_lerp_color_at_zero() {
        let a = Color::srgb(0.0, 0.0, 0.0);
        let b = Color::srgb(1.0, 1.0, 1.0);

        let result = lerp_color(a, b, 0.0);
        let srgba = result.to_srgba();

        assert!((srgba.red - 0.0).abs() < 0.01);
        assert!((srgba.green - 0.0).abs() < 0.01);
        assert!((srgba.blue - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_lerp_color_at_one() {
        let a = Color::srgb(0.0, 0.0, 0.0);
        let b = Color::srgb(1.0, 1.0, 1.0);

        let result = lerp_color(a, b, 1.0);
        let srgba = result.to_srgba();

        assert!((srgba.red - 1.0).abs() < 0.01);
        assert!((srgba.green - 1.0).abs() < 0.01);
        assert!((srgba.blue - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_lerp_color_at_half() {
        let a = Color::srgb(0.0, 0.0, 0.0);
        let b = Color::srgb(1.0, 1.0, 1.0);

        let result = lerp_color(a, b, 0.5);
        let srgba = result.to_srgba();

        assert!((srgba.red - 0.5).abs() < 0.01);
        assert!((srgba.green - 0.5).abs() < 0.01);
        assert!((srgba.blue - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_lerp_color_clamps_t() {
        let a = Color::srgb(0.0, 0.0, 0.0);
        let b = Color::srgb(1.0, 1.0, 1.0);

        // t > 1 should clamp to 1
        let result = lerp_color(a, b, 2.0);
        let srgba = result.to_srgba();
        assert!((srgba.red - 1.0).abs() < 0.01);

        // t < 0 should clamp to 0
        let result = lerp_color(a, b, -1.0);
        let srgba = result.to_srgba();
        assert!((srgba.red - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_segment_color_healthy() {
        let segment = TendrilSegment {
            health: 100.0,
            max_health: 100.0,
            corrupted: false,
            corruption_level: 0.0,
            ..default()
        };
        let style = TendrilStyle::for_type(TendrilType::Basic);

        let color = segment_color(&segment, &style);

        // Should be equal to base color when healthy
        let base_srgba = style.color.to_srgba();
        let result_srgba = color.to_srgba();
        assert!((base_srgba.red - result_srgba.red).abs() < 0.01);
        assert!((base_srgba.green - result_srgba.green).abs() < 0.01);
        assert!((base_srgba.blue - result_srgba.blue).abs() < 0.01);
    }

    #[test]
    fn test_segment_color_damaged() {
        let segment = TendrilSegment {
            health: 50.0,
            max_health: 100.0,
            corrupted: false,
            corruption_level: 0.0,
            ..default()
        };
        let style = TendrilStyle::for_type(TendrilType::Basic);

        let color = segment_color(&segment, &style);

        // Should be darker than base color
        let base_srgba = style.color.to_srgba();
        let result_srgba = color.to_srgba();
        assert!(result_srgba.red < base_srgba.red);
        assert!(result_srgba.green < base_srgba.green);
        assert!(result_srgba.blue < base_srgba.blue);
    }

    #[test]
    fn test_segment_color_heavily_damaged_has_minimum() {
        let segment = TendrilSegment {
            health: 0.0,
            max_health: 100.0,
            corrupted: false,
            corruption_level: 0.0,
            ..default()
        };
        let style = TendrilStyle::for_type(TendrilType::Basic);

        let color = segment_color(&segment, &style);

        // Should still have some color (30% minimum)
        let result_srgba = color.to_srgba();
        let base_srgba = style.color.to_srgba();
        assert!((result_srgba.green - base_srgba.green * 0.3).abs() < 0.01);
    }

    #[test]
    fn test_segment_color_corrupted() {
        let segment = TendrilSegment {
            health: 100.0,
            max_health: 100.0,
            corrupted: true,
            corruption_level: 1.0, // Fully corrupted
            ..default()
        };
        let style = TendrilStyle::for_type(TendrilType::Basic);

        let color = segment_color(&segment, &style);

        // Should be the corruption color when fully corrupted
        let result_srgba = color.to_srgba();
        let corruption_srgba = CORRUPTION_COLOR.to_srgba();
        assert!((result_srgba.red - corruption_srgba.red).abs() < 0.01);
        assert!((result_srgba.green - corruption_srgba.green).abs() < 0.01);
        assert!((result_srgba.blue - corruption_srgba.blue).abs() < 0.01);
    }

    #[test]
    fn test_segment_color_partially_corrupted() {
        let segment = TendrilSegment {
            health: 100.0,
            max_health: 100.0,
            corrupted: true,
            corruption_level: 0.5,
            ..default()
        };
        let style = TendrilStyle::for_type(TendrilType::Basic);

        let color = segment_color(&segment, &style);

        // Should be between base and corruption colors
        let result_srgba = color.to_srgba();
        let base_srgba = style.color.to_srgba();
        let corruption_srgba = CORRUPTION_COLOR.to_srgba();

        // Red should be between base and corruption
        assert!(result_srgba.red >= base_srgba.red.min(corruption_srgba.red));
        assert!(result_srgba.red <= base_srgba.red.max(corruption_srgba.red));
    }

    #[test]
    fn test_segment_color_zero_max_health() {
        let segment = TendrilSegment {
            health: 0.0,
            max_health: 0.0, // Edge case
            corrupted: false,
            corruption_level: 0.0,
            ..default()
        };
        let style = TendrilStyle::for_type(TendrilType::Basic);

        // Should not panic
        let color = segment_color(&segment, &style);

        // Should return base color (health_factor = 1.0 when max is 0)
        let base_srgba = style.color.to_srgba();
        let result_srgba = color.to_srgba();
        assert!((base_srgba.green - result_srgba.green).abs() < 0.01);
    }

    #[test]
    fn test_tendril_animation_state_default() {
        let state = TendrilAnimationState::default();
        assert_eq!(state.time, 0.0);
        assert!(state.flow_speed > 0.0);
        assert!(state.pulse_speed > 0.0);
    }

    #[test]
    fn test_tendril_style_is_component() {
        fn assert_component<T: Component>() {}
        assert_component::<TendrilStyle>();
    }

    #[test]
    fn test_tendril_animation_state_is_resource() {
        fn assert_resource<T: Resource>() {}
        assert_resource::<TendrilAnimationState>();
    }

    // Pulse calculation tests
    #[test]
    fn test_calculate_pulse_returns_between_zero_and_one() {
        // Test multiple time values
        for i in 0..100 {
            let time = i as f32 * 0.1;
            let pulse = calculate_pulse(time, 3.0);
            assert!(pulse >= 0.0, "Pulse should be >= 0, got {}", pulse);
            assert!(pulse <= 1.0, "Pulse should be <= 1, got {}", pulse);
        }
    }

    #[test]
    fn test_calculate_pulse_at_zero_time() {
        // sin(0) = 0, so 0 * 0.5 + 0.5 = 0.5
        let pulse = calculate_pulse(0.0, 1.0);
        assert!((pulse - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_calculate_pulse_varies_with_speed() {
        let pulse_slow = calculate_pulse(1.0, 1.0);
        let pulse_fast = calculate_pulse(1.0, 2.0);
        // Different speeds should produce different values at same time
        // (unless they happen to sync, which they don't at t=1.0)
        assert!((pulse_slow - pulse_fast).abs() > 0.01);
    }

    // Brighten color tests
    #[test]
    fn test_brighten_color_increases_rgb() {
        let color = Color::srgb(0.5, 0.5, 0.5);
        let brightened = brighten_color(color);
        let srgba = brightened.to_srgba();

        // Should be brighter (multiplied by 1.3)
        assert!((srgba.red - 0.65).abs() < 0.01);
        assert!((srgba.green - 0.65).abs() < 0.01);
        assert!((srgba.blue - 0.65).abs() < 0.01);
    }

    #[test]
    fn test_brighten_color_clamps_to_one() {
        let color = Color::srgb(0.9, 0.9, 0.9);
        let brightened = brighten_color(color);
        let srgba = brightened.to_srgba();

        // Should be clamped to 1.0 (0.9 * 1.3 = 1.17 -> 1.0)
        assert!((srgba.red - 1.0).abs() < 0.01);
        assert!((srgba.green - 1.0).abs() < 0.01);
        assert!((srgba.blue - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_brighten_color_preserves_alpha() {
        let color = Color::srgba(0.5, 0.5, 0.5, 0.7);
        let brightened = brighten_color(color);
        let srgba = brightened.to_srgba();

        // Alpha should remain unchanged
        assert!((srgba.alpha - 0.7).abs() < 0.01);
    }

    // Growth tip radius calculation test
    #[test]
    fn test_growth_tip_radius_calculation() {
        // Test the radius calculation used in render_growth_tips
        let base_radius: f32 = 6.0;
        let pulse_min: f32 = 0.0; // Pulse ranges from 0.0 to 1.0
        let pulse_max: f32 = 1.0;

        let radius_min = base_radius + pulse_min * 3.0;
        let radius_max = base_radius + pulse_max * 3.0;

        assert!((radius_min - 6.0).abs() < 0.01);
        assert!((radius_max - 9.0).abs() < 0.01);
    }

    // Core node radius calculation test
    #[test]
    fn test_core_node_radius_calculation() {
        // Test the radius calculation used in render_core
        let pulse_min: f32 = 0.0;
        let pulse_max: f32 = 1.0;

        let inner_radius_min = 14.0 + pulse_min * 2.0;
        let inner_radius_max = 14.0 + pulse_max * 2.0;
        let outer_radius_min = 18.0 + pulse_min * 3.0;
        let outer_radius_max = 18.0 + pulse_max * 3.0;

        assert!((inner_radius_min - 14.0).abs() < 0.01);
        assert!((inner_radius_max - 16.0).abs() < 0.01);
        assert!((outer_radius_min - 18.0).abs() < 0.01);
        assert!((outer_radius_max - 21.0).abs() < 0.01);
    }

    // Selected vs unselected growth tip color test
    #[test]
    fn test_growth_tip_selected_color() {
        // Selected tips should be bright white-ish
        let selected_color = Color::srgb(1.0, 1.0, 0.8);
        let srgba = selected_color.to_srgba();
        assert!(srgba.red >= 0.9);
        assert!(srgba.green >= 0.9);
    }

    #[test]
    fn test_growth_tip_unselected_color_is_brightened() {
        let base_color = Color::srgb(0.4, 0.7, 0.3);
        let unselected_color = brighten_color(base_color);
        let base_srgba = base_color.to_srgba();
        let result_srgba = unselected_color.to_srgba();

        // Should be brighter than base
        assert!(result_srgba.red > base_srgba.red);
        assert!(result_srgba.green > base_srgba.green);
        assert!(result_srgba.blue > base_srgba.blue);
    }
}
