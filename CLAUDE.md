# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Environment

This project uses a Nix flake for its dev shell — always work inside it.

## System

This is a text-based adventure game (hence the name TAG). While 99% of it will be 2D, do not rule out the use of 3D.

## Commands

```bash
cargo build          # compile (dev profile, deps at opt-level 3)
cargo run            # build and launch the game
cargo check          # fast type-check without linking
cargo clippy         # lint
cargo test           # run tests
```

## Architecture

This is a [Bevy](https://bevyengine.org/) ECS game (v0.18). All game logic lives in `src/game.rs` behind `GamePlugin`, which is registered in `src/main.rs` alongside `DefaultPlugins`.

**Bevy ECS pattern used throughout:**
- **Components** — plain structs/enums derived from `Component`, attached to entities.
- **Resources** — global singletons derived from `Resource`, accessed via `Res<T>` / `ResMut<T>`.
- **Systems** — ordinary functions whose parameters are `Query`, `Commands`, `Res`, etc.; registered via `app.add_systems(Schedule, system)`.
- **Plugins** — structs implementing `Plugin` that group related systems/resources; the canonical way to organise features as the codebase grows.

New features should be added as additional `Plugin` implementations in `src/` and registered in `main.rs`, keeping `main.rs` as a thin entry point.

## Usage

This is a **learning** project. When prompted, provide explanation and suggest code but **do not do any edits yourself**. Prefer 'real-world' solutions and avoid 'hacks' - strive to build a system that is idiomatic and follows best practices.
