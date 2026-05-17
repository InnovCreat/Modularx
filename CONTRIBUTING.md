# Contributing

## Getting Started

1. Fork the repository and create a feature branch off `main`
2. Follow the build instructions in [`docs/building.md`](docs/building.md)
3. Make your changes, run `cargo clippy` and `cargo test`
4. Open a pull request with a clear description of what changed and why

## Code Style

- Run `cargo fmt` before committing — the project uses standard Rust formatting
- Run `cargo clippy -- -D warnings` and fix all warnings before opening a PR
- No comments explaining what code does — only add a comment when the *why* is non-obvious

## Module Guidelines

| Module | Constraint |
|--------|-----------|
| `sacred_math/` | Pure Rust, no Bevy — keeps geometry logic testable without a GPU |
| `render/` | Bevy systems only — no geometry math here |
| `interaction/` | Input and camera only — no rendering logic |
| `archive/` | Append-only — never mutate existing entries |

## Adding a Platonic Solid

1. Add a variant to `PlatonicSolid` in `src/sacred_math/platonic.rs`
2. Implement `frequency()`, `name()`, `vertices()`, and `build_mesh()` for the variant
3. Add a dual in `dual()`
4. Map it to a `KeyCode` in the `select_solid` system in `src/render/mod.rs`
5. Add a color in `SacredMaterial::for_solid()` in `src/render/material.rs`

## Adding a Render Mode

1. Add a variant to `RenderMode` in `src/render/modes.rs`
2. Extend the `next()` match arm to include it in the cycle
3. Implement the actual rendering behavior (material swap, pipeline change, etc.)

## Reporting Issues

Open a GitHub issue with:
- OS and GPU
- Rust toolchain version (`rustc --version`)
- Steps to reproduce
- Expected vs actual behaviour
