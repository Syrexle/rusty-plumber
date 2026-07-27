# Rusty Plumber

A complete original Super Mario-style 2D platformer written entirely in Rust with Bevy and built for WebAssembly/GitHub Pages.

## Delivery / license status

This repository and its deployed demo are provided only as proof-of-completion for the related AZZLE task.

Delivery/protocol rights are pending AZZLE acceptance and payment. Until then, this code, assets, demo, and related artifacts are not licensed for reuse, copying, redistribution, resale, or derivative work.

## Features

- Smooth left/right player movement, variable jump, gravity and camera follow
- Tile-like platform collision, moving enemies with simple patrol AI, hazards and stomp defeats
- Collectibles, score, lives, HUD, checkpoints, multiple levels, win/loss states
- Keyboard controls and responsive browser canvas
- Original procedural rectangle art and generated sound effects only
- GitHub Actions workflow that compiles WASM and deploys to Pages

## Controls

- Move: Arrow keys or A/D
- Jump: Space, W, or Up Arrow
- Restart after win/loss: R

## Local run

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve --open
```

Native development build:

```bash
cargo run
```

Production WASM build:

```bash
trunk build --release --public-url ./
```
