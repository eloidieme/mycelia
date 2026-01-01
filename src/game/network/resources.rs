//! Network resources

use bevy::prelude::*;

/// Reference to the core node entity for quick access
#[derive(Resource)]
pub struct CoreNodeEntity(pub Entity);
