# Building

## Prerequisites

- **Rust stable** — install via [rustup](https://rustup.rs/)
- A GPU with Vulkan, Metal, or DirectX 12 support (Bevy's default backends)

### Linux — system libraries

Bevy requires several system packages on Linux:

```bash
# Debian / Ubuntu
sudo apt-get install -y \
  libasound2-dev libudev-dev libwayland-dev \
  libxkbcommon-dev pkg-config

# Arch
sudo pacman -S alsa-lib systemd-libs wayland libxkbcommon pkgconf
```

### macOS

No extra steps — Metal is used automatically.

### Windows

No extra steps — DirectX 12 is used automatically.

## Development Build

```bash
cargo run
```

The `dynamic_linking` feature is enabled for Bevy in the dev profile, which significantly reduces incremental compile times.

## Release Build

```bash
cargo run --release
```

Release profile uses full LTO, a single codegen unit, and strips debug symbols — producing a small, optimized binary.

## Asset Hot-Reload

Bevy does **not** hot-reload shaders in the current configuration. If you edit `assets/shaders/sacred_pulse.wgsl`, restart the app to see changes.

## Running Tests

```bash
cargo test
```

Math utilities in `sacred_math/` are unit-testable without a GPU.

## IDE Setup

[rust-analyzer](https://rust-analyzer.github.io/) works out of the box. For best results in VS Code, install the `rust-analyzer` extension and set:

```json
"rust-analyzer.cargo.features": ["dynamic_linking"]
```
