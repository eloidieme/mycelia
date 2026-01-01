//! Graph traversal utilities for the fungal network
//!
//! Provides functions to traverse and query the network graph structure.

use bevy::prelude::*;

use super::components::{NetworkChildren, NetworkParent};

/// Check if a segment is connected to the core node
///
/// Traverses up the parent chain to see if it eventually reaches the core.
pub fn is_connected_to_core(entity: Entity, parents: &Query<&NetworkParent>, core: Entity) -> bool {
    let mut current = entity;
    loop {
        if current == core {
            return true;
        }
        match parents.get(current) {
            Ok(parent) => current = parent.0,
            Err(_) => return false,
        }
    }
}

/// Find all segments downstream from a given segment (including itself)
///
/// This includes all children, grandchildren, etc. Useful for finding
/// what would be severed if this segment is destroyed.
pub fn find_downstream_segments(entity: Entity, children: &Query<&NetworkChildren>) -> Vec<Entity> {
    let mut result = Vec::new();
    let mut stack = vec![entity];

    while let Some(current) = stack.pop() {
        result.push(current);
        if let Ok(kids) = children.get(current) {
            stack.extend(kids.0.iter());
        }
    }

    result
}

/// Calculate hop distance from core node
///
/// Returns None if the entity is not connected to the core.
pub fn distance_from_core(
    entity: Entity,
    parents: &Query<&NetworkParent>,
    core: Entity,
) -> Option<u32> {
    let mut current = entity;
    let mut distance = 0;

    loop {
        if current == core {
            return Some(distance);
        }
        match parents.get(current) {
            Ok(parent) => {
                current = parent.0;
                distance += 1;
            }
            Err(_) => return None,
        }
    }
}

/// Find the path from an entity to the core
///
/// Returns the list of entities from the given entity to the core (inclusive).
/// Returns None if not connected to core.
pub fn path_to_core(
    entity: Entity,
    parents: &Query<&NetworkParent>,
    core: Entity,
) -> Option<Vec<Entity>> {
    let mut path = Vec::new();
    let mut current = entity;

    loop {
        path.push(current);
        if current == core {
            return Some(path);
        }
        match parents.get(current) {
            Ok(parent) => current = parent.0,
            Err(_) => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a minimal test app
    fn create_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app
    }

    #[test]
    fn test_is_connected_to_core_direct() {
        let mut app = create_test_app();

        // Spawn core (no parent)
        let core = app.world_mut().spawn_empty().id();

        // Spawn segment with core as parent
        let segment = app.world_mut().spawn(NetworkParent(core)).id();

        app.update();

        assert!(is_connected_helper(&app, segment, core));
    }

    #[test]
    fn test_is_connected_to_core_chain() {
        let mut app = create_test_app();

        // Core -> A -> B -> C
        let core = app.world_mut().spawn_empty().id();
        let a = app.world_mut().spawn(NetworkParent(core)).id();
        let b = app.world_mut().spawn(NetworkParent(a)).id();
        let c = app.world_mut().spawn(NetworkParent(b)).id();

        app.update();

        assert!(is_connected_helper(&app, c, core));
        assert!(is_connected_helper(&app, b, core));
        assert!(is_connected_helper(&app, a, core));
    }

    #[test]
    fn test_is_connected_to_core_not_connected() {
        let mut app = create_test_app();

        let core = app.world_mut().spawn_empty().id();
        // Orphan segment with no parent
        let orphan = app.world_mut().spawn_empty().id();

        app.update();

        assert!(!is_connected_helper(&app, orphan, core));
    }

    #[test]
    fn test_is_connected_to_core_wrong_tree() {
        let mut app = create_test_app();

        let core = app.world_mut().spawn_empty().id();
        let other_root = app.world_mut().spawn_empty().id();
        let segment = app.world_mut().spawn(NetworkParent(other_root)).id();

        app.update();

        assert!(!is_connected_helper(&app, segment, core));
    }

    #[test]
    fn test_find_downstream_segments_single() {
        let mut app = create_test_app();

        let segment = app.world_mut().spawn(NetworkChildren::default()).id();

        app.update();

        let result = find_downstream_helper(&app, segment);
        assert_eq!(result.len(), 1);
        assert!(result.contains(&segment));
    }

    #[test]
    fn test_find_downstream_segments_tree() {
        let mut app = create_test_app();

        // Build: A -> B, C
        //        B -> D
        let d = app.world_mut().spawn(NetworkChildren::default()).id();
        let c = app.world_mut().spawn(NetworkChildren::default()).id();
        let b = app.world_mut().spawn(NetworkChildren(vec![d])).id();
        let a = app.world_mut().spawn(NetworkChildren(vec![b, c])).id();

        app.update();

        let result = find_downstream_helper(&app, a);
        assert_eq!(result.len(), 4);
        assert!(result.contains(&a));
        assert!(result.contains(&b));
        assert!(result.contains(&c));
        assert!(result.contains(&d));
    }

    #[test]
    fn test_distance_from_core_direct() {
        let mut app = create_test_app();

        let core = app.world_mut().spawn_empty().id();
        let segment = app.world_mut().spawn(NetworkParent(core)).id();

        app.update();

        assert_eq!(distance_helper(&app, segment, core), Some(1));
    }

    #[test]
    fn test_distance_from_core_chain() {
        let mut app = create_test_app();

        let core = app.world_mut().spawn_empty().id();
        let a = app.world_mut().spawn(NetworkParent(core)).id();
        let b = app.world_mut().spawn(NetworkParent(a)).id();
        let c = app.world_mut().spawn(NetworkParent(b)).id();

        app.update();

        assert_eq!(distance_helper(&app, core, core), Some(0));
        assert_eq!(distance_helper(&app, a, core), Some(1));
        assert_eq!(distance_helper(&app, b, core), Some(2));
        assert_eq!(distance_helper(&app, c, core), Some(3));
    }

    #[test]
    fn test_distance_from_core_not_connected() {
        let mut app = create_test_app();

        let core = app.world_mut().spawn_empty().id();
        let orphan = app.world_mut().spawn_empty().id();

        app.update();

        assert_eq!(distance_helper(&app, orphan, core), None);
    }

    #[test]
    fn test_path_to_core() {
        let mut app = create_test_app();

        let core = app.world_mut().spawn_empty().id();
        let a = app.world_mut().spawn(NetworkParent(core)).id();
        let b = app.world_mut().spawn(NetworkParent(a)).id();

        app.update();

        let path = path_to_core_helper(&app, b, core);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path, vec![b, a, core]);
    }

    #[test]
    fn test_path_to_core_not_connected() {
        let mut app = create_test_app();

        let core = app.world_mut().spawn_empty().id();
        let orphan = app.world_mut().spawn_empty().id();

        app.update();

        assert!(path_to_core_helper(&app, orphan, core).is_none());
    }

    // Helper functions for tests that need query access
    fn is_connected_helper(app: &App, entity: Entity, core: Entity) -> bool {
        let mut current = entity;
        loop {
            if current == core {
                return true;
            }
            match app.world().get::<NetworkParent>(current) {
                Some(parent) => current = parent.0,
                None => return false,
            }
        }
    }

    fn find_downstream_helper(app: &App, entity: Entity) -> Vec<Entity> {
        let mut result = Vec::new();
        let mut stack = vec![entity];

        while let Some(current) = stack.pop() {
            result.push(current);
            if let Some(kids) = app.world().get::<NetworkChildren>(current) {
                stack.extend(kids.0.iter());
            }
        }

        result
    }

    fn distance_helper(app: &App, entity: Entity, core: Entity) -> Option<u32> {
        let mut current = entity;
        let mut distance = 0;

        loop {
            if current == core {
                return Some(distance);
            }
            match app.world().get::<NetworkParent>(current) {
                Some(parent) => {
                    current = parent.0;
                    distance += 1;
                }
                None => return None,
            }
        }
    }

    fn path_to_core_helper(app: &App, entity: Entity, core: Entity) -> Option<Vec<Entity>> {
        let mut path = Vec::new();
        let mut current = entity;

        loop {
            path.push(current);
            if current == core {
                return Some(path);
            }
            match app.world().get::<NetworkParent>(current) {
                Some(parent) => current = parent.0,
                None => return None,
            }
        }
    }
}
