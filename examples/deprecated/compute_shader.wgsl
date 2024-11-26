
// Note I think using the 'storage' keyword here makes the input
// in uniform address space. Within a workgroup, using 'workgroup'
// address space is faster.
@group(0) @binding(0) var<storage, read_write> input: array<u32>;

// Things I want to remember:
// - uniform address space is read only
// - storage address space is read and write

@compute @workgroup_size(2,1,1)
fn cs_main(
    @builtin(local_invocation_id) lid: vec3<u32>,
) {
  
    var n = arrayLength(&input)/2;
    let start = n*lid.x;

    for (var i: u32 = 0; i < n - 1; i = i + 1) {
        for (var j: u32 = start; j < start+n - i - 1; j = j + 1) {
            if (input[j] > input[j + 1]) {
                // Swap the elements
                let temp = input[j];
                input[j] = input[j + 1];
                input[j + 1] = temp;
            }
        }
    }

    //storageBarrier(); // Coordinate access by invocations in a single workgroup to buffers in storage address space
    workgroupBarrier(); // Coordinate access by invocations in a single workgroup to buffers in workgroup address space

    if (lid.x == 0) {
        n = arrayLength(&input);
        for (var i: u32 = 0; i < n - 1; i = i + 1) {
            for (var j: u32 = 0; j < n - i - 1; j = j + 1) {
                if (input[j] > input[j + 1]) {
                    // Swap the elements
                    let temp = input[j];
                    input[j] = input[j + 1];
                    input[j + 1] = temp;
                }
            }
        }
    }
}
