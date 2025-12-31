# Mycelia - Game Design Specification

## Overview

Mycelia is a Vampire Survivors-inspired roguelite where you control a fungal spore network spreading through a forest floor. Instead of moving a single character, you grow by extending tendrils in directions you choose. The core fantasy is **ecosystem mastery** - becoming the apex predator of the underground world.

**Target Platform:** PC (Mouse + Keyboard, Gamepad)
**Engine:** Bevy (Rust)
**Art Style:** Pixel art with continuous animated tendril growth
**Run Length:** 15-20 minutes
**Pacing:** Frantic, VS-style constant pressure

---

## Core Mechanics

### Growth & Movement

- **Directional + Organic Growth:** Player picks a direction, tendrils pathfind with procedural shaping and some organic randomness
- **Mouse-from-Tip Control:** The active tendril tip chases the mouse cursor position
- **Explicit Tip Selection:** Click on a tendril tip to select it as the active growth point
- **Free Overlap:** Tendrils can layer and cross freely, allowing dense network formations
- **Retract Ability:** Can pull back tendrils to create new growth directions if blocked

### Camera System

- **Dynamic Zoom:** Camera auto-zooms to show the entire network, zooming out as you grow larger
- **Multi-Screen Sprawl:** Network can extend beyond visible area
- **Camera Controls:**
  - Minimap click to jump to locations
  - Tip-centered lock option
  - Keyboard pan + scroll wheel zoom

### Resource System

**Primary Resource: Nutrients/Energy**

Nutrients are gathered and spent to extend tendrils. Sources:
- **Enemy Drops:** Killed enemies leave collectible nutrient orbs
- **Environmental Nodes:** Fixed nutrient-rich spots on the map to grow toward
- **Passive Absorption:** Tendrils automatically generate nutrients from occupied territory
- **Decomposition:** Dead enemy bodies slowly release nutrients over time where they fell

**Collection Method:** Magnetic pull - resources slowly drift toward the nearest tendril

---

## Combat System

### Attack Model

- **Active Spore/Attack Cooldowns:** Tendrils periodically release attacks with cooldowns (like VS weapons)
- Attacks are automatic once tendrils are placed
- Different tendril types have different attack patterns

### Tendril Types (Prototype: 2-3 types)

Tendrils are **specialized segments** - player chooses which type when placing new growth.

Planned types:
- **Basic:** Standard growth, moderate damage
- **Toxic:** Poison damage over time, area denial
- **Sticky:** Slows enemies that contact it
- **Explosive:** Releases spore bursts on cooldown

**Synergy Bonuses:** Adjacent different tendril types create bonus effects (e.g., toxic + sticky = enhanced trapping zones)

### Network Abilities

Player has active abilities that affect the whole network:
- Heal/repair damaged sections
- Speed boost for growth
- Retract (pull back tendrils)
- More unlocked via meta-progression

---

## Danger & Survival

### What Makes Territory Dangerous

1. **Network Vulnerability:** Thin or spread tendrils can be severed, cutting off parts of your network
2. **Hostile Environment:** Biome regions with damaging conditions (acid soil, dry patches, predators)

### Severance Mechanics

- **Slow Decay:** When network is severed, cut-off sections wither over time, giving you a chance to reconnect
- **Draw Cut Line:** To intentionally sever corrupted sections, draw a line across your network to cut multiple paths at once

### Corruption/Infection (Lose Condition)

- Enemies (primarily rival fungi) can corrupt your network
- **Real-Time Continuous Spread:** Corruption visibly crawls along tendrils toward your core
- **Curable with Cost:** Spend significant resources to cleanse corruption
- **Game Over:** If corruption reaches your core node, you lose

### Health Display

- **Total Network Mass:** Aggregate health of all tendrils displayed as a percentage
- Individual tendril damage shown through visual degradation

---

## Enemies

### Behavior

- **Direct Aggression:** Enemies pathfind toward your tendrils and core to attack
- **Danger Sense:** See enemy indicators at edge of vision before they're fully visible

### Enemy Categories (Prototype: 3-4 types, one from each category)

Each category has unique mechanics:

1. **Insects:** Move in swarms, fast, melee attacks
2. **Competing Fungi:**
   - Mirror Match: They grow and spread like you, competing for space
   - Static Colonies: Pre-existing fungal zones to destroy or avoid
   - Boss Encounters: Large rival fungi with unique mechanics
   - Corruption Source: Primary source of the infection mechanic
3. **Bacteria:** Infect and corrupt, enable the infection mechanic

### Bosses

- **Fixed Map Locations:** Bosses guard specific zones on the map
- **Encounter Trigger:** Reach their territory by growing toward them
- Must defeat all major threats for "Ecosystem Collapse" win condition

---

## Progression

### In-Run Progression

- **Territory Milestones:** Reaching certain coverage % or areas triggers upgrade choices
- **3 Random Choices:** Classic VS style - pick one of three random upgrades
- **Upgrades Include:** New tendril types, enhanced attacks, network abilities, stat boosts

### Difficulty Scaling

- **Territory-Based:** Enemy strength scales with your network size
- Your own growth makes the game harder, creating natural tension

### Meta-Progression (Between Runs)

Full roguelite unlock system:
- **New Tendril Types:** Unlock new specialized tendrils to appear in upgrade pools
- **Starting Loadouts:** Unlock ability to start with certain upgrades
- **New Network Abilities:** Unlock new active abilities for the ability pool
- **Character Strains:** Different starting properties and playstyles

---

## Map & Exploration

### Generation

- **Fully Procedural:** Every run has a unique randomly generated map

### Biomes

Map has distinct biome regions, each with unique hazards:
- Forest floor (baseline)
- Rocky area (movement restrictions)
- Wet/swamp zone (different enemy types)
- Each biome has specific environmental hazards

### Exploration

- **Scout Spores:** Special ability to send temporary scouts that reveal areas but don't claim territory
- **Fog of War:** Unrevealed areas are hidden until scouted or grown into

---

## Run Structure

### Starting State

- **Character Selection:** Choose a fungal "strain" with different starting properties
- Begin with a small established core network

### Win Condition

- **Ecosystem Collapse:** Eliminate all major threats (boss fungi, apex predators)
- Bosses are at fixed map locations that must be reached and defeated

### Lose Condition

- **Corruption Reaches Core:** If the infection mechanic spreads to your central node, game over

### Pause Behavior

- **True Pause:** Everything freezes, can review network and plan
- Pauses during upgrade selection

---

## Technical Requirements

### Performance

- **60 FPS Minimum:** Smooth gameplay is non-negotiable, scale back visuals if needed
- Must handle hundreds of tendril segments and enemies simultaneously

### Visual Style

- **Pixel Art:** Retro aesthetic with clear readability
- **Continuous Animated Lines:** Smooth, organic growth animation along tendril paths

### Input Support

- Mouse + Keyboard (primary)
- Gamepad (full support)

### Architecture

- **Multiplayer Aware:** Design systems to allow multiplayer later without major rewrite
- Bevy ECS architecture supports this naturally

### Audio

- **Full Soundtrack:** Composed music tracks that evolve with gameplay phases
- (Not required for prototype)

---

## Prototype Scope

### MVP Features (Playable Prototype)

**Core Loop:** Grow, kill, survive

**Content:**
- 2-3 tendril types (basic + 1-2 specialized)
- 3-4 enemy types (one from each category: insect, fungi, bacteria)
- 1 boss encounter
- 1 playable strain/character

**Systems:**
- Growth mechanics with mouse-from-tip control
- Active spore attacks with cooldowns
- Territory milestone upgrades (3 random choices)
- Nutrient collection (magnetic pull)
- Network severance and slow decay
- Basic corruption mechanic
- Procedural map generation
- Dynamic zoom camera
- Danger sense for off-screen enemies
- Total network mass health display
- True pause
- Basic meta-progression hooks

**Deferred for Later:**
- Full strain roster
- Complete tendril type variety
- All enemy types and bosses
- Full soundtrack
- Multiplayer implementation
- Complete meta-progression system
