//! Network components

use bevy::prelude::*;

/// Marker component for the core node (lose condition if destroyed)
#[derive(Component, Debug, Default)]
pub struct CoreNode;

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
}
