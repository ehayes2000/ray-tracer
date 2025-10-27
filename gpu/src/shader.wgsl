struct Sphere {
    radius: f32,
    _pad: f32,
    location: vec2<f32>,
}

const WIDTH: u32 = 8;
const HEIGHT: u32 = 8;

@group(0) @binding(0) var<storage, read_write> output: array<u32>;
// bindings seem to be compiled away if you don't use the data
// this results in bind_group erros when creating the pipeline
@group(0) @binding(1) var<storage, read> scene: array<Sphere>;

@compute
// https://www.w3.org/TR/WGSL/#workgroup-size-attr
@workgroup_size(8,8)
fn main(
// https://www.w3.org/TR/WGSL/#builtin-inputs-outputs
    @builtin(global_invocation_id) id: vec3<u32>
) {
    let sphere: Sphere = scene[0];
    let idx = id.y * HEIGHT + id.x;
    let red = u32(255.0 * f32(idx) / f32(WIDTH * HEIGHT));
    if (sqrt(pow(f32(id.x) - sphere.location.x, 2) + pow(f32(id.y) - sphere.location.y, 2)) <= sphere.radius) {
        output[idx] = 0;
    } else {
        output[idx] = red;
    }
    // output[id.y * 8 + id.x] = id.y * 8 + id.x;
}
