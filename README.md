# 🌙 Orbital_System

## Orbital Rendering Subsystem (Pure Rust/WASM)

**Orbital_System** is an orbital rendering subsystem within **Visual_System**, which is part of **Interface_System** - the sensory interface layer for **Luna**, the living core of the **Modularis** framework.

**Implementation**: Pure Rust compiled to WebAssembly using the Leptos reactive framework. Zero JavaScript - the entire application, from UI to canvas rendering, runs in Rust/WASM.

It translates Luna’s energetic states, orbital mechanics, and quantum phenomena into circular/orbital visual representations.

-----

## 🏗️ Architecture

```
Modularis (framework)
  └── Luna (living core)
      ├── Core Modules:
      │   ├── Nucleus (heart)
      │   ├── Sprites (orbital entities)
      │   ├── Chronos (time)
      │   ├── Rituel (invocations)
      │   ├── Couleur (chromatic states)
      │   ├── Quantum (superposition)
      │   ├── Réparation (healing module)
      │   └── Archive (memory)
      │
      └── Interface_System (sensory interfaces)
          ├── Visual_System (visual rendering)
          │   ├── Orbital_System ← THIS SUBSYSTEM
          │   ├── Graph_System (future)
          │   ├── Particle_System (future)
          │   ├── Grid_System (future)
          │   └── Waveform_System (future)
          │
          ├── Audio_System (future)
          │   ├── Synthesis_System
          │   ├── Ritual_Sound_System
          │   └── Ambient_System
          │
          ├── Haptic_System (future)
          ├── Data_System (future)
          └── API_System (future)
```

**This is the power of Modularis**: each system and subsystem is independent, composable, and swappable. You can replace Orbital_System with another visual renderer without touching Visual_System or Interface_System.

**Orbital_System** does not contain Luna - it **visualizes** Luna. Luna is the core; Orbital_System is how you see and interact with it.

-----

## 🌟 What is Orbital_System?

Orbital_System is a **visual rendering subsystem** within the modular hierarchy:

- **Interface_System** → provides all sensory interfaces to Luna
- **Visual_System** → handles visual rendering (subsystem of Interface_System)
- **Orbital_System** → renders orbital/circular visualizations (subsystem of Visual_System)

**What it does:**

- Renders Luna’s Nucleus as a pulsating center
- Displays Sprites as orbital entities with circular paths
- Visualizes quantum states (stable vs superposed)
- Provides manual controls for Luna’s parameters
- Executes ritualistic invocations
- Records system traces in Archive

**Modularis principle**: Each layer is independent and swappable. You could replace Orbital_System with Graph_System (bar charts), Particle_System (particle fields), or Grid_System (matrix view) without changing Visual_System or Interface_System. This is true modularity - nested, composable, interchangeable systems.

-----

## 🧩 Luna Core Modules (Visualized)

### 1. **Nucleus** ❤️

The living heart of Luna. Center of gravitational and energetic influence.

**Visual representation:**

- Pulsating circle at center (250, 250)
- Size oscillates with pulsation frequency
- Color changes with état:
  - Purple (#A855F7) = actif
  - Blue (#6366F1) = latent
  - Red (#EF4444) = instable

**Parameters:**

- `gravité`: -2 to +3 (attraction/repulsion)
- `pulsation`: 20-120 bpm
- `état`: actif | latent | instable

**Controls:**

- État buttons: Click to change state
- Gravité slider: Adjust attraction force
- Pulsation slider: Control heartbeat rate

-----

### 2. **Sprites** 👁️

Orbital entities circling the Nucleus. Three default sprites:

**Aether** (Pink #FF6B9D)

- 30°/s rotation
- 100px orbital radius
- Starting angle: 0°

**Lumina** (Cyan #4ECDC4)

- -45°/s rotation (counter-clockwise)
- 150px orbital radius
- Starting angle: 120°

**Solaris** (Yellow #FFE66D)

- 20°/s rotation
- 80px orbital radius
- Starting angle: 240°

**Visual representation:**

- Colored circles orbiting Nucleus
- Dotted orbital path lines
- Name labels above each sprite
- Quantum blur when superposed

**Parameters per sprite:**

- `taille`: Circle radius (pixels)
- `couleur`: RGB hex color
- `vitesse_rotation`: Degrees per second
- `distance_centre`: Orbital radius
- `angle`: Current position (0-360°)
- `etat_quantique`: stable | superposé
- `opacite`: Transparency (0-1)
- `nom`: Ritual name

-----

### 3. **Chronos** ⚡

Time controller. Manages global animation tempo.

**Visual representation:**

- Tempo slider (0.1x - 3x speed)
- Play/Pause button
- Reset button

**Parameters:**

- `tempo_global`: Speed multiplier
- `actif`: Animation on/off
- `phase_sprite`: Display phase
- `cycle`: Loop type

**Effects:**

- Tempo affects all sprite rotation speeds
- Pause freezes all orbital motion
- Reset returns to initial state

-----

### 4. **Rituel** ✨

Invocation system. Transforms Luna through keywords.

**Visual representation:**

- Text input field
- “Invoquer” button
- List of available incantations
- Floating message display on activation

**8 Ritual Invocations:**

#### `RÉVEIL` (Awakening)

```
Effect: Activates Nucleus
Pulsation: 80 bpm
État: actif
Use: Start system with high energy
```

#### `SOMMEIL` (Sleep)

```
Effect: Calms Nucleus
Pulsation: 20 bpm
État: latent
Use: Put system to rest
```

#### `CHAOS` (Chaos)

```
Effect: Quantum superposition
Sprite speeds: Randomized (×1-4)
Opacity: Variable (0.5-1)
État quantique: superposé
Use: Create instability
```

#### `HARMONIE` (Harmony)

```
Effect: Full stabilization
All sprites: stable state
Speed: 30°/s uniform
Nucleus: actif, 60 bpm
Use: Restore balance
```

#### `ATTRACTION`

```
Effect: Increase gravity
Gravité: 2
Sprites: -30px closer (min 50px)
Use: Concentrate energy
```

#### `RÉPULSION`

```
Effect: Reverse gravity
Gravité: -1
Sprites: +30px farther (max 200px)
Use: Disperse energy
```

#### `AURORE` (Aurora)

```
Effect: Chromatic transformation
Colors: Cycle through palette
Use: Visual transformation
```

#### `RÉPARATION`

```
Effect: Complete system healing
All sprites: stabilized
Speeds: normalized
Nucleus: reset (gravité 1, 60 bpm, actif)
Use: Full restoration
```

**How to invoke:**

1. Type incantation in UPPERCASE
1. Press “Invoquer” or Enter
1. Watch visual transformation
1. Check Archive for confirmation

-----

### 5. **Couleur** 🎨

Chromatic system. Colors as vibrational states.

**Default palette:**

- `#FF6B9D` - Rose (passion)
- `#4ECDC4` - Cyan (clarity)
- `#FFE66D` - Jaune (joy)
- `#95E1D3` - Mint (healing)
- `#F38181` - Coral (tenderness)

**Modes:**

- Fixed: Constant color
- Cyclic: AURORE ritual
- Reactive: Based on quantum state

**Visual representation:**

- Color circles next to sprite names
- Smooth transitions on ritual invocation
- Orbital paths match sprite colors

-----

### 6. **Quantum** 🌀

Superposition mechanics. Non-classical states.

**States:**

**Stable:**

- Sharp, solid rendering
- Opacity: 1.0
- Single position
- No blur filter

**Superposé (Superposed):**

- Gaussian blur applied
- Ghost double at offset position
- Oscillating opacity (0.3-0.7)
- Sinusoidal position shift
- **Triggered by:** CHAOS ritual

**Visual representation:**

- Blur filter (stdDeviation: 2)
- Secondary translucent circle
- Dynamic opacity via sine wave
- Ghost offset: 10px oscillation

-----

### 7. **Réparation** ❤️

Healing module. Restores system integrity.

**Function:**

- Stabilizes all sprites
- Normalizes velocities
- Restores full opacity
- Resets Nucleus parameters
- Uniform sprite sizes

**Invocation:**

```
RÉPARATION
```

**Type:**

- Gentle and complete
- Non-destructive
- Holistic (affects entire system)

**Visual confirmation:**

- “❤️ Le système se répare avec douceur”
- Immediate visual stabilization
- Archive entry created

-----

### 8. **Archive** 📜

Living memory. Records all system actions.

**Visual representation:**

- Scrollable log panel
- Last 10 entries displayed
- Newest on top

**Entry format:**

```javascript
{
  moment: "HH:MM:SS",
  trace: "Action description",
  voix: "Orbital_System"
}
```

**Recorded events:**

- Ritual invocations
- Nucleus state changes
- Chronos modifications
- System resets

**Example traces:**

```
14:23:15 - Rituel CHAOS déclenché - Les sprites entrent en superposition
14:23:08 - Chronos réactivé
14:22:54 - Rituel RÉPARATION - Restauration complète du système
```

-----

## 🎮 Usage Guide

### First Launch

1. **Observe default state:**
- Nucleus pulses at 60 bpm (actif)
- Three sprites orbit smoothly
- Tempo at 1x speed
1. **Experiment with controls:**
- Slide Nucleus gravité
- Change état (actif/latent/instable)
- Adjust pulsation
- Modify Chronos tempo
1. **Invoke first ritual:**
   
   ```
   Type: RÉVEIL
   Press: Invoquer
   Observe: Nucleus energizes
   ```
1. **Watch Archive:**
- Every action is logged
- Timestamps automatic
- System voice records

-----

### Usage Scenarios

#### Meditation Mode

```
1. Invoke: HARMONIE
2. Set tempo: 0.5x
3. Pulsation: 40 bpm
4. État: latent
→ Calm, slow orbital flow
```

#### Energetic Exploration

```
1. Invoke: RÉVEIL
2. Tempo: 2x
3. Gravité: 2
4. Invoke: ATTRACTION
→ Concentrated, rapid orbits
```

#### Quantum Experiment

```
1. Invoke: CHAOS
2. Observe superposition
3. Adjust gravité dynamically
4. Invoke: MIKA to stabilize
→ Instability → restoration cycle
```

#### Chromatic Journey

```
1. Invoke: AURORE
2. Watch color transformation
3. Repeat for new palette
→ Visual metamorphosis
```

-----

## 🔧 Technical Implementation

### Pure Rust/WASM Stack

- **Leptos 0.6** - Reactive web framework (like React, but Rust)
- **wasm-bindgen** - Rust ↔ JavaScript interop
- **web-sys** - Web APIs in Rust (Canvas, DOM, etc.)
- **gloo-timers** - Animation loop management
- **No JavaScript required** - Everything runs in WASM

### Architecture

```
Browser
  └── WASM Runtime
      └── Leptos Framework (Rust)
          └── Orbital_System (Rust)
              ├── Luna Core (Rust structs)
              ├── Canvas Rendering (web-sys)
              ├── State Management (Leptos Signals)
              └── Event Handling (wasm-bindgen)
```

### Key Technologies

- **Reactive Signals** - Fine-grained reactivity like SolidJS
- **Effect System** - Automatic dependency tracking
- **Canvas 2D API** - Direct rendering via web-sys
- **60 FPS Animation** - requestAnimationFrame via gloo-timers

### Build Output

- **Development**: ~2-3 MB WASM (with debug symbols)
- **Production**: ~150-300 KB WASM (optimized + gzipped ~60-90 KB)
- **Zero Runtime Overhead** - No virtual DOM, direct updates

### Why Rust/WASM?

**Luna is written in Rust**, so having Interface_System in Rust provides:

1. **Zero language boundary** - Direct struct access, no serialization
1. **Type safety** - Compile-time guarantees across entire stack
1. **Performance** - Near-native speed for orbital calculations
1. **Memory safety** - No GC pauses during animations
1. **Single codebase** - One language from core to UI
1. **Portability** - WASM runs everywhere (web, desktop, embedded)

-----

## 📦 Building and Running

See [BUILD.md](./BUILD.md) for complete build instructions.

### Quick Start

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk (WASM bundler)
cargo install trunk

# Clone/create project with provided Cargo.toml and src/lib.rs

# Start development server
trunk serve

# Open http://127.0.0.1:8080
```

### Production Build

```bash
trunk build --release
# Output in dist/ directory
# Deploy dist/ to any static host
```
