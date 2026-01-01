//! Progression systems
//!
//! Systems for managing nutrient economy.

use bevy::prelude::*;

use super::events::{NutrientPurpose, NutrientSource, NutrientSpendFailed, NutrientsGained, NutrientsSpent};
use super::resources::{Nutrients, PassiveNutrientConfig};
use crate::game::network::NetworkStats;

/// Process passive nutrient generation based on network size
pub fn passive_nutrient_generation(
    time: Res<Time>,
    network_stats: Res<NetworkStats>,
    config: Res<PassiveNutrientConfig>,
    mut nutrients: ResMut<Nutrients>,
    mut events: EventWriter<NutrientsGained>,
) {
    // Calculate income from connected segments
    let segment_income = network_stats.connected_segments as f32 * config.per_segment_rate;

    // Calculate income from territory coverage
    let territory_income = network_stats.territory_coverage * config.territory_bonus_rate;

    // Scale by delta time
    let total = (segment_income + territory_income) * time.delta_secs();

    if total > 0.0 {
        nutrients.add(total);
        events.send(NutrientsGained::new(total, NutrientSource::PassiveAbsorption));
    }
}

/// Helper function to attempt spending nutrients with event firing
///
/// Returns true if the spend was successful, false otherwise.
/// Fires appropriate events based on the result.
#[allow(dead_code)] // Will be used by growth/ability systems in future tickets
pub fn try_spend_nutrients(
    amount: f32,
    purpose: NutrientPurpose,
    nutrients: &mut Nutrients,
    spent_events: &mut EventWriter<NutrientsSpent>,
    failed_events: &mut EventWriter<NutrientSpendFailed>,
) -> bool {
    let available = nutrients.current;
    if nutrients.spend(amount) {
        spent_events.send(NutrientsSpent::new(amount, purpose));
        true
    } else {
        failed_events.send(NutrientSpendFailed::new(amount, available, purpose));
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Unit tests for Nutrients::spend (doesn't require full app setup)
    #[test]
    fn test_spend_nutrients_success() {
        let mut nutrients = Nutrients::new(50.0, 100.0);
        let result = nutrients.spend(30.0);
        assert!(result);
        assert_eq!(nutrients.current, 20.0);
    }

    #[test]
    fn test_spend_nutrients_failure() {
        let mut nutrients = Nutrients::new(20.0, 100.0);
        let result = nutrients.spend(30.0);
        assert!(!result);
        assert_eq!(nutrients.current, 20.0); // Unchanged
    }

    // Note: Integration tests for passive_nutrient_generation are in mod.rs
    // as they require proper state machine setup
}
