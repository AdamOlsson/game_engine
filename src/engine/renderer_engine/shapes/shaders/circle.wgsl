struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct InstanceInput {
    @location(2) position: vec3<f32>,
    @location(3) color: vec3<f32>,
    @location(4) radius: f32,
    @location(5) rotation: f32,
    @location(6) sprite_coords: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
};

@group(0) @binding(0) var<uniform> window_size: vec2<f32>;
@group(1) @binding(0) var texture: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;
@group(1) @binding(2) var<uniform> sprite_info: vec4<f32>;


fn scale_one_sprite_coordinate(
    vs_position: vec2<f32>, target_top_left: vec2<f32>, target_bot_right: vec2<f32>
) -> vec2<f32> {
    let target_dimensions = target_bot_right - target_top_left;
    let target_radius = target_dimensions / 2.0;
    let target_center = (target_top_left + target_bot_right) / 2.0;
    return (vs_position*target_radius) + target_center;
}

fn compute_sprite_coordinate(
    vertex_position: vec3<f32>, sprite_data: vec4<f32>, sprite_bbox: vec4<f32>
) -> vec2<f32> {
    let norm_cell_dim = sprite_data.zw / sprite_data.xy;
    let sprite_coordinate_upside_down = scale_one_sprite_coordinate(vertex_position.xy, sprite_bbox.xy, sprite_bbox.zw);
    let sprite_coordinate = abs(sprite_coordinate_upside_down - vec2<f32>(0.0,1.0));
    let norm_sprite_coordinate = sprite_coordinate*norm_cell_dim;
    return norm_sprite_coordinate;
}

@vertex
fn vs_main(
    @builtin(vertex_index) vs_index: u32,
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = instance.color;

    let scaled_object_position = instance.position / vec3<f32>(window_size, 1.0);

    // Circle vertices are defined with radius 1.0 using vertices
    let scaled_object_radius = vec2<f32>(instance.radius, instance.radius) / window_size;
    let scaled_vertex_position = vertex.position * vec3<f32>(scaled_object_radius, 1.0);

    let rotation_matrix = mat2x2<f32>(
            vec2<f32>(cos(-instance.rotation), -sin(-instance.rotation)),
            vec2<f32>(sin(-instance.rotation),  cos(-instance.rotation)));
    let rotated_vertex_position = rotation_matrix * scaled_vertex_position.xy;

    out.clip_position = vec4<f32>(rotated_vertex_position, 0.0, 1.0) + vec4<f32>(scaled_object_position, 0.0);

    let none = vec4<f32>(-1.0,0.0,0.0,0.0);
    if (instance.sprite_coords.x == none.x) {
        out.tex_coord = vec2<f32>(-1.0,-1.0);
    } else {
        let norm_cell_dim = sprite_info.zw / sprite_info.xy;
        out.tex_coord = compute_sprite_coordinate(vertex.position, sprite_info, instance.sprite_coords); 
    } 

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (in.tex_coord.x == -1.0 && in.tex_coord.y == -1.0) {
        return vec4<f32>(in.color, 1.0);
    }
    let texel = textureSample(texture, texture_sampler, in.tex_coord);
    return texel.bgra;
}
