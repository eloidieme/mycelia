//! Nutrient events
//!
//! Events for tracking nutrient gains and expenditures.

use bevy::prelude::*;

/// Source of nutrient income
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NutrientSource {
    /// Dropped by defeated enemies
    EnemyDrop,
    /// Collected from environment nodes
    EnvironmentNode,
    /// Passive generation from network
    PassiveAbsorption,
    /// From decomposing severed segments
    Decomposition,
    /// For testing/debugging
    Debug,
}

/// Purpose of nutrient expenditure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NutrientPurpose {
    /// Growing new tendril segments
    Growth,
    /// Using network abilities
    Ability,
    /// Cleansing corruption
    Cleanse,
    /// For testing/debugging
    Debug,
}

/// Event fired when nutrients are gained
#[derive(Event, Debug)]
pub struct NutrientsGained {
    /// Amount of nutrients gained
    pub amount: f32,
    /// Source of the nutrients
    pub source: NutrientSource,
}

impl NutrientsGained {
    /// Create a new NutrientsGained event
    #[must_use]
    pub fn new(amount: f32, source: NutrientSource) -> Self {
        Self { amount, source }
    }
}

/// Event fired when nutrients are successfully spent
#[derive(Event, Debug)]
pub struct NutrientsSpent {
    /// Amount of nutrients spent
    pub amount: f32,
    /// Purpose of the expenditure
    pub purpose: NutrientPurpose,
}

impl NutrientsSpent {
    /// Create a new NutrientsSpent event
    #[must_use]
    pub fn new(amount: f32, purpose: NutrientPurpose) -> Self {
        Self { amount, purpose }
    }
}

/// Event fired when a nutrient spend attempt fails due to insufficient resources
#[derive(Event, Debug)]
pub struct NutrientSpendFailed {
    /// Amount that was needed
    pub amount_needed: f32,
    /// Amount that was available
    pub amount_available: f32,
    /// Purpose of the failed spend
    pub purpose: NutrientPurpose,
}

impl NutrientSpendFailed {
    /// Create a new NutrientSpendFailed event
    #[must_use]
    pub fn new(amount_needed: f32, amount_available: f32, purpose: NutrientPurpose) -> Self {
        Self {
            amount_needed,
            amount_available,
            purpose,
        }
    }

    /// Calculate how many more nutrients are needed
    #[must_use]
    pub fn deficit(&self) -> f32 {
        self.amount_needed - self.amount_available
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // NutrientSource tests
    #[test]
    fn test_nutrient_source_variants() {
        // Just verify all variants exist and are distinct
        let sources = [
            NutrientSource::EnemyDrop,
            NutrientSource::EnvironmentNode,
            NutrientSource::PassiveAbsorption,
            NutrientSource::Decomposition,
            NutrientSource::Debug,
        ];
        assert_eq!(sources.len(), 5);
    }

    #[test]
    fn test_nutrient_source_equality() {
        assert_eq!(NutrientSource::EnemyDrop, NutrientSource::EnemyDrop);
        assert_ne!(NutrientSource::EnemyDrop, NutrientSource::Decomposition);
    }

    // NutrientPurpose tests
    #[test]
    fn test_nutrient_purpose_variants() {
        let purposes = [
            NutrientPurpose::Growth,
            NutrientPurpose::Ability,
            NutrientPurpose::Cleanse,
            NutrientPurpose::Debug,
        ];
        assert_eq!(purposes.len(), 4);
    }

    #[test]
    fn test_nutrient_purpose_equality() {
        assert_eq!(NutrientPurpose::Growth, NutrientPurpose::Growth);
        assert_ne!(NutrientPurpose::Growth, NutrientPurpose::Ability);
    }

    // NutrientsGained tests
    #[test]
    fn test_nutrients_gained_new() {
        let event = NutrientsGained::new(10.0, NutrientSource::EnemyDrop);
        assert_eq!(event.amount, 10.0);
        assert_eq!(event.source, NutrientSource::EnemyDrop);
    }

    #[test]
    fn test_nutrients_gained_is_event() {
        fn assert_event<T: Event>() {}
        assert_event::<NutrientsGained>();
    }

    // NutrientsSpent tests
    #[test]
    fn test_nutrients_spent_new() {
        let event = NutrientsSpent::new(5.0, NutrientPurpose::Growth);
        assert_eq!(event.amount, 5.0);
        assert_eq!(event.purpose, NutrientPurpose::Growth);
    }

    #[test]
    fn test_nutrients_spent_is_event() {
        fn assert_event<T: Event>() {}
        assert_event::<NutrientsSpent>();
    }

    // NutrientSpendFailed tests
    #[test]
    fn test_nutrient_spend_failed_new() {
        let event = NutrientSpendFailed::new(50.0, 20.0, NutrientPurpose::Cleanse);
        assert_eq!(event.amount_needed, 50.0);
        assert_eq!(event.amount_available, 20.0);
        assert_eq!(event.purpose, NutrientPurpose::Cleanse);
    }

    #[test]
    fn test_nutrient_spend_failed_deficit() {
        let event = NutrientSpendFailed::new(50.0, 20.0, NutrientPurpose::Cleanse);
        assert!((event.deficit() - 30.0).abs() < 0.001);
    }

    #[test]
    fn test_nutrient_spend_failed_is_event() {
        fn assert_event<T: Event>() {}
        assert_event::<NutrientSpendFailed>();
    }
}
