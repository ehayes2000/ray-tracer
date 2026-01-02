
@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> @builtin(position) vec4<f32> {
if (in_vertex_index == 0u) {
        return vec4<f32>(-1.0, -1.0, 0.0, 1.0);
    } else if (in_vertex_index == 1u) {
        return vec4<f32>(3.0, -1.0, 0.0, 1.0);  // Counter-clockwise
    } else {
        return vec4<f32>(-1.0, 3.0, 0.0, 1.0);
    }
}

// Fragment shader

@fragment
fn fs_main(
    @builtin(position) in: vec4<f32>
    ) -> @location(0) vec4<f32> {

    let s = in / 512.0;

    return vec4<f32>(s.x, s.y, (s.x + s.y) * .5, 1.0);
}
