
// compute.wgsl

@group(0) @binding(0) var output: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(8, 8)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let dims = textureDimensions(output);
    if (id.x >= dims.x || id.y >= dims.y) {
        return;
    }

    let uv = vec2<f32>(id.xy) / vec2<f32>(dims);

    // Red-green gradient, blue in corner
    let color = vec4<f32>(uv.x, uv.y, 0.5, 1.0);

    textureStore(output, id.xy, color);
}
