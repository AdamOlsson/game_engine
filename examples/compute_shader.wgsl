@group(0) @binding(0) var<storage, read_write> input: array<u32>;

@compute @workgroup_size(1)
fn cs_main(
    @builtin(global_invocation_id) id: vec3<u32>,
) {
    input[id.x] = input[id.x] + 1;
}
