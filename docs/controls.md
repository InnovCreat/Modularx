# Controls

## Camera

| Input | Action |
|-------|--------|
| Left mouse button + drag | Orbit (rotate) around target |
| Right mouse button + drag | Pan (shift target point) |
| Scroll wheel | Zoom in / out (distance 1 – 40 units) |

Pitch is clamped to ±1.4 rad so the camera never flips past the poles.

## Solid Selection

| Key | Solid | Frequency |
|-----|-------|-----------|
| `1` | Tetrahedron | 720 Hz |
| `2` | Cube | 1440 Hz |
| `3` | Octahedron | 2160 Hz |
| `4` | Dodecahedron | 3600 Hz |
| `5` | Icosahedron | 6480 Hz |

Switching a solid updates both the mesh and the active frequency used for the composite pulse.

## Render Modes

Press `R` to cycle through six modes in order:

| Mode | Description |
|------|-------------|
| Wireframe | Edge-only display |
| Hidden Line | Wireframe with back-face culling |
| Shaded | Flat-shaded faces, no shader |
| X-Ray | See-through shaded |
| Realistic | Directional light + shadows |
| Sacred Pulse | Custom WGSL shader — Fresnel glow + φ pulsation |

The current mode is printed to stdout (`info!`) on each cycle.
