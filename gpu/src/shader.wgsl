@group(0) @binding(0) var<storage, read_write> output: array<u32>;

@compute
// https://www.w3.org/TR/WGSL/#workgroup-size-attr
@workgroup_size(8,8)
fn main(
// https://www.w3.org/TR/WGSL/#builtin-inputs-outputs
    @builtin(global_invocation_id) id: vec3<u32>
) {
    output[id.y * 8 + id.x] = id.y * 8 + id.x;
}
