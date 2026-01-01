# Mycelia - Project Context for Claude

## Project Overview

Mycelia is a Vampire Survivors-inspired roguelite built with Rust and Bevy. The player controls a spreading fungal network instead of a single character. The core fantasy is **ecosystem mastery** - becoming the apex predator of the underground world.

**Repository:** https://github.com/eloidieme/mycelia

## Tech Stack

- **Language:** Rust (edition 2021)
- **Engine:** Bevy 0.15
- **Build:** Cargo with dynamic linking for dev builds

## Development Workflow

### TDD Approach

This project follows Test-Driven Development:

1. Write failing tests first
2. Implement minimal code to pass tests
3. Refactor while keeping tests green

### Ticket-Based Workflow

All work is tracked through GitHub issues using templates:

- **Bug reports:** `.github/ISSUE_TEMPLATE/bug_report.md`
- **Features:** `.github/ISSUE_TEMPLATE/feature_request.md`

Each issue should include:

- Clear acceptance criteria
- Test plan defined before implementation
- Regression tests for bugs

### Commands

```bash
# Check project compiles
cargo check

# Run all tests
cargo test

# Run with dynamic linking (faster iteration)
cargo run

# Run clippy for linting
cargo clippy

# Format code
cargo fmt
```

## Project Structure

```
src/
├── main.rs           # Entry point, window setup
├── lib.rs            # Library root, GamePlugin, GameState
└── game/
    ├── mod.rs        # GameSystemsPlugin aggregating all systems
    ├── network/      # Fungal network (tendrils, growth, severance)
    ├── combat/       # Spore attacks, damage, abilities
    ├── enemies/      # Enemy AI, spawning, corruption
    ├── progression/  # Upgrades, milestones, meta-progression
    ├── map/          # Procedural generation, biomes, fog
    ├── ui/           # HUD, menus, upgrade selection
    └── camera/       # Dynamic zoom, panning, minimap
```

## Core Game Concepts

### The Network

- Player grows tendrils by directing active growth tips with mouse
- Tendrils can be specialized: Basic, Toxic, Sticky, Explosive
- Adjacent different types create synergy bonuses
- Network can be severed; cut sections decay slowly

### Combat

- Tendrils auto-attack with spore releases on cooldowns
- Network abilities: heal, speed boost, retract
- Enemies can corrupt the network (real-time spread)
- Corruption curable with nutrient cost

### Enemies

Three categories with unique mechanics:

- **Insects:** Fast swarms, melee attacks
- **Fungi:** Compete for territory, corruption source, bosses
- **Bacteria:** Infect and enable corruption

### Progression

- Territory milestones trigger upgrade choices (3 random options)
- Nutrients gathered from: enemy drops, environment, passive, decomposition
- Meta-progression unlocks new tendril types, abilities, strains
- Difficulty scales with network size (territory-based)

### Win/Lose

- **Win:** Ecosystem collapse (eliminate all bosses at fixed map locations)
- **Lose:** Corruption reaches core node

## Current State

**Phase:** Playable prototype in development

**Prototype scope:**

- 2-3 tendril types
- 3-4 enemy types (one per category)
- 1 boss encounter
- Core loop: grow, kill, survive

## Key Design Decisions

1. **60 FPS minimum** - Performance is non-negotiable
2. **Pixel art with animated tendril lines** - Organic feel within retro aesthetic
3. **Multiplayer-aware architecture** - Design for future co-op without rewrite
4. **True pause** - Everything freezes during upgrade selection

## Testing Guidelines

- Unit tests go in `mod.rs` or dedicated `tests.rs` within each module
- Use `#[cfg(test)]` modules
- Test components and systems in isolation where possible
- For Bevy systems, use `App::new()` with minimal plugins for integration tests

## Code Conventions

- Follow Rust naming conventions
- Use Bevy's `Component`, `Resource`, `Event` markers appropriately
- Prefer composition over inheritance (ECS-native patterns)
- Keep systems focused and single-purpose
- Document public APIs with doc comments
- use context7 if you needs docs or reference (bevy or other crates)

## Spec Reference

Full game design specification is in `spec.md`. Always consult it for:

- Detailed mechanic descriptions
- UI/UX requirements
- Prototype scope boundaries

## Reviewing handling guidelines

When you address comments on your PRs, only make changes for comments you deem relevant, implement the changes and explain them briefly as a response to the comment on GitHub. If you decided it's not relevant, explain briefly your reasoning as a comment response.
