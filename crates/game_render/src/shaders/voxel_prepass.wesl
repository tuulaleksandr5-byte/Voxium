import bevy_pbr::prepass::io as prepass_io;
import bevy_pbr::render::pbr_prepass_functions::calculate_motion_vector;

@group(constants::MATERIAL_BIND_GROUP) @binding(100) var tile_array: texture_2d_array<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(101) var tile_sampler: sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(102) var<uniform> tile_layer: u32;

fn discard_leaf(in: prepass_io::VertexOutput) {
    let texel = textureSample(tile_array, tile_sampler, in.uv, i32(tile_layer));
    if tile_layer == 9u && texel.a < 0.45 { discard; }
}

@if(PREPASS_FRAGMENT)
@fragment
fn fragment(in: prepass_io::VertexOutput) -> prepass_io::FragmentOutput {
    discard_leaf(in);
    var out: prepass_io::FragmentOutput;
@if(NORMAL_PREPASS)
    out.normal = vec4(in.world_normal * 0.5 + vec3(0.5), 1.0);
@if(UNCLIPPED_DEPTH_ORTHO_EMULATION)
    out.frag_depth = in.unclipped_depth;
@if(MOTION_VECTOR_PREPASS)
    out.motion_vector = calculate_motion_vector(in.world_position, in.previous_world_position);
    return out;
}
@else
// Alpha-masked directional shadows require a fragment entry point even when
// there are no color/depth outputs. PREPASS_FRAGMENT is false in this variant.
@fragment
fn fragment(in: prepass_io::VertexOutput) {
    discard_leaf(in);
}
