//! Progression system
//!
//! Handles the nutrient economy and upgrade systems:
//! - Nutrient resource tracking
//! - Passive nutrient generation
//! - Nutrient spending and events
//! - Territory milestone tracking (future)
//! - Upgrade selection (future)
//! - Meta-progression unlocks (future)

use bevy::prelude::*;

use crate::GameState;

pub mod events;
pub mod resources;
mod systems;

pub use events::*;
pub use resources::*;

/// Plugin for the progression system
pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<Nutrients>()
            .init_resource::<NutrientCosts>()
            .init_resource::<PassiveNutrientConfig>()
            // Events
            .add_event::<NutrientsGained>()
            .add_event::<NutrientsSpent>()
            .add_event::<NutrientSpendFailed>()
            // Systems - passive generation only in Playing state
            .add_systems(
                Update,
                systems::passive_nutrient_generation.run_if(in_state(GameState::Playing)),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::network::NetworkStats;
    use bevy::state::app::StatesPlugin;

    fn create_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(StatesPlugin)
            .init_state::<GameState>()
            .init_resource::<NetworkStats>()
            .add_plugins(ProgressionPlugin);
        app
    }

    #[test]
    fn test_progression_plugin_builds() {
        let mut app = create_test_app();
        app.update();
    }

    #[test]
    fn test_nutrients_resource_exists() {
        let mut app = create_test_app();
        app.update();

        assert!(app.world().get_resource::<Nutrients>().is_some());
    }

    #[test]
    fn test_nutrient_costs_resource_exists() {
        let mut app = create_test_app();
        app.update();

        assert!(app.world().get_resource::<NutrientCosts>().is_some());
    }

    #[test]
    fn test_passive_config_resource_exists() {
        let mut app = create_test_app();
        app.update();

        assert!(app
            .world()
            .get_resource::<PassiveNutrientConfig>()
            .is_some());
    }

    #[test]
    fn test_nutrients_default_values() {
        let mut app = create_test_app();
        app.update();

        let nutrients = app.world().resource::<Nutrients>();
        assert_eq!(nutrients.current, 50.0);
        assert_eq!(nutrients.max, 100.0);
    }

    #[test]
    fn test_passive_generation_only_in_playing() {
        let mut app = create_test_app();
        app.update();

        // In Menu state - nutrients should not increase
        app.world_mut()
            .resource_mut::<NetworkStats>()
            .connected_segments = 100;
        let initial = app.world().resource::<Nutrients>().current;

        app.update();

        let after_menu = app.world().resource::<Nutrients>().current;
        assert_eq!(
            initial, after_menu,
            "Nutrients should not change in Menu state"
        );

        // Transition to Playing
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        let after_playing = app.world().resource::<Nutrients>().current;
        assert!(
            after_playing > after_menu,
            "Nutrients should increase in Playing state"
        );
    }

    #[test]
    fn test_passive_generation_scales_with_segments() {
        let mut app = create_test_app();
        app.update();

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Playing);
        app.update();
        app.update();

        {
            let mut stats = app.world_mut().resource_mut::<NetworkStats>();
            stats.connected_segments = 10;
            stats.territory_coverage = 10.0;
        }
        app.world_mut().resource_mut::<Nutrients>().current = 50.0;

        for _ in 0..6000 {
            app.update();
        }

        let with_10 = app.world().resource::<Nutrients>().current;

        {
            let mut stats = app.world_mut().resource_mut::<NetworkStats>();
            stats.connected_segments = 200;
            stats.territory_coverage = 20.0;
        }
        app.world_mut().resource_mut::<Nutrients>().current = 50.0;

        for _ in 0..6000 {
            app.update();
        }

        let with_20 = app.world().resource::<Nutrients>().current;

        dbg!(&with_10);
        dbg!(&with_20);
        assert!(
            with_20 > with_10,
            "Expected 20 segments ({}) to generate more than 10 segments ({})",
            with_20,
            with_10
        );
    }
}
