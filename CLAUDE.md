# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Environment

This project uses a Nix flake for its dev shell — always work inside it.

## System

This is a text-based adventure game (hence the name TAG). While 99% of it will be 2D, do not rule out the use of 3D. The game targets only Web (via WASM).

## Commands

```bash
cargo build          # compile (dev profile, deps at opt-level 3)
cargo run            # build and launch the game
cargo check          # fast type-check without linking
cargo clippy         # lint
cargo test           # run tests
```

## Usage

This is a **learning** project. When prompted, provide explanation and suggest code but **do not do any edits yourself**. Prefer 'real-world' solutions and avoid 'hacks' - strive to build a system that is idiomatic and follows best practices.
