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


    out.clip_position = vec4<f32>(scaled_vertex_position, 1.0) + vec4<f32>(scaled_object_position, 0.0);
    // TODO: How do I know which of these elements the vertex should sample for texture
    // - I need to know which vertex_index within the local shape (i.e vertex_index - base_vertex)
    // - base_vertex I need to find a way to provide but I don't need that until I maybe would
    // would merge the circle and rect vertex buffers.
    let half_px = 0.0078125;
    var tex_coords = array<vec3<f32>, 6>(
        vec3<f32>(half_px,half_px,0.0), vec3<f32>(half_px,0.125-half_px,0.0), vec3<f32>(0.125-half_px,half_px,0.0),
        vec3<f32>(0.125 - half_px, 0.125 - half_px, 0.0),
        vec3<f32>(0.0,0.125-half_px,0.0),
        vec3<f32>(0.125-half_px,0.0,0.0)
    );
    // TODO: in.tex_cell == 0 means use color.
    // TODO: offset all coordinates depending on the in.tex_cell
    // let offset_tex_coords = ...
    //let offset_tex_coords = tex_coords;
    out.tex_coord = tex_coords[in_vertex_index]; 
   
    return out;
}

@group(1) @binding(0) var texture: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let texel = textureSample(texture, texture_sampler, in.tex_coord.xy);
    return texel.bgra;
}
