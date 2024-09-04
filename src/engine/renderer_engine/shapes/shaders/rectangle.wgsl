struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct InstanceInput {
    @location(2) color: vec3<f32>,
    @location(3) position: vec3<f32>,
    @location(4) width: f32,
    @location(5) height: f32,
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
    curr: vec2<f32>, target_top_left: vec2<f32>, target_bot_right: vec2<f32>
) -> vec2<f32> {
    return target_top_left + (curr * (target_bot_right - target_top_left));
}

fn compute_sprite_coordinates(
    sprite_data: vec4<f32>, sprite_coords: vec4<f32>
) -> array<vec2<f32>, 6> {
    let cell_dims = sprite_data.zw / sprite_data.xy; // Normalized

    let top_left = vec2<f32>(0.0,0.0); 
    let bot_left = vec2<f32>(0.0,1.0); 
    let top_right = vec2<f32>(1.0,0.0); 
    let bot_right =  vec2<f32>(1.0,1.0); 

    let target_top_left = sprite_coords.xy;
    let target_bot_right = sprite_coords.zw;
   
    let scaled_top_left  = scale_one_sprite_coordinate(top_left, target_top_left, target_bot_right);
    let scaled_bot_left  = scale_one_sprite_coordinate(bot_left, target_top_left, target_bot_right);
    let scaled_top_right = scale_one_sprite_coordinate(top_right, target_top_left, target_bot_right);
    let scaled_bot_right = scale_one_sprite_coordinate(bot_right, target_top_left, target_bot_right);

    let offset_top_left = scaled_top_left*cell_dims;
    let offset_bot_left = scaled_bot_left*cell_dims;
    let offset_top_right = scaled_top_right*cell_dims;
    let offset_bot_right = scaled_bot_right*cell_dims;

    return array<vec2<f32>, 6>(
       offset_top_left, offset_bot_left, offset_top_right,
       offset_bot_right, offset_bot_left, offset_top_right
    );
}

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    vertex: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    var out: VertexOutput;
    out.color = instance.color; 

    let scaled_object_position = instance.position / vec3<f32>(window_size, 1.0);
    
    let scaled_object_width = instance.width / window_size[0];
    let scaled_object_height = instance.height / window_size[1];
    let scaled_vertex_position = vertex.position * vec3<f32>(scaled_object_width, scaled_object_height, 1.0);

    out.clip_position = vec4<f32>(scaled_vertex_position, 1.0) + vec4<f32>(scaled_object_position, 0.0);
    let none = vec4<f32>(-1.0,0.0,0.0,0.0);
    if (instance.sprite_coords.x == none.x) {
        out.tex_coord = vec2<f32>(-1.0,-1.0);
    } else {
        var sprite_coordinates = compute_sprite_coordinates(sprite_info, instance.sprite_coords);
        out.tex_coord = sprite_coordinates[in_vertex_index];
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
