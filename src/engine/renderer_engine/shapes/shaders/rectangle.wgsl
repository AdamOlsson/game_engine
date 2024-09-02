struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct InstanceInput {
    @location(2) color: vec3<f32>,
    @location(3) position: vec3<f32>,
    @location(4) width: f32,
    @location(5) height: f32,
    @location(6) tex_cell: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
};

@group(0) @binding(0) var<uniform> window_size: vec2<f32>;

@group(1) @binding(0) var texture: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;
@group(1) @binding(2) var<uniform> texture_base: array<vec4<f32>, 7>;

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
    let u32_max = 4294967295u;
    if (instance.tex_cell == u32_max) {
        out.tex_coord = vec2<f32>(-1.0,-1.0);
    } else {
        let sprite_data = texture_base[0];
        let sprite_width = sprite_data.x;
        let sprite_height = sprite_data.y;
        let cell_width = sprite_data.z;
        let cell_height = sprite_data.w;
        let px = 1.0 / sprite_width;

        let offset_x = px*cell_width * f32(instance.tex_cell);
        let base_coord = texture_base[in_vertex_index + 1u].xy;
        let offset = vec2<f32>(offset_x, 0.0);
        out.tex_coord = base_coord + offset;
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
