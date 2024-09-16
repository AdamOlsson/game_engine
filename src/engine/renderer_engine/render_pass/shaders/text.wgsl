
struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct InstanceInput {
    @location(1) position: vec3<f32>,
    @location(2) tex_coords: vec4<f32>,
    @location(3) size: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
};

@group(0) @binding(0) var<uniform> window_size: vec2<f32>;

@group(1) @binding(0) var texture: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;
@group(1) @binding(2) var<uniform> font_info: vec4<f32>;

fn scale_one_font_coordinate(
    vs_position: vec2<f32>, target_top_left: vec2<f32>, target_bot_right: vec2<f32>
) -> vec2<f32> {
    return target_top_left + (vs_position * (target_bot_right - target_top_left));
}

fn compute_font_coordinate(
    vertex_position: vec3<f32>, font_data: vec4<f32>, font_bbox: vec4<f32>
) -> vec2<f32> {
    let norm_cell_dim = font_data.zw / font_data.xy;
    let font_coordinate_upside_down = scale_one_font_coordinate(vertex_position.xy, font_bbox.xy, font_bbox.zw);
    let font_coordinate = abs(font_coordinate_upside_down - vec2<f32>(0.0,1.0));
    let norm_font_coordinate = font_coordinate*norm_cell_dim;
    return norm_font_coordinate;
}

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    var out: VertexOutput;
    
    let scaled_object_position = instance.position / vec3<f32>(window_size, 1.0);

    let scaled_object_width = instance.size/ window_size[0];
    let scaled_object_height = instance.size / window_size[1];
    let scaled_vertex_position = vertex.position * vec3<f32>(scaled_object_width, scaled_object_height, 1.0);

    out.clip_position = vec4<f32>(scaled_vertex_position, 1.0) + vec4<f32>(scaled_object_position, 0.0);
    out.tex_coord = compute_font_coordinate(vertex.position, font_info, instance.tex_coords);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let texel = textureSample(texture, texture_sampler, in.tex_coord);
    return texel.bgra;
}
