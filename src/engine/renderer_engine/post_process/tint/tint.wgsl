struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
};

@group(0) @binding(0) var texture_sampler: sampler;
@group(0) @binding(1) var texture: texture_2d<f32>;

@vertex
fn vs_main(
    @location(0) position: vec3<f32>, 
    @builtin(vertex_index) vs_index: u32,
) -> VertexOutput {

    var shader_coords = array<vec2<f32>, 4>(
        vec2<f32>(0.0,0.0),
        vec2<f32>(0.0,1.0),
        vec2<f32>(1.0,0.0),
        vec2<f32>(1.0,1.0),
    );

    var output: VertexOutput;
    output.clip_position = vec4<f32>(position, 1.0);
    output.tex_coord = shader_coords[vs_index];
    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let texel = textureSample(texture, texture_sampler, in.tex_coord.xy);
    let tint = texel * 0.1;
    return vec4(tint.r, tint.g, tint.b, texel.a);
}
