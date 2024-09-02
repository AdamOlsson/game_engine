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
    @location(1) tex_coord: vec3<f32>,
};

@group(0) @binding(0) var<uniform> window_size: vec2<f32>;

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

    // TODO: 
    // 3. Move the list of texture coords to a buffer

    out.clip_position = vec4<f32>(scaled_vertex_position, 1.0) + vec4<f32>(scaled_object_position, 0.0);
    let u32_max = 4294967295u;
    if (instance.tex_cell == u32_max) {
        out.tex_coord = vec3<f32>(-1.0,-1.0,-1.0);
    } else {
        let sprite_width = 128u;
        let cell_width: u32 = 16u;
        let cell_height: u32 = 16u;
        let px = 1.0 / f32(sprite_width);
        let cell_right_edge = px*f32(cell_width);
        let cell_bottom_edge = px*f32(cell_height);
        let offset_x = px*f32(cell_width * instance.tex_cell);
        var tex_coords = array<vec3<f32>, 6>(
                vec3<f32>(offset_x, 0.0, 0.0),
                vec3<f32>(offset_x, cell_bottom_edge, 0.0),
                vec3<f32>(cell_right_edge + offset_x, 0.0, 0.0),
                vec3<f32>(cell_right_edge + offset_x, cell_bottom_edge, 0.0),
                vec3<f32>(0.0 + offset_x, cell_bottom_edge, 0.0),
                vec3<f32>(cell_right_edge + offset_x, 0.0,0.0)
            );
        out.tex_coord = tex_coords[in_vertex_index]; 
    }

    return out;
}

@group(1) @binding(0) var texture: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (in.tex_coord.x == -1.0 && in.tex_coord.y == -1.0  && in.tex_coord.z == -1.0  ) {
        return vec4<f32>(in.color, 1.0);
    }

    let texel = textureSample(texture, texture_sampler, in.tex_coord.xy);
    return texel.bgra;
}
