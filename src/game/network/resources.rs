//! Network resources

use bevy::prelude::*;

/// Reference to the core node entity for quick access
#[derive(Resource)]
pub struct CoreNodeEntity(pub Entity);

/// Currently active growth tip entity, if any
#[derive(Resource, Debug, Default)]
pub struct ActiveGrowthTip(pub Option<Entity>);

/// Tracks overall network statistics
#[derive(Resource, Debug, Default)]
pub struct NetworkStats {
    /// Total mass (health) of the network
    pub total_mass: f32,
    /// Maximum mass achieved
    pub max_mass: f32,
    /// Number of tendril segments
    pub segment_count: u32,
    /// Number of active growth tips
    pub tip_count: u32,
    /// Territory coverage percentage
    pub territory_coverage: f32,
    /// Number of segments connected to core
    pub connected_segments: u32,
    /// Number of severed segments
    pub severed_segments: u32,
}

/// Configuration for network behavior
#[derive(Resource, Debug)]
pub struct NetworkConfig {
    /// Length of each tendril segment in world units
    pub segment_length: f32,
    /// Default health for new segments
    pub segment_health: f32,
    /// Rate at which severed segments decay (HP per second)
    pub decay_rate: f32,
    /// Delay before severed segments start decaying (seconds)
    pub decay_start_delay: f32,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            segment_length: 16.0,
            segment_health: 50.0,
            decay_rate: 10.0,
            decay_start_delay: 2.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active_growth_tip_default() {
        let active_tip = ActiveGrowthTip::default();
        assert!(active_tip.0.is_none());
    }

    #[test]
    fn test_can_set_active_growth_tip() {
        let entity = Entity::from_raw(42);
        let active_tip = ActiveGrowthTip(Some(entity));
        assert_eq!(active_tip.0, Some(entity));
    }

    #[test]
    fn test_can_clear_active_growth_tip() {
        let mut active_tip = ActiveGrowthTip(Some(Entity::from_raw(42)));
        active_tip.0 = None;
        assert!(active_tip.0.is_none());
    }

    #[test]
    fn test_network_stats_default() {
        let stats = NetworkStats::default();
        assert_eq!(stats.total_mass, 0.0);
        assert_eq!(stats.segment_count, 0);
        assert_eq!(stats.tip_count, 0);
        assert_eq!(stats.connected_segments, 0);
        assert_eq!(stats.severed_segments, 0);
    }

    #[test]
    fn test_network_stats_is_resource() {
        fn assert_resource<T: Resource>() {}
        assert_resource::<NetworkStats>();
    }

    #[test]
    fn test_network_config_default() {
        let config = NetworkConfig::default();
        assert!(config.segment_length > 0.0);
        assert!(config.segment_health > 0.0);
        assert!(config.decay_rate > 0.0);
        assert!(config.decay_start_delay >= 0.0);
    }

    #[test]
    fn test_network_config_is_resource() {
        fn assert_resource<T: Resource>() {}
        assert_resource::<NetworkConfig>();
    }
}
