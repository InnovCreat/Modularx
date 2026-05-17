# Modularx — GHZ 639 CORE

A native 3D sacred geometry viewer built with **Rust + Bevy 0.15**.

The engine renders the five Platonic solids with a custom WGSL shader driven by a 639 Hz composite pulsation signal. An orbital camera lets you inspect any solid from any angle.

---

## Features

- **Five Platonic solids** — procedurally generated meshes with flat shading and correct outward normals
- **Sacred frequency system** — 639 Hz base frequency with per-solid harmonics (720 – 6480 Hz)
- **Custom WGSL shader** — Fresnel rim glow + φ-modulated pulsation
- **Six render modes** — Wireframe, Hidden Line, Shaded, X-Ray, Realistic, Sacred Pulse
- **Orbital camera** — left-drag orbit, scroll zoom, right-drag pan
- **Living Archive** — in-memory event log with Arweave seal hook (stub)

---

## Quick Start

```bash
# Install Rust (stable toolchain)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the repository
git clone https://github.com/InnovCreat/modularx
cd modularx

# Run in development mode
cargo run

# Optimized release build
cargo run --release
```

See [`docs/building.md`](docs/building.md) for platform-specific notes and dependency requirements.

---

## Controls

| Input | Action |
|-------|--------|
| Left drag | Orbit camera |
| Right drag | Pan camera |
| Scroll wheel | Zoom in / out |
| `1` – `5` | Select Platonic solid |
| `R` | Cycle render mode |

Full reference: [`docs/controls.md`](docs/controls.md)

---

## Platonic Solids & Frequencies

| Key | Solid | Faces | Frequency | Element |
|-----|-------|-------|-----------|---------|
| `1` | Tetrahedron | 4 | 720 Hz | Fire |
| `2` | Cube | 6 | 1440 Hz | Earth |
| `3` | Octahedron | 8 | 2160 Hz | Air |
| `4` | Dodecahedron | 12 | 3600 Hz | Ether |
| `5` | Icosahedron | 20 | 6480 Hz | Water |

---

## Project Structure

```
modularx/
├── src/
│   ├── main.rs               # App entry point, plugin registration
│   ├── sacred_math/          # Geometry & frequency core
│   │   ├── platonic.rs       # Mesh generation for all 5 solids
│   │   ├── geometry.rs       # φ, Fibonacci, Sri Yantra helpers
│   │   ├── frequencies.rs    # SacredFrequencies resource + pulse()
│   │   └── sri_yantra.rs     # Sri Yantra intersection engine
│   ├── render/               # Bevy rendering layer
│   │   ├── material.rs       # SacredMaterial (WGSL uniform bridge)
│   │   ├── modes.rs          # RenderMode enum + R-key cycling
│   │   └── pulse.rs          # Pulsating component + scale animation
│   ├── interaction/
│   │   └── camera.rs         # OrbitalCamera (yaw/pitch/distance/pan)
│   ├── holographic/          # Future: OpenXR stereo + binaural audio
│   └── archive/              # LivingArchive event log + Arweave hook
├── assets/
│   └── shaders/
│       └── sacred_pulse.wgsl # Fragment shader (Fresnel + φ pulsation)
├── docs/
│   ├── building.md
│   └── controls.md
├── ARCHITECTURE.md
└── CONTRIBUTING.md
```

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust (stable) |
| Engine | Bevy 0.15 |
| Shaders | WGSL (via Bevy's material pipeline) |
| Future VR | OpenXR (stub in `holographic/`) |
| Future archiving | Arweave (stub in `archive/`) |

---

## License

See [LICENSE](LICENSE).
