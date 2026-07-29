#import bevy_pbr::forward_io::VertexOutput

// ── Uniforms (group 2 = SacredMaterialAdvanced) ──────────────────────────────
@group(2) @binding(0)  var<uniform> base_color:           vec4<f32>;
@group(2) @binding(1)  var<uniform> pulse_phase:          f32;
@group(2) @binding(2)  var<uniform> harmonic_hz:          f32;
@group(2) @binding(3)  var<uniform> fresnel_power:        f32;
@group(2) @binding(4)  var<uniform> fresnel_intensity:    f32;
@group(2) @binding(5)  var<uniform> glow_intensity:       f32;
@group(2) @binding(6)  var<uniform> emission_strength:    f32;
@group(2) @binding(7)  var<uniform> distortion_amount:    f32;
@group(2) @binding(8)  var<uniform> chromatic_aberration: f32;
@group(2) @binding(9)  var<uniform> superposition_blend:  f32;
@group(2) @binding(10) var<uniform> holographic_intensity:f32;
@group(2) @binding(11) var<uniform> crystalline_facets:   f32;
@group(2) @binding(12) var<uniform> crystalline_intensity:f32;

const PHI: f32 = 1.618033988;
const TAU: f32 = 6.28318530;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn sacred_pulse(t: f32, hz: f32) -> f32 {
    let base     = sin(TAU * t * 0.639);          // 639 Hz normalized
    let harmonic = sin(TAU * t * hz / 1000.0) * 0.3;
    return 0.5 + 0.5 * clamp(base + harmonic, -1.0, 1.0);
}

// Crystalline facet pattern — angular quantization
fn crystalline(uv: vec2<f32>, facets: f32) -> f32 {
    let angle = atan2(uv.y, uv.x);
    let quantized = floor(angle / TAU * facets) / facets * TAU;
    let diff = abs(angle - quantized);
    return 1.0 - smoothstep(0.0, 0.15, diff);
}

// Simple chromatic split: offset sample by derivate approximation
fn chromatic_shift(color: vec3<f32>, amount: f32, fresnel: f32) -> vec3<f32> {
    let shift = amount * fresnel * 0.5;
    return vec3<f32>(
        color.r + shift,
        color.g,
        color.b - shift,
    );
}

// Scanline holographic lines
fn scanlines(world_pos: vec3<f32>, intensity: f32, t: f32) -> f32 {
    let line = sin(world_pos.y * 80.0 + t * 2.0) * 0.5 + 0.5;
    return 1.0 + line * intensity * 0.3;
}

// ── Fragment ──────────────────────────────────────────────────────────────────
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let n = normalize(in.world_normal);
    let v = normalize(in.world_position.xyz);

    // ── Fresnel (rim glow) ─────────────────────────────────────────────────
    let ndotv   = clamp(dot(n, v), 0.0, 1.0);
    let fresnel = pow(1.0 - ndotv, fresnel_power) * fresnel_intensity;

    // ── Sacred pulsation at solid's frequency ─────────────────────────────
    let pulse = sacred_pulse(pulse_phase, harmonic_hz);

    // ── Glow color (φ-tinted rim) ──────────────────────────────────────────
    let glow_rgb = vec3<f32>(0.2, 0.5, 1.0) * glow_intensity * pulse;

    // ── Emission (self-illumination) ────────────────────────────────────────
    let emission = base_color.rgb * emission_strength * pulse;

    // ── Crystalline facets ─────────────────────────────────────────────────
    let uv2      = in.world_normal.xy;
    let crystal  = crystalline(uv2, crystalline_facets) * crystalline_intensity;

    // ── Holographic scanlines ──────────────────────────────────────────────
    let holo = scanlines(in.world_position.xyz, holographic_intensity, pulse_phase);

    // ── Superposition ghost (quantum blur approximation) ───────────────────
    let superpos = sin(pulse_phase * PHI + ndotv * TAU) * superposition_blend * 0.2;

    // ── Compose ────────────────────────────────────────────────────────────
    var rgb = base_color.rgb + glow_rgb * fresnel + emission + vec3<f32>(crystal) + vec3<f32>(superpos);
    rgb = chromatic_shift(rgb, chromatic_aberration, fresnel);
    rgb *= holo;
    rgb = clamp(rgb, vec3<f32>(0.0), vec3<f32>(2.0)); // allow slight overbright

    return vec4<f32>(rgb, base_color.a);
}
