# AZZLE Task #4 Unified Execution Checklist — Rusty Plumber

Task state checked from AZZLE open market task 4: POSTED, claimable, but funded=false / lockedUsdc=0. Do not claim until deliverable QA is buttoned up and funding/scope risk is accepted.

## Differences and reconciliation

Compared `TASK4_CHECKLIST.md` against `clicky_implementation.md` and merged the brief's six work items into the existing requirement audit without duplicate work.

- Enemy sprite flip bug: the original checklist only said “multiple enemies with simple AI.” The brief adds a concrete bug fix: enemies moving left must use `Sprite::flip_x`, not negative/near-zero `Transform.scale.x`. Status is now COMPLETE after replacing transform-scale facing with `sprite.flip_x = velocity.x < 0.0` in `enemy_ai`.
- Camera follow: both files cover this. The checklist marked it PRESENT; the brief adds smooth follow and level-bound clamping. Code already lerps camera x/y and clamps to current `LEVELS[game.level].width`, so no duplicate work needed.
- Collectibles: both files cover collectibles, but the brief adds per-level tracking. Status is now COMPLETE after adding `collectibles_total` and `collectibles_remaining` to `Game`, initializing/resetting them per level, decrementing on pickup, and surfacing count in the HUD.
- Multiple levels: both files cover this. Existing `LEVELS` data is already data-driven and has two distinct playable levels with level transition on goal collision. No duplicate work needed.
- Checkpoints: both files cover this. Existing checkpoint collision updates `game.checkpoint`, deaths respawn there, and level changes reset to the next level's spawn. No duplicate work needed.
- Win/loss conditions: both files cover this. Existing `GameMode::Won` / `GameMode::Lost` banners and `R` restart reset all game state. Restart now also resets collectible counts.
- Original assets / deployment: the checklist had asset/deploy tasks not in the brief. These remain required for AZZLE delivery and were retained.
- Gap fixed in this pass: enemy flip-X bug and per-level collectible tracking.

## Unified plan and execution status

| # | Unified requirement | Source | Status | Evidence | Action |
|---|---|---|---|---|---|
| 1 | Complete Super Mario-style 2D platformer | Checklist | COMPLETE | Bevy platformer includes movement, jumping, gravity, collision, platforms, collectibles, enemies, hazards, camera, levels, checkpoints, win/loss, HUD, sounds, shop | Keep QA only |
| 2 | Entirely in Rust using Bevy | Checklist | COMPLETE | `Cargo.toml` uses Bevy 0.14; gameplay is in `src/main.rs` | Keep |
| 3 | Compile to WebAssembly | Checklist / brief constraint | COMPLETE | `trunk build --release --public-url /rusty-plumber/` is the deployment build | Re-run before push |
| 4 | Deploy to GitHub Pages | Checklist / brief constraint | COMPLETE | `.github/workflows/pages.yml` deploys `dist`; live Pages deployment succeeded for prior push | Re-run workflow after next push if committing |
| 5 | Public GitHub repository | Checklist | COMPLETE | `origin=https://github.com/Syrexle/rusty-plumber.git` | Keep |
| 6 | Source code, build config, deployment workflow | Checklist | COMPLETE | `src/main.rs`, `Cargo.toml`, `Trunk.toml`, `.github/workflows/pages.yml` | Keep |
| 7 | Smooth player movement left/right | Checklist / brief context | COMPLETE | `player_input` handles A/D and arrows with acceleration/friction | Browser QA optional |
| 8 | Jumping | Checklist / brief context | COMPLETE | `player_input` handles Space/W/Up with variable jump and jump SFX | Browser QA optional |
| 9 | Gravity | Checklist / brief context | COMPLETE | `apply_velocity` applies `GRAVITY` and terminal fall speed | Keep |
| 10 | Collision detection | Checklist / brief context | COMPLETE | AABB `overlap`, platform resolution, collectibles/enemy/hazard/goal/checkpoint collision | Keep |
| 11 | Platforms | Checklist / brief context | COMPLETE | `LevelSpec.platforms` drives platform spawn | Keep |
| 12 | Collectibles spawn from level data | Checklist / brief item 3 | COMPLETE | `LevelSpec.coins` drives `Collectible` entity spawn in `spawn_level` | Keep |
| 13 | Collectibles despawn exactly once on pickup | Brief item 3 | COMPLETE | `collect_items` despawns matching collectible entity via `despawn_recursive()` | Keep |
| 14 | Collectibles update score and play sound | Checklist / brief item 3 | COMPLETE | `collect_items` adds score/bank/frog coins and calls `play("coin")` | Keep |
| 15 | Track per-level collectible counts | Brief item 3 | COMPLETE | Added `collectibles_total` / `collectibles_remaining`, reset on restart and level transition, decremented on pickup, displayed in HUD | Done in this pass |
| 16 | Multiple enemies with simple patrol AI | Checklist / brief context | COMPLETE | `LevelSpec.enemies`; `enemy_ai` patrols between left/right bounds | Keep |
| 17 | Fix enemy left-facing flat/invisible sprite bug | Brief item 1 | COMPLETE | `enemy_ai` now sets `Sprite::flip_x` and keeps `Transform.scale.x = 1.0` instead of near-zero/negative transform scale | Done in this pass |
| 18 | Hazards | Checklist | COMPLETE | `LevelSpec.hazards`; `hazard_goal_checkpoint` damages on overlap; falling below threshold damages | Keep |
| 19 | Smooth clamped camera follow | Checklist / brief item 2 | COMPLETE | `camera_follow` lerps toward player and clamps x/y to level/canvas bounds | Keep |
| 20 | Sound effects | Checklist / brief context | COMPLETE | WASM `azzlePlaySound` bridge supports jump/coin/hurt/checkpoint/win/stomp runtime sounds | Browser QA optional |
| 21 | Score | Checklist / brief context | COMPLETE | Score increments on collectibles, stomp, checkpoint, goal | Keep |
| 22 | Lives | Checklist / brief context | COMPLETE | `damage_player` decrements lives; `GameMode::Lost` at zero | Keep |
| 23 | HUD | Checklist | COMPLETE | HUD displays level, score, lives, apple count, bank coins, frog coins, controls, owned powerups | Done in this pass for apple count |
| 24 | Data-driven multiple levels | Checklist / brief item 4 | COMPLETE | `LEVELS` contains two distinct `LevelSpec` layouts: platforms, enemies, collectibles, hazards, checkpoint, start, goal, shop, width | Keep |
| 25 | Level loader / transition | Brief item 4 | COMPLETE | Reaching goal despawns `LevelEntity`, increments `game.level`, resets checkpoint and collectible counts, then calls `spawn_level` | Keep |
| 26 | Checkpoint entities | Checklist / brief item 5 | COMPLETE | `Checkpoint { position }` entity spawned per level from `LevelSpec.checkpoint` | Keep |
| 27 | Death respawns at latest checkpoint | Brief item 5 | COMPLETE | `hazard_goal_checkpoint` updates `game.checkpoint`; `damage_player` respawns at `game.checkpoint` while lives remain | Keep |
| 28 | Win condition | Checklist / brief item 6 | COMPLETE | Final-level goal sets `GameMode::Won`, plays win sound, banner shows final score | Keep |
| 29 | Loss condition | Checklist / brief item 6 | COMPLETE | `lives <= 0` sets `GameMode::Lost`; banner shows game over | Keep |
| 30 | Restart resets all game state | Brief item 6 | COMPLETE | `restart_input` resets level, lives, score, collectible counts, bank/frog coins, powerups, shop state, checkpoint, mode, and respawns level | Done in this pass for collectible counts |
| 31 | Keyboard controls | Checklist | COMPLETE | A/D/arrows/W/Space/R/E/Enter/Esc | Keep |
| 32 | Responsive browser support | Checklist | COMPLETE | Bevy window uses `fit_canvas_to_parent: true`; live visual QA rendered fullscreen canvas | Browser smoke after push |
| 33 | Original assets only | Checklist | COMPLETE | Gameplay references use `assets/original/*.png`; removed `assets/pixel_adventure` and unused audio files from tracked repo; generated art script committed | Keep |
| 34 | Compile/run without errors | Checklist | COMPLETE | `cargo fmt --check`, `cargo check`, and `trunk build --release --public-url /rusty-plumber/` passed after reconciliation changes | Keep verifying before each push |
| 35 | GitHub Pages loads directly and is playable desktop | Checklist | READY TO DEPLOY | Prior live deployment loaded and rendered game; this pass now has a clean local release build | Push/watch workflow, then smoke-test live URL |

## Final execution order

1. Read both root files and reconcile requirements. COMPLETE.
2. Inspect `src/main.rs` against the brief's acceptance criteria. COMPLETE.
3. Fix enemy patrol facing with `Sprite::flip_x`. COMPLETE.
4. Add per-level collectible count tracking and HUD display. COMPLETE.
5. Update this unified checklist with no duplicate work. COMPLETE.
6. Run `cargo fmt --check`, `cargo check`, and `trunk build --release --public-url /rusty-plumber/`. COMPLETE.
7. If checks pass, commit/push and watch GitHub Pages workflow. PENDING.
8. Smoke-test live URL with cache busting. PENDING.
9. Only after funded/scope looks safe: decide whether to claim AZZLE task 4. PENDING USER DECISION.
