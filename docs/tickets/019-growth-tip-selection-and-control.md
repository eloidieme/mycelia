title:	[FEATURE] Growth Tip Selection and Control
state:	OPEN
author:	eloidieme
labels:	enhancement
comments:	0
assignees:	
projects:	
milestone:	
number:	19
--
## Summary
Implement the ability to select and control growth tips with mouse input. This is the foundation for interactive tendril growth - players click on growth tips to select them, and the selected tip follows the mouse cursor to determine growth direction.

## Motivation
The spec defines "Mouse-from-Tip Control" where the active tendril tip chases the mouse cursor position, and "Explicit Tip Selection" where clicking on a tip selects it as the active growth point. This is the core interaction model for the game.

## Detailed Design

### Components
- `SelectedGrowthTip` - Marker component for the currently selected tip
- `GrowthTipHover` - Marker for tips under mouse cursor (for visual feedback)

### Systems
1. **Tip Hover Detection** - Raycast from cursor to detect tips under mouse
2. **Tip Selection** - On click, mark the clicked tip as selected (deselect others)
3. **Growth Direction Update** - Selected tip's direction points toward cursor
4. **Visual Feedback** - Render selected vs unselected tips differently (already partially done in rendering)

### Resources
- `ActiveGrowthTip` - Resource holding the Entity of the currently selected tip (if any)

### Interactions
- Left-click on a tip: Select it as active
- Left-click on empty space: Could spawn new tip from core (or deselect - TBD)
- Mouse movement: Update selected tip's target direction

## Acceptance Criteria
- [ ] Clicking on a growth tip selects it (only one can be selected)
- [ ] Selected tip is visually distinct (brighter/highlighted)
- [ ] Selected tip's direction updates to point toward mouse cursor
- [ ] Hovering over tips shows visual feedback
- [ ] `ActiveGrowthTip` resource tracks the currently selected entity
- [ ] Unit tests for selection logic
- [ ] Integration tests for hover detection

## Test Plan
1. Unit tests for `ActiveGrowthTip` resource operations
2. Unit tests for selection/deselection logic
3. System tests verifying direction updates toward cursor
4. Visual verification that selected tips are highlighted

## Dependencies
- #5 Basic Tendril Segment Data Structure (GrowthTip component)
- #6 Tendril Rendering (visual distinction for selected tips)
- #3 Input Handling System (CursorWorldPosition)
