#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> base_color:    vec4<f32>;
@group(2) @binding(1) var<uniform> pulse_phase:   f32;
@group(2) @binding(2) var<uniform> fresnel_power: f32;
@group(2) @binding(3) var<uniform> render_mode:   u32;

const PHI: f32 = 1.618033988;
const TAU: f32 = 6.28318530;

// Must match material.rs constants
const MODE_SHADED:       u32 = 0u;
const MODE_XRAY:         u32 = 1u;
const MODE_REALISTIC:    u32 = 2u;
const MODE_SACRED_PULSE: u32 = 3u;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let n = normalize(in.world_normal);
    let v = normalize(in.world_position.xyz);

    // Flat color, no effects
    if render_mode == MODE_SHADED {
        return base_color;
    }

    // Edge-glow only: strong Fresnel rim, face nearly invisible
    // Creates an X-Ray silhouette without requiring alpha blending
    if render_mode == MODE_XRAY {
        let rim = pow(1.0 - abs(dot(n, v)), 2.0);
        let rgb = base_color.rgb * rim + vec3<f32>(0.0, 0.1, 0.2) * (1.0 - rim) * 0.15;
        return vec4<f32>(rgb, 1.0);
    }

    let fresnel = pow(1.0 - abs(dot(n, v)), fresnel_power);

    // Fresnel rim highlight, no pulsation
    if render_mode == MODE_REALISTIC {
        let rim = vec3<f32>(1.0, 1.0, 1.0);
        let rgb = base_color.rgb * 0.7 + fresnel * rim * 0.6;
        return vec4<f32>(rgb, 1.0);
    }

    // Full sacred pulse: Fresnel + φ-modulated pulsation + glow
    let pulse = 0.5 + 0.5 * sin(TAU * pulse_phase * PHI);
    let glow_color = vec3<f32>(0.2, 0.5, 1.0);
    let rgb = base_color.rgb + fresnel * glow_color * pulse;
    return vec4<f32>(rgb, 1.0);
}
