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
        if self.max > 0.0 {
            self.current / self.max
        } else {
            0.0
        }
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

/// Position and orientation of a tendril segment in world space
#[derive(Component, Debug, Clone)]
pub struct TendrilPosition {
    /// World position of this segment
    pub position: Vec2,
    /// Direction this segment is facing (normalized)
    pub direction: Vec2,
}

impl TendrilPosition {
    pub fn new(position: Vec2, direction: Vec2) -> Self {
        Self {
            position,
            direction: direction.normalize_or_zero(),
        }
    }
}

impl Default for TendrilPosition {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            direction: Vec2::X, // Default facing right
        }
    }
}

/// Connection to parent segment (toward core)
#[derive(Component, Debug)]
pub struct NetworkParent(pub Entity);

/// Connection to child segments (away from core)
#[derive(Component, Debug, Default, Clone)]
pub struct NetworkChildren(pub Vec<Entity>);

impl NetworkChildren {
    pub fn add_child(&mut self, child: Entity) {
        self.0.push(child);
    }

    pub fn remove_child(&mut self, child: Entity) {
        self.0.retain(|&e| e != child);
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

/// Marker for segments disconnected from core (will decay)
#[derive(Component, Debug)]
pub struct Severed {
    /// Time since this segment was severed
    pub time_since_severance: f32,
    /// Rate at which health decays (HP per second)
    pub decay_rate: f32,
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

    #[test]
    fn test_health_percentage_zero_max() {
        let health = Health {
            current: 0.0,
            max: 0.0,
        };
        assert_eq!(health.percentage(), 0.0); // Should not panic
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

    // TendrilPosition tests
    #[test]
    fn test_tendril_position_default() {
        let pos = TendrilPosition::default();
        assert_eq!(pos.position, Vec2::ZERO);
        assert_eq!(pos.direction, Vec2::X);
    }

    #[test]
    fn test_tendril_position_new_normalizes_direction() {
        let pos = TendrilPosition::new(Vec2::new(10.0, 20.0), Vec2::new(3.0, 4.0));
        assert_eq!(pos.position, Vec2::new(10.0, 20.0));
        // 3-4-5 triangle, normalized should be (0.6, 0.8)
        assert!((pos.direction.x - 0.6).abs() < 0.001);
        assert!((pos.direction.y - 0.8).abs() < 0.001);
        assert!((pos.direction.length() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_tendril_position_new_handles_zero_direction() {
        let pos = TendrilPosition::new(Vec2::ZERO, Vec2::ZERO);
        assert_eq!(pos.direction, Vec2::ZERO);
    }

    #[test]
    fn test_tendril_position_is_component() {
        fn assert_component<T: Component>() {}
        assert_component::<TendrilPosition>();
    }

    // NetworkParent tests
    #[test]
    fn test_network_parent_is_component() {
        fn assert_component<T: Component>() {}
        assert_component::<NetworkParent>();
    }

    // NetworkChildren tests
    #[test]
    fn test_network_children_default_empty() {
        let children = NetworkChildren::default();
        assert!(children.is_empty());
        assert_eq!(children.len(), 0);
    }

    #[test]
    fn test_network_children_add_child() {
        let mut children = NetworkChildren::default();
        let entity = Entity::from_raw(42);
        children.add_child(entity);
        assert!(!children.is_empty());
        assert_eq!(children.len(), 1);
        assert!(children.0.contains(&entity));
    }

    #[test]
    fn test_network_children_remove_child() {
        let entity1 = Entity::from_raw(1);
        let entity2 = Entity::from_raw(2);
        let mut children = NetworkChildren(vec![entity1, entity2]);

        children.remove_child(entity1);
        assert_eq!(children.len(), 1);
        assert!(!children.0.contains(&entity1));
        assert!(children.0.contains(&entity2));
    }

    #[test]
    fn test_network_children_is_component() {
        fn assert_component<T: Component>() {}
        assert_component::<NetworkChildren>();
    }

    // GrowthTip tests
    #[test]
    fn test_growth_tip_default_not_selected() {
        let tip = GrowthTip::default();
        assert!(!tip.selected);
    }

    #[test]
    fn test_growth_tip_is_component() {
        fn assert_component<T: Component>() {}
        assert_component::<GrowthTip>();
    }

    // Severed tests
    #[test]
    fn test_severed_is_component() {
        fn assert_component<T: Component>() {}
        assert_component::<Severed>();
    }

    #[test]
    fn test_severed_fields() {
        let severed = Severed {
            time_since_severance: 1.5,
            decay_rate: 10.0,
        };
        assert_eq!(severed.time_since_severance, 1.5);
        assert_eq!(severed.decay_rate, 10.0);
    }
}
