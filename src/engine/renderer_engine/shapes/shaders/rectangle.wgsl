struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct InstanceInput {
    @location(2) color: vec3<f32>,
    @location(3) position: vec3<f32>,
    @location(4) width: f32,
    @location(5) height: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> window_size: vec2<f32>;

@vertex
fn vs_main(
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

    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
