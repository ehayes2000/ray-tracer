
const SIZE: u32 = 16;

@group(0) @binding(0) var output: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(SIZE, SIZE)
fn main(
  @builtin(global_invocation_id) gid: vec3<u32>,
  @builtin(workgroup_id)         wid: vec3<u32>,
) {
  let dims = textureDimensions(output);
  if (gid.x >= dims.x || gid.y >= dims.y) { return; }

  // number of workgroups in x/y (same math as your dispatch)
  let groups = (dims + vec2<u32>(SIZE - 1u)) / vec2<u32>(SIZE);

  // normalize wid into 0..1 across the grid of workgroups
  let uv = vec2<f32>(wid.xy) / vec2<f32>(groups - vec2<u32>(1u));

  if (wid.x % 2 == 0 || wid.y % 2 == 0) {
    let color = vec4<f32>(0.0,0.0,0.0,0.0);
    textureStore(output, gid.xy, color);
  } else {
    let color = vec4<f32>(uv.x, uv.y, 0.0, 1.0);
    textureStore(output, gid.xy, color);
  }
}
