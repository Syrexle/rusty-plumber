## Coordination with existing checklist
Before implementing any item, compare this brief against the existing `TASK4_CHECKLIST.md` in the `rusty-plumber` project. Align terminology and acceptance criteria with that file, and avoid duplicating work already marked complete. If there are conflicts, prefer the checklist’s required deliverables, then propose updates back to the brief as a single consolidated plan. Document any overlaps, changes, or gaps in a short "Differences and reconciliation" note before coding.

# Rusty Plumber — Task 4 Implementation Brief (Hermes)

## Context
Bevy WASM platformer in the `rusty-plumber` project (azle-projects). Task 4 on the azle.org open market, deadline August 4. Already working: smooth player movement, jumping, gravity, collision detection, platforms, multiple enemies, sound effects, score, lives. This brief covers the remaining gaps plus one bug fix.

## Work Items

### 1. Fix enemy sprite flip bug
When an enemy moves left, the sprite turns flat/invisible. Likely cause: negative scale applied on the wrong axis, or flipping the entire Transform instead of setting the sprite's `flip_x` flag. Fix by toggling `Sprite::flip_x` based on movement direction; do not mutate Transform scale for facing.
- Acceptance: enemies render correctly facing both directions while patrolling.

### 2. Camera follow
Smooth-follow camera tracking the player, with lerp/damping and clamping to level bounds so the camera never shows outside the level.
- Acceptance: camera follows the player horizontally and vertically, no jitter, clamped at level edges.

### 3. Collectibles
Coin-style collectibles placed per level. On pickup: despawn, play sound, increment score. Track per-level collectible counts.
- Acceptance: collectibles spawn from level data, are collectible exactly once, update score and play SFX.

### 4. Multiple levels
Data-driven level definitions (at least 2-3 levels): platform layout, enemy spawns, collectibles, checkpoints, start point, and exit/goal. Level loader that despawns the current level and spawns the next. Level progression on reaching the goal.
- Acceptance: at least two distinct playable levels with transition between them.

### 5. Checkpoints
Checkpoint entities within levels. Touching one sets the player's respawn point. On death (with lives remaining), respawn at the last activated checkpoint instead of level start.
- Acceptance: respawn honors the most recently activated checkpoint; checkpoints persist within a level attempt.

### 6. Win/loss conditions
Win: reaching the final level's goal shows a win screen with final score and a restart option. Loss: lives reaching zero shows a game-over screen with restart. Restart fully resets game state (score, lives, level, checkpoints).
- Acceptance: both end states reachable, restart works cleanly from each, no stale state after restart.

## Constraints
- Keep it WASM-compatible; verify the web build still deploys to GitHub Pages.
- Follow existing project structure and Bevy version already in Cargo.toml; no engine upgrades.
- Small, reviewable commits per work item.