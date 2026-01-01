//! State components and resources

use bevy::prelude::*;

/// Resource tracking run statistics
#[derive(Resource, Debug, Default, Clone)]
pub struct RunStats {
    /// Time elapsed in current run (seconds)
    pub elapsed_time: f32,
    /// Number of enemies killed this run
    pub enemies_killed: u32,
    /// Maximum territory coverage achieved
    pub max_territory: f32,
    /// Total nutrients collected this run
    pub nutrients_collected: f32,
}

impl RunStats {
    /// Reset all stats to zero for a new run
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Resource tracking pause state details
#[derive(Resource, Debug, Default)]
pub struct PauseState {
    /// Whether the pause was triggered by upgrade selection
    pub was_paused_by_upgrade: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_stats_default() {
        let stats = RunStats::default();
        assert_eq!(stats.elapsed_time, 0.0);
        assert_eq!(stats.enemies_killed, 0);
        assert_eq!(stats.max_territory, 0.0);
        assert_eq!(stats.nutrients_collected, 0.0);
    }

    #[test]
    fn test_run_stats_is_resource() {
        fn assert_resource<T: Resource>() {}
        assert_resource::<RunStats>();
    }

    #[test]
    fn test_run_stats_reset() {
        let mut stats = RunStats {
            elapsed_time: 100.0,
            enemies_killed: 50,
            max_territory: 0.5,
            nutrients_collected: 1000.0,
        };
        stats.reset();
        assert_eq!(stats.elapsed_time, 0.0);
        assert_eq!(stats.enemies_killed, 0);
    }

    #[test]
    fn test_pause_state_default() {
        let pause = PauseState::default();
        assert!(!pause.was_paused_by_upgrade);
    }

    #[test]
    fn test_pause_state_is_resource() {
        fn assert_resource<T: Resource>() {}
        assert_resource::<PauseState>();
    }
}
