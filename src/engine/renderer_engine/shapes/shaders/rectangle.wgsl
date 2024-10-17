struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct InstanceInput {
    @location(2) color: vec3<f32>,
    @location(3) center: vec3<f32>,
    @location(4) rotation: f32,
    @location(5) width: f32,
    @location(6) height: f32,
    @location(7) sprite_coords: vec4<f32>,
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
    curr: vec2<f32>, target_top_left: vec2<f32>, target_bot_right: vec2<f32>
) -> vec2<f32> {
    // Vertices are offset to positive x and y axis
    let offset_curr = curr + vec2<f32>(1.0,1.0);
    // Normalize width and height from 2.0 to 1.0
    let normalized_curr = offset_curr / 2.0;
    return target_top_left + (normalized_curr * (target_bot_right - target_top_left));
}

fn compute_sprite_coordinate(
    vertex_position: vec3<f32>, sprite_data: vec4<f32>, sprite_bbox: vec4<f32>
) -> vec2<f32> {
    let norm_cell_dim = sprite_data.zw / sprite_data.xy;
    let font_coordinate_upside_down = scale_one_sprite_coordinate(vertex_position.xy, sprite_bbox.xy, sprite_bbox.zw);
    let font_coordinate = abs(font_coordinate_upside_down - vec2<f32>(0.0,1.0));
    let norm_font_coordinate = font_coordinate*norm_cell_dim;
    return norm_font_coordinate;
}

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    vertex: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    var out: VertexOutput;
    out.color = instance.color; 

    let scaled_object_center = instance.center / vec3<f32>(window_size, 1.0);
    
    let scaled_object_width = instance.width / window_size[0];
    let scaled_object_height = instance.height / window_size[1];
    let scaled_vertex_position = vertex.position * vec3<f32>(scaled_object_width/2.0, scaled_object_height/2.0, 1.0);

    let rotation_matrix = mat2x2<f32>(
            vec2<f32>(cos(instance.rotation), -sin(instance.rotation)),
            vec2<f32>(sin(instance.rotation),  cos(instance.rotation)));
    let rotated_vertex_position = rotation_matrix * scaled_vertex_position.xy;

    out.clip_position = vec4<f32>(rotated_vertex_position, 0.0, 1.0) + vec4<f32>(scaled_object_center, 0.0);
    let none = vec4<f32>(-1.0,0.0,0.0,0.0);
    if (instance.sprite_coords.x == none.x) {
        out.tex_coord = vec2<f32>(-1.0,-1.0);
    } else {
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
