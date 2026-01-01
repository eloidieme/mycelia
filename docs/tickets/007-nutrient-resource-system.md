# [FEATURE] Nutrient Resource System

## Summary

Implement the nutrient/energy resource system that powers network growth and abilities. Nutrients are the primary currency for all player actions.

## Motivation

Nutrients are central to Mycelia's economy:
- **Growth cost:** Extending tendrils costs nutrients
- **Ability cost:** Network abilities consume nutrients
- **Corruption cure:** Cleansing corruption costs nutrients
- **Strategic tension:** Balancing growth vs reserves

This ticket establishes the resource tracking; collection mechanics come later.

## Spec Reference

From `spec.md`:
- **Primary Resource:** Nutrients/Energy - gathered and spent to extend
- **Sources:** Enemy drops, environmental nodes, passive absorption, decomposition
- **Curable with Cost:** Spend significant resources to cleanse corruption
- **Collection Method:** Magnetic pull - resources drift toward nearest tendril

## Proposed Implementation

### Resources

```rust
// src/game/progression/resources.rs

/// Global nutrient pool
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
            current: 50.0,  // Start with some nutrients
            max: 100.0,
        }
    }
}

impl Nutrients {
    pub fn add(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn spend(&mut self, amount: f32) -> bool {
        if self.current >= amount {
            self.current -= amount;
            true
        } else {
            false
        }
    }

    pub fn can_afford(&self, amount: f32) -> bool {
        self.current >= amount
    }

    pub fn percentage(&self) -> f32 {
        self.current / self.max
    }

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

/// Passive nutrient generation configuration
#[derive(Resource, Debug)]
pub struct PassiveNutrientConfig {
    /// Nutrients per second per connected segment
    pub per_segment_rate: f32,
    /// Bonus for territory coverage
    pub territory_bonus_rate: f32,
}

impl Default for PassiveNutrientConfig {
    fn default() -> Self {
        Self {
            per_segment_rate: 0.1,   // 0.1 per segment per second
            territory_bonus_rate: 1.0, // +1 per % territory per second
        }
    }
}
```

### Events

```rust
// src/game/progression/events.rs

/// Event when nutrients are gained
#[derive(Event, Debug)]
pub struct NutrientsGained {
    pub amount: f32,
    pub source: NutrientSource,
}

/// Event when nutrients are spent
#[derive(Event, Debug)]
pub struct NutrientsSpent {
    pub amount: f32,
    pub purpose: NutrientPurpose,
}

/// Event when a spend attempt fails (insufficient nutrients)
#[derive(Event, Debug)]
pub struct NutrientSpendFailed {
    pub amount_needed: f32,
    pub amount_available: f32,
    pub purpose: NutrientPurpose,
}

#[derive(Debug, Clone, Copy)]
pub enum NutrientSource {
    EnemyDrop,
    EnvironmentNode,
    PassiveAbsorption,
    Decomposition,
    Debug, // For testing
}

#[derive(Debug, Clone, Copy)]
pub enum NutrientPurpose {
    Growth,
    Ability,
    Cleanse,
    Debug, // For testing
}
```

### Systems

```rust
// src/game/progression/systems.rs

/// Process passive nutrient generation
pub fn passive_nutrient_generation(
    time: Res<Time>,
    network_stats: Res<NetworkStats>,
    config: Res<PassiveNutrientConfig>,
    mut nutrients: ResMut<Nutrients>,
    mut events: EventWriter<NutrientsGained>,
) {
    let segment_income = network_stats.connected_segments as f32 * config.per_segment_rate;
    let territory_income = network_stats.territory_coverage * config.territory_bonus_rate;
    let total = (segment_income + territory_income) * time.delta_secs();

    if total > 0.0 {
        nutrients.add(total);
        events.send(NutrientsGained {
            amount: total,
            source: NutrientSource::PassiveAbsorption,
        });
    }
}

/// Validate and process nutrient spending
pub fn try_spend_nutrients(
    amount: f32,
    purpose: NutrientPurpose,
    nutrients: &mut Nutrients,
    gained_events: &mut EventWriter<NutrientsSpent>,
    failed_events: &mut EventWriter<NutrientSpendFailed>,
) -> bool {
    if nutrients.spend(amount) {
        gained_events.send(NutrientsSpent { amount, purpose });
        true
    } else {
        failed_events.send(NutrientSpendFailed {
            amount_needed: amount,
            amount_available: nutrients.current,
            purpose,
        });
        false
    }
}
```

### File Structure

```
src/game/progression/
├── mod.rs          # Plugin registration
├── resources.rs    # Nutrients, NutrientCosts, PassiveNutrientConfig
├── events.rs       # Nutrient events
└── systems.rs      # Passive generation, spend validation
```

## Test Plan (TDD)

### Unit Tests

```rust
#[test]
fn test_nutrients_default() {
    let nutrients = Nutrients::default();
    assert!(nutrients.current > 0.0);
    assert!(nutrients.max > nutrients.current);
}

#[test]
fn test_nutrients_add() {
    let mut nutrients = Nutrients { current: 50.0, max: 100.0 };
    nutrients.add(25.0);
    assert_eq!(nutrients.current, 75.0);
}

#[test]
fn test_nutrients_add_caps_at_max() {
    let mut nutrients = Nutrients { current: 80.0, max: 100.0 };
    nutrients.add(50.0);
    assert_eq!(nutrients.current, 100.0);
}

#[test]
fn test_nutrients_spend_success() {
    let mut nutrients = Nutrients { current: 50.0, max: 100.0 };
    let result = nutrients.spend(30.0);
    assert!(result);
    assert_eq!(nutrients.current, 20.0);
}

#[test]
fn test_nutrients_spend_failure() {
    let mut nutrients = Nutrients { current: 20.0, max: 100.0 };
    let result = nutrients.spend(30.0);
    assert!(!result);
    assert_eq!(nutrients.current, 20.0); // Unchanged
}

#[test]
fn test_nutrients_can_afford() {
    let nutrients = Nutrients { current: 50.0, max: 100.0 };
    assert!(nutrients.can_afford(50.0));
    assert!(nutrients.can_afford(49.0));
    assert!(!nutrients.can_afford(51.0));
}

#[test]
fn test_nutrients_percentage() {
    let nutrients = Nutrients { current: 25.0, max: 100.0 };
    assert_eq!(nutrients.percentage(), 0.25);
}

#[test]
fn test_nutrients_increase_max() {
    let mut nutrients = Nutrients { current: 50.0, max: 100.0 };
    nutrients.increase_max(50.0);
    assert_eq!(nutrients.max, 150.0);
    assert_eq!(nutrients.current, 50.0); // Current unchanged
}

#[test]
fn test_nutrient_costs_default_positive() {
    let costs = NutrientCosts::default();
    assert!(costs.growth_cost > 0.0);
    assert!(costs.ability_cost_per_second > 0.0);
}
```

### Integration Tests

```rust
#[test]
fn test_passive_generation_increases_nutrients() {
    let mut app = App::new();
    // Setup with ProgressionPlugin
    // Set network_stats.connected_segments = 10

    let initial = app.world.resource::<Nutrients>().current;

    // Advance time
    app.update();

    let final_nutrients = app.world.resource::<Nutrients>().current;
    assert!(final_nutrients > initial);
}

#[test]
fn test_nutrients_gained_event_fired() {
    let mut app = App::new();
    // Setup

    app.update();

    // Check NutrientsGained events
    let events = app.world.resource::<Events<NutrientsGained>>();
    assert!(!events.is_empty());
}

#[test]
fn test_spend_failure_fires_event() {
    let mut app = App::new();
    // Setup with low nutrients
    // Attempt to spend more than available

    // Check NutrientSpendFailed event fired
}
```

## Acceptance Criteria

- [ ] `Nutrients` resource tracks current and max nutrients
- [ ] `add()` respects max cap
- [ ] `spend()` returns false and doesn't deduct if insufficient
- [ ] `can_afford()` correctly checks availability
- [ ] `NutrientCosts` defines all cost values
- [ ] `PassiveNutrientConfig` defines generation rates
- [ ] Passive generation runs each frame in Playing state
- [ ] `NutrientsGained` event fires on nutrient acquisition
- [ ] `NutrientsSpent` event fires on successful spend
- [ ] `NutrientSpendFailed` event fires on failed spend attempt
- [ ] All unit tests pass
- [ ] All integration tests pass

## Dependencies

- **Ticket #2:** Game state machine (generation only in Playing state)
- **Ticket #5:** Network stats (for passive generation calculation)

## Out of Scope

- Nutrient pickup entities (future ticket)
- Magnetic pull collection (future ticket)
- Decomposition timing (future ticket)
- UI display of nutrients (Ticket #8 or future)
- Upgrade that increases max nutrients
