#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return material_color * height(mesh.world_position.xy);
}


fn height(uv: vec2<f32>) -> f32 {
    return 0.5 * (1.0 + uv.x.cos() * uv.y.cos());
}