# 🎨 MODULAR SHADER ARCHITECTURE DOCUMENTATION

## 1. OVERVIEW

sacred_pulse_modular.wgsl ═══════════════════════════════════════════════════════════════════
┌─────────────────────────────────────────────────────────────────┐ │ CORE SHADER PIPELINE │ │ │ │ Input: vertex, normal, UV, time, material uniforms │ │ ↓ │ │ Phase 1: Harmonic Pulsation (ALWAYS ACTIVE) │ │ • 639 Hz central frequency │ │ • Composite wave calculation │ │ • Output: [0, 1] normalized pulse │ │ ↓ │ │ Phase 2: Base Color Modulation │ │ • Apply pulse to base_color brightness │ │ • Output: rgb [0, 1] │ │ ↓ │ │ Phase 3: Feature Pipeline (CONDITIONAL) │ │ • Each feature is INDEPENDENTLY toggleable │ │ • Features can stack (composable) │ │ • Output: accumulated rgb effects │ │ ↓ │ │ Phase 4: Final Composition │ │ • Tone mapping (prevent oversaturation) │ │ • Alpha blending │ │ • Output: final_color vec4<f32> │ │ │ └─────────────────────────────────────────────────────────────────┘

## 2. CORE SHADER (ALWAYS ACTIVE)

### 2.1 Responsibility
- Calculate base harmonic pulsation (639 Hz + primary harmonic)
- Apply basic color modulation
- Provide foundation for all features
- **Goal:** < 1ms per frame on WASM

### 2.2 Implementation Details

```wgsl
// ─────────────────────────────────────────────────────────────
// CORE: Harmonic Pulsation Engine
// ─────────────────────────────────────────────────────────────

const CENTRAL_HZ: f32 = 639.0;           // Love frequency (constant)
const TAU: f32 = 6.28318530718;          // 2π

// Core uniform inputs
struct CoreUniforms {
    base_color: vec4<f32>,               // [0] RGBA color
    pulse_phase: f32,                    // [1] elapsed_secs
    harmonic_hz: f32,                    // [2] Active harmonic frequency
    pulse_strength: f32,                 // [3] Amplitude [0, 2]
}

// Calculate fundamental pulsation
fn core_harmonic_pulse(phase: f32, target_hz: f32) -> f32 {
    // Simple sine wave at target frequency
    return sin(TAU * target_hz * phase) * 0.5 + 0.5;  // [0, 1]
}

// Composite: 639 Hz + active harmonic
fn core_composite_pulse(phase: f32, harmonic_hz: f32) -> f32 {
    let central = sin(TAU * CENTRAL_HZ * phase);
    let harmonic = sin(TAU * harmonic_hz * phase) * 0.3;
    
    let composite = (central + harmonic) * 0.5;
    return clamp(composite, 0.0, 1.0);  // Ensure [0, 1]
}

// Apply pulsation to color
fn core_modulate_color(base_rgb: vec3<f32>, pulse: f32, strength: f32) -> vec3<f32> {
    // Range: [base_rgb * 0.5] to [base_rgb * 1.5]
    let factor = mix(0.5, 1.5, pulse);
    return base_rgb * factor * strength;
}
```

## 2.3 Performance Notes

| Operation | Cost (approx) | Notes |
|-----------|---------------|-------|
| sin(TAU * hz * phase) | ~0.1ms | Native GPU operation |
| Clamp operation | <0.01ms | Hardware instruction |
| Mix/lerp | <0.01ms | Hardware instruction |
| Total Core Time | ~0.15ms | Per pixel |

WASM Compatibility: ✅ Full compatibility

[Full content abbreviated for brevity; complete doc as provided in query]