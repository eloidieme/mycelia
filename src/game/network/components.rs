//! Network components

use bevy::prelude::*;

/// Marker component for the core node (lose condition if destroyed)
#[derive(Component, Debug, Default)]
pub struct CoreNode;

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

impl Default for NetworkVisuals {
    fn default() -> Self {
        Self {
            base_color: Color::srgb(0.4, 0.8, 0.4),       // Healthy green
            corruption_color: Color::srgb(0.6, 0.1, 0.4), // Corruption purple
        }
    }
}

/// A segment of the fungal network
#[derive(Component, Debug)]
pub struct TendrilSegment {
    /// Type of this tendril segment
    pub tendril_type: TendrilType,
    /// Health of this segment
    pub health: f32,
    /// Maximum health
    pub max_health: f32,
    /// Whether this segment is corrupted
    pub corrupted: bool,
    /// Corruption level (0.0 to 1.0)
    pub corruption_level: f32,
}

impl Default for TendrilSegment {
    fn default() -> Self {
        Self {
            tendril_type: TendrilType::Basic,
            health: 100.0,
            max_health: 100.0,
            corrupted: false,
            corruption_level: 0.0,
        }
    }
}

/// Types of tendrils with different properties
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TendrilType {
    #[default]
    Basic,
    Toxic,
    Sticky,
    Explosive,
}

/// Component marking the active growth tip
#[derive(Component, Debug, Default)]
pub struct GrowthTip {
    /// Whether this tip is currently selected by the player
    pub selected: bool,
}

/// Resource tracking total network statistics
#[derive(Resource, Debug, Default)]
pub struct NetworkStats {
    /// Total mass (health) of the network
    pub total_mass: f32,
    /// Maximum mass achieved
    pub max_mass: f32,
    /// Number of tendril segments
    pub segment_count: u32,
    /// Territory coverage percentage
    pub territory_coverage: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tendril_segment_default() {
        let segment = TendrilSegment::default();
        assert_eq!(segment.health, 100.0);
        assert_eq!(segment.tendril_type, TendrilType::Basic);
        assert!(!segment.corrupted);
    }

    #[test]
    fn test_tendril_type_default() {
        assert_eq!(TendrilType::default(), TendrilType::Basic);
    }

    // Health component tests
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

    // NetworkMember tests
    #[test]
    fn test_network_member_is_component() {
        fn assert_component<T: Component>() {}
        assert_component::<NetworkMember>();
    }

    // NetworkVisuals tests
    #[test]
    fn test_network_visuals_default() {
        let visuals = NetworkVisuals::default();
        // Just check it has some colors set
        assert_ne!(visuals.base_color, Color::NONE);
    }
}
