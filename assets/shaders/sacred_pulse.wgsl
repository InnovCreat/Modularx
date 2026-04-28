#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::view

// --- Uniforms (group 2 = custom material) ---
@group(2) @binding(0) var<uniform> base_color:    vec4<f32>;
@group(2) @binding(1) var<uniform> pulse_phase:   f32;
@group(2) @binding(2) var<uniform> fresnel_power: f32;

const PHI: f32 = 1.618033988;
const TAU: f32 = 6.28318530;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let n = normalize(in.world_normal);
    // View direction from fragment toward the camera
    let v = normalize(view.world_position.xyz - in.world_position.xyz);

    // Fresnel holographique — rim glow at silhouette
    let fresnel = pow(1.0 - saturate(dot(n, v)), fresnel_power);

    // Pulsation composite 639 Hz × φ
    let pulse = 0.5 + 0.5 * sin(TAU * pulse_phase * PHI);

    // Glow éthéré dynamique
    let glow_color = vec3<f32>(0.2, 0.5, 1.0);
    let rgb = base_color.rgb + fresnel * glow_color * pulse;

    return vec4<f32>(rgb, base_color.a);
}
