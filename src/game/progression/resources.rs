//! Progression resources
//!
//! Resources for tracking and configuring the nutrient economy.

use bevy::prelude::*;

/// Global nutrient pool - the primary currency for all player actions
#[derive(Resource, Debug)]
pub struct Nutrients {
    /// Current nutrient count
    pub current: f32,
    /// Maximum nutrients that can be stored
    pub max: f32,
}

impl Default for Nutrients {
    fn default() -> Self {
        Self {
            current: 50.0, // Start with some nutrients
            max: 100.0,
        }
    }
}

impl Nutrients {
    /// Create a new Nutrients resource with specified values
    #[must_use]
    pub fn new(current: f32, max: f32) -> Self {
        Self { current, max }
    }

    /// Add nutrients, capped at max
    pub fn add(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    /// Attempt to spend nutrients. Returns true if successful, false if insufficient.
    /// Does not deduct if insufficient.
    pub fn spend(&mut self, amount: f32) -> bool {
        if self.current >= amount {
            self.current -= amount;
            true
        } else {
            false
        }
    }

    /// Check if we can afford a given amount
    #[must_use]
    pub fn can_afford(&self, amount: f32) -> bool {
        self.current >= amount
    }

    /// Get current nutrients as a percentage of max (0.0 to 1.0)
    #[must_use]
    pub fn percentage(&self) -> f32 {
        if self.max > 0.0 {
            self.current / self.max
        } else {
            0.0
        }
    }

    /// Increase the maximum nutrient capacity
    pub fn increase_max(&mut self, amount: f32) {
        self.max += amount;
    }
}

/// Configuration for nutrient costs
#[derive(Resource, Debug)]
pub struct NutrientCosts {
    /// Cost to grow one tendril segment
    pub growth_cost: f32,
    /// Cost per second of network ability use
    pub ability_cost_per_second: f32,
    /// Base cost to cleanse corruption from a segment
    pub cleanse_base_cost: f32,
    /// Multiplier for cleanse cost based on corruption level
    pub cleanse_level_multiplier: f32,
}

impl Default for NutrientCosts {
    fn default() -> Self {
        Self {
            growth_cost: 5.0,
            ability_cost_per_second: 2.0,
            cleanse_base_cost: 10.0,
            cleanse_level_multiplier: 2.0,
        }
    }
}

impl NutrientCosts {
    /// Calculate the cost to cleanse a segment at a given corruption level
    #[must_use]
    pub fn cleanse_cost(&self, corruption_level: f32) -> f32 {
        self.cleanse_base_cost + (corruption_level * self.cleanse_level_multiplier)
    }
}

/// Passive nutrient generation configuration
#[derive(Resource, Debug)]
pub struct PassiveNutrientConfig {
    /// Nutrients per second per connected segment
    pub per_segment_rate: f32,
    /// Bonus nutrients per percentage of territory covered per second
    pub territory_bonus_rate: f32,
}

impl Default for PassiveNutrientConfig {
    fn default() -> Self {
        Self {
            per_segment_rate: 0.1,    // 0.1 per segment per second
            territory_bonus_rate: 1.0, // +1 per % territory per second
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Nutrients tests
    #[test]
    fn test_nutrients_default() {
        let nutrients = Nutrients::default();
        assert!(nutrients.current > 0.0);
        assert!(nutrients.max >= nutrients.current);
    }

    #[test]
    fn test_nutrients_new() {
        let nutrients = Nutrients::new(25.0, 75.0);
        assert_eq!(nutrients.current, 25.0);
        assert_eq!(nutrients.max, 75.0);
    }

    #[test]
    fn test_nutrients_add() {
        let mut nutrients = Nutrients::new(50.0, 100.0);
        nutrients.add(25.0);
        assert_eq!(nutrients.current, 75.0);
    }

    #[test]
    fn test_nutrients_add_caps_at_max() {
        let mut nutrients = Nutrients::new(80.0, 100.0);
        nutrients.add(50.0);
        assert_eq!(nutrients.current, 100.0);
    }

    #[test]
    fn test_nutrients_spend_success() {
        let mut nutrients = Nutrients::new(50.0, 100.0);
        let result = nutrients.spend(30.0);
        assert!(result);
        assert_eq!(nutrients.current, 20.0);
    }

    #[test]
    fn test_nutrients_spend_failure() {
        let mut nutrients = Nutrients::new(20.0, 100.0);
        let result = nutrients.spend(30.0);
        assert!(!result);
        assert_eq!(nutrients.current, 20.0); // Unchanged
    }

    #[test]
    fn test_nutrients_spend_exact_amount() {
        let mut nutrients = Nutrients::new(50.0, 100.0);
        let result = nutrients.spend(50.0);
        assert!(result);
        assert_eq!(nutrients.current, 0.0);
    }

    #[test]
    fn test_nutrients_can_afford() {
        let nutrients = Nutrients::new(50.0, 100.0);
        assert!(nutrients.can_afford(50.0));
        assert!(nutrients.can_afford(49.0));
        assert!(!nutrients.can_afford(51.0));
    }

    #[test]
    fn test_nutrients_can_afford_zero() {
        let nutrients = Nutrients::new(0.0, 100.0);
        assert!(nutrients.can_afford(0.0));
        assert!(!nutrients.can_afford(0.01));
    }

    #[test]
    fn test_nutrients_percentage() {
        let nutrients = Nutrients::new(25.0, 100.0);
        assert!((nutrients.percentage() - 0.25).abs() < 0.001);
    }

    #[test]
    fn test_nutrients_percentage_full() {
        let nutrients = Nutrients::new(100.0, 100.0);
        assert!((nutrients.percentage() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_nutrients_percentage_empty() {
        let nutrients = Nutrients::new(0.0, 100.0);
        assert!((nutrients.percentage() - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_nutrients_percentage_zero_max() {
        let nutrients = Nutrients::new(0.0, 0.0);
        assert_eq!(nutrients.percentage(), 0.0); // Should not panic
    }

    #[test]
    fn test_nutrients_increase_max() {
        let mut nutrients = Nutrients::new(50.0, 100.0);
        nutrients.increase_max(50.0);
        assert_eq!(nutrients.max, 150.0);
        assert_eq!(nutrients.current, 50.0); // Current unchanged
    }

    #[test]
    fn test_nutrients_is_resource() {
        fn assert_resource<T: Resource>() {}
        assert_resource::<Nutrients>();
    }

    // NutrientCosts tests
    #[test]
    fn test_nutrient_costs_default_positive() {
        let costs = NutrientCosts::default();
        assert!(costs.growth_cost > 0.0);
        assert!(costs.ability_cost_per_second > 0.0);
        assert!(costs.cleanse_base_cost > 0.0);
        assert!(costs.cleanse_level_multiplier > 0.0);
    }

    #[test]
    fn test_nutrient_costs_cleanse_cost() {
        let costs = NutrientCosts::default();
        // At 0% corruption: base cost only
        let cost_0 = costs.cleanse_cost(0.0);
        assert_eq!(cost_0, costs.cleanse_base_cost);

        // At 50% corruption: base + 0.5 * multiplier
        let cost_50 = costs.cleanse_cost(0.5);
        assert!((cost_50 - (costs.cleanse_base_cost + 0.5 * costs.cleanse_level_multiplier)).abs() < 0.001);

        // At 100% corruption: base + multiplier
        let cost_100 = costs.cleanse_cost(1.0);
        assert!((cost_100 - (costs.cleanse_base_cost + costs.cleanse_level_multiplier)).abs() < 0.001);
    }

    #[test]
    fn test_nutrient_costs_is_resource() {
        fn assert_resource<T: Resource>() {}
        assert_resource::<NutrientCosts>();
    }

    // PassiveNutrientConfig tests
    #[test]
    fn test_passive_config_default_positive() {
        let config = PassiveNutrientConfig::default();
        assert!(config.per_segment_rate > 0.0);
        assert!(config.territory_bonus_rate > 0.0);
    }

    #[test]
    fn test_passive_config_is_resource() {
        fn assert_resource<T: Resource>() {}
        assert_resource::<PassiveNutrientConfig>();
    }
}
