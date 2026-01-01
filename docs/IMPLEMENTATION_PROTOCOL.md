# Implementation Protocol

This document defines the standard workflow for implementing features and fixing bugs in Mycelia. All development follows Test-Driven Development (TDD) principles with a ticket-based workflow.

---

## Core Principles

1. **Tests First:** Write failing tests before implementation code
2. **Small Increments:** Implement in small, testable chunks
3. **Green Before Refactor:** Only refactor when tests pass
4. **One Ticket, One Branch:** Each ticket gets its own feature branch
5. **No Broken Main:** Master branch must always compile and pass tests

---

## Workflow Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        TICKET LIFECYCLE                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  1. PREPARE          2. RED            3. GREEN                 │
│  ┌──────────┐       ┌──────────┐      ┌──────────┐             │
│  │ Read     │       │ Write    │      │ Write    │             │
│  │ ticket   │──────▶│ failing  │─────▶│ minimal  │             │
│  │ Create   │       │ tests    │      │ code to  │             │
│  │ branch   │       │          │      │ pass     │             │
│  └──────────┘       └──────────┘      └──────────┘             │
│                                              │                  │
│                                              ▼                  │
│  5. COMPLETE         4. REFACTOR      ┌──────────┐             │
│  ┌──────────┐       ┌──────────┐      │ Tests    │             │
│  │ PR       │       │ Clean up │      │ pass?    │             │
│  │ Review   │◀──────│ code     │◀─────│          │             │
│  │ Merge    │       │ quality  │  YES └──────────┘             │
│  └──────────┘       └──────────┘             │ NO              │
│                                              │                  │
│                                              ▼                  │
│                                        (back to GREEN)          │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Phase 1: Prepare

### 1.1 Read the Ticket

Before writing any code:

- [ ] Read the full ticket description
- [ ] Understand the motivation and spec references
- [ ] Review the proposed implementation
- [ ] Study the test plan
- [ ] Check dependencies (ensure prerequisite tickets are complete)
- [ ] Review acceptance criteria

### 1.2 Create Feature Branch

```bash
# Format: <issue-number>-<short-description>
git checkout master
git pull origin master
git checkout -b 001-camera-system
```

### 1.3 Set Up File Structure

Create the files/directories outlined in the ticket:

```bash
# Example for camera system
mkdir -p src/game/camera
touch src/game/camera/mod.rs
touch src/game/camera/components.rs
touch src/game/camera/systems.rs
```

Add module declarations but leave implementations empty:

```rust
// src/game/camera/mod.rs
pub mod components;
pub mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, _app: &mut App) {
        // TODO: Implement
    }
}
```

### 1.4 Verify Clean State

```bash
cargo check   # Must compile
cargo test    # Existing tests must pass
```

---

## Phase 2: Red (Write Failing Tests)

### 2.1 Write Unit Tests First

Start with the simplest unit tests from the ticket's test plan:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_controller_default_values() {
        let controller = CameraController::default();
        assert_eq!(controller.zoom, 1.0);
        // This will fail - CameraController doesn't exist yet
    }
}
```

### 2.2 Verify Tests Fail

```bash
cargo test

# Expected output:
# error[E0433]: failed to resolve: use of undeclared type `CameraController`
```

**Important:** Tests must fail for the RIGHT reason:
- ✅ Type doesn't exist yet
- ✅ Method not implemented
- ✅ Wrong return value
- ❌ Syntax error in test
- ❌ Wrong import path

### 2.3 Write All Unit Tests

Write all unit tests from the ticket before implementing:

```rust
#[test]
fn test_zoom_clamps_to_bounds() { ... }

#[test]
fn test_camera_settings_resource() { ... }
```

### 2.4 Stub Integration Tests

For integration tests that need the full app:

```rust
#[test]
#[ignore] // Enable after unit tests pass
fn test_camera_spawns_on_startup() {
    let mut app = App::new();
    // ...
}
```

---

## Phase 3: Green (Make Tests Pass)

### 3.1 Implement Minimal Code

Write the **minimum code necessary** to make one test pass:

```rust
// Just enough to pass test_camera_controller_default_values
#[derive(Component, Debug)]
pub struct CameraController {
    pub zoom: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self { zoom: 1.0 }
    }
}
```

### 3.2 Run Tests Frequently

```bash
cargo test --lib  # Fast: only library tests

# After each small change:
# - One test should go from FAIL to PASS
# - No previously passing tests should break
```

### 3.3 One Test at a Time

Work through tests in order of complexity:

1. Default/constructor tests
2. Simple method tests
3. Edge case tests
4. Integration tests (un-ignore them one by one)

### 3.4 Track Progress

Update acceptance criteria as tests pass:

```markdown
## Acceptance Criteria

- [x] `CameraController` component tracks zoom level
- [x] Zoom clamps to min/max bounds
- [ ] Scroll wheel zooms in/out  <-- Working on this
- [ ] WASD/Arrow keys pan camera
```

---

## Phase 4: Refactor

### 4.1 Only Refactor When Green

**All tests must pass before refactoring.**

```bash
cargo test  # Must be green
```

### 4.2 Refactoring Checklist

- [ ] Remove code duplication
- [ ] Improve naming clarity
- [ ] Simplify complex logic
- [ ] Add documentation comments
- [ ] Ensure consistent formatting (`cargo fmt`)
- [ ] Address clippy warnings (`cargo clippy`)

### 4.3 Run Tests After Each Refactor

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

### 4.4 Performance Check

For systems that run every frame:

```bash
cargo run  # Visual check for smoothness
# Debug overlay (F3) should show 60+ FPS
```

---

## Phase 5: Complete

### 5.1 Final Verification

```bash
# Full test suite
cargo test

# Linting
cargo clippy -- -D warnings

# Formatting
cargo fmt --check

# Build check
cargo build --release
```

### 5.2 Self-Review Checklist

Before creating PR:

- [ ] All acceptance criteria marked complete
- [ ] All tests from ticket implemented and passing
- [ ] No `TODO` comments left (except documented future work)
- [ ] No `unwrap()` on fallible operations in non-test code
- [ ] No commented-out code
- [ ] Doc comments on public items
- [ ] No compiler warnings

### 5.3 Commit Guidelines

Use semantic commits:

```bash
# Feature implementation
git add -A
git commit -m "feat(camera): implement CameraController component

- Add CameraController with zoom tracking
- Add CameraSettings resource
- Implement zoom clamping

Closes #1"

# Test additions
git commit -m "test(camera): add unit tests for zoom behavior"

# Bug fixes
git commit -m "fix(camera): clamp zoom before applying to projection"
```

### 5.4 Create Pull Request

```bash
git push -u origin 001-camera-system

gh pr create --title "feat: Basic 2D camera with dynamic zoom foundation" \
  --body "## Summary
Implements #1

## Changes
- CameraController component
- CameraSettings resource
- Zoom and pan systems

## Test Plan
- All unit tests from ticket implemented
- Manual testing: zoom with scroll, pan with WASD

## Checklist
- [x] Tests pass
- [x] Clippy clean
- [x] Formatted

Closes #1"
```

### 5.5 Merge and Clean Up

After PR approval:

```bash
# Merge via GitHub UI or:
gh pr merge --squash

# Clean up local branch
git checkout master
git pull
git branch -d 001-camera-system
```

---

## TDD Patterns for Bevy

### Testing Components

```rust
#[test]
fn test_component_default() {
    let component = MyComponent::default();
    assert_eq!(component.field, expected_value);
}

#[test]
fn test_component_methods() {
    let mut component = MyComponent::new(10.0);
    component.modify(5.0);
    assert_eq!(component.value(), 15.0);
}
```

### Testing Resources

```rust
#[test]
fn test_resource_operations() {
    let mut resource = MyResource::default();
    resource.add(10.0);
    assert!(resource.can_afford(10.0));
    assert!(!resource.can_afford(11.0));
}
```

### Testing Systems (Integration)

```rust
#[test]
fn test_system_modifies_world() {
    // Minimal app setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, my_system);

    // Setup initial state
    let entity = app.world_mut().spawn(MyComponent::default()).id();

    // Run one update
    app.update();

    // Assert world changed correctly
    let component = app.world().get::<MyComponent>(entity).unwrap();
    assert_eq!(component.value, expected);
}
```

### Testing State Transitions

```rust
#[test]
fn test_state_transition() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .init_state::<GameState>()
       .add_systems(OnEnter(GameState::Playing), setup_system);

    // Start in Menu
    assert_eq!(*app.world().resource::<State<GameState>>(), GameState::Menu);

    // Transition to Playing
    app.world_mut().resource_mut::<NextState<GameState>>()
       .set(GameState::Playing);
    app.update();

    // Verify transition
    assert_eq!(*app.world().resource::<State<GameState>>(), GameState::Playing);
}
```

### Testing Events

```rust
#[test]
fn test_event_sent() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_event::<MyEvent>()
       .add_systems(Update, system_that_sends_event);

    app.update();

    let events = app.world().resource::<Events<MyEvent>>();
    let mut reader = events.get_reader();
    let event_count = reader.read(events).count();
    assert_eq!(event_count, 1);
}
```

---

## Common Pitfalls

### ❌ Don't: Write Implementation Before Tests

```rust
// BAD: Implementing without tests
pub fn calculate_damage(base: f32, multiplier: f32) -> f32 {
    base * multiplier
}
// No tests! How do we know this is correct?
```

### ✅ Do: Test First

```rust
#[test]
fn test_calculate_damage() {
    assert_eq!(calculate_damage(10.0, 1.5), 15.0);
}

#[test]
fn test_calculate_damage_zero_multiplier() {
    assert_eq!(calculate_damage(10.0, 0.0), 0.0);
}

// NOW implement
pub fn calculate_damage(base: f32, multiplier: f32) -> f32 {
    base * multiplier
}
```

### ❌ Don't: Write All Code Then All Tests

Large implementations without tests become hard to debug.

### ✅ Do: Small Red-Green Cycles

```
Write test_a → Implement for test_a → Pass
Write test_b → Implement for test_b → Pass
Write test_c → Implement for test_c → Pass
Refactor all
```

### ❌ Don't: Skip Edge Cases

```rust
// BAD: Only testing happy path
#[test]
fn test_health_damage() {
    let mut health = Health::new(100.0);
    health.damage(30.0);
    assert_eq!(health.current, 70.0);
}
```

### ✅ Do: Test Edge Cases

```rust
#[test]
fn test_health_damage_normal() { ... }

#[test]
fn test_health_damage_exceeds_current() {
    let mut health = Health::new(50.0);
    health.damage(100.0);
    assert_eq!(health.current, 0.0); // Doesn't go negative
}

#[test]
fn test_health_damage_zero() {
    let mut health = Health::new(100.0);
    health.damage(0.0);
    assert_eq!(health.current, 100.0);
}

#[test]
fn test_health_damage_negative_input() {
    // What should happen? Document and test it.
}
```

---

## Quick Reference

### Commands

```bash
# Development cycle
cargo check           # Fast compile check
cargo test --lib      # Run library tests only
cargo test            # Run all tests
cargo clippy          # Lint check
cargo fmt             # Format code

# Full verification
cargo test && cargo clippy -- -D warnings && cargo fmt --check

# Run game
cargo run
```

### Branch Naming

```
<issue-number>-<short-kebab-description>

Examples:
001-camera-system
002-game-state-machine
fix-camera-zoom-bounds
```

### Commit Prefixes

| Prefix | Usage |
|--------|-------|
| `feat:` | New feature |
| `fix:` | Bug fix |
| `test:` | Adding tests |
| `refactor:` | Code restructuring |
| `docs:` | Documentation |
| `chore:` | Build/tooling changes |

---

## Summary

1. **Read ticket thoroughly** before starting
2. **Create feature branch** from master
3. **Write failing tests** first (Red)
4. **Implement minimal code** to pass tests (Green)
5. **Refactor** only when green
6. **Create PR** with all acceptance criteria met
7. **Merge** and clean up

The goal is sustainable development with confidence. Tests catch regressions, small increments reduce debugging time, and clear protocols ensure consistency.
