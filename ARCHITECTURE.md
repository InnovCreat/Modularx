# Architecture

## Plugin Graph

```
App
├── SacredMathPlugin      sacred_math/mod.rs
│   ├── SacredFrequencies (Resource) — 639 Hz base + per-solid harmonic
│   ├── PlatonicRegistry  (Resource) — tracks the active solid
│   └── tick_frequencies  (System)  — advances elapsed time
│
├── RenderPlugin          render/mod.rs
│   ├── MaterialPlugin<SacredMaterial>
│   ├── RenderMode        (Resource) — current display mode
│   ├── PulsationPlugin
│   │   └── apply_global_pulse (System) — drives Pulsating component scale
│   ├── setup_scene       (Startup)  — spawns camera, lights, initial solid
│   ├── cycle_render_mode (Update)   — R key cycles RenderMode
│   ├── update_pulse      (Update)   — feeds pulse_phase into SacredMaterial
│   ├── select_solid      (Update)   — keys 1-5 update PlatonicRegistry
│   └── swap_solid        (Update)   — despawn/respawn on registry change
│
├── InteractionPlugin     interaction/mod.rs
│   └── OrbitalCameraPlugin
│       ├── OrbitalCamera (Resource) — yaw, pitch, distance, target
│       ├── orbit_left_drag  (Update)
│       ├── zoom_scroll      (Update)
│       └── pan_right_drag   (Update)
│
└── ArchivePlugin         archive/mod.rs
    └── LivingArchive (Resource) — append-only event log
```

## Data Flow

```
SacredFrequencies
  └─ pulse(t) ──────────────┬─► apply_global_pulse → Transform.scale
                             └─► update_pulse → SacredMaterial.pulse_phase
                                                  └─► sacred_pulse.wgsl

PlatonicRegistry.active
  └─ select_solid (keys 1-5) ──► swap_solid
                                  ├─ PlatonicSolid::build_mesh() → Mesh
                                  └─ SacredMaterial::for_solid()  → color
```

## Module Responsibilities

### `sacred_math/`

| File | Responsibility |
|------|---------------|
| `platonic.rs` | Vertex tables + face tables for all 5 solids; flat-shaded mesh builder; `PlatonicRegistry` resource |
| `frequencies.rs` | `SacredFrequencies` resource; `pulse(t)` — composite sine of 639 Hz + active solid harmonic |
| `geometry.rs` | Golden ratio φ, Fibonacci utilities, shared math constants |
| `sri_yantra.rs` | Sri Yantra intersection engine (triangles, Bindu, lotus petals) |

### `render/`

| File | Responsibility |
|------|---------------|
| `material.rs` | `SacredMaterial` — AsBindGroup wrapping `base_color`, `pulse_phase`, `fresnel_power`; per-solid color palette |
| `modes.rs` | `RenderMode` enum (6 modes); R-key cycling system |
| `pulse.rs` | `Pulsating` component; `apply_global_pulse` scales entities via `SacredFrequencies::pulse()` |

### `interaction/camera.rs`

Spherical coordinate orbital camera. State lives in `OrbitalCamera` resource:

- Yaw/pitch updated by left-drag mouse delta
- Distance clamped [1, 40] by scroll
- Target offset shifted by right-drag pan
- `apply_camera()` reconstructs `Transform` from spherical coords each frame

### `assets/shaders/sacred_pulse.wgsl`

Fragment shader bound as a custom Bevy material:

- **Fresnel** — `pow(1 - |dot(N, V)|, fresnel_power)` for rim glow
- **Pulsation** — `0.5 + 0.5 * sin(TAU * pulse_phase * φ)`
- Output = `base_color.rgb + fresnel * glow_color * pulse`

### `holographic/` (stub)

Reserved for OpenXR stereo rendering and 639 Hz binaural audio. Currently an empty plugin.

### `archive/`

`LivingArchive` resource — append-only `Vec<ArchiveEntry>` with a `seal()` method. The TODO marker indicates future Arweave HTTP integration.

## Extending the Engine

**Add a new solid:** implement `build_mesh()` logic in `platonic.rs`, extend the `PlatonicSolid` enum, and map it to a frequency and key.

**Add a render pass:** add a variant to `RenderMode` and implement the mode switch in `cycle_render_mode`.

**Add geometry:** drop a new file under `sacred_math/`, expose it through `SacredMathPlugin`.
