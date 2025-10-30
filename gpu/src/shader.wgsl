
const N_OBJECTS = 2;
const IMG_W = 512;
const IMG_H = 512;
const VFOV = 90.0;
const FOCAL_LEN = 1.0;
const LOOK_FROM = vec3f(10.0, 5.0, 0.0);
const LOOK_AT = vec3f(0.0, 0.0, 0.0);
const VUP = vec3f(0.0, 1.0, 0.0);
const _W = LOOK_FROM - LOOK_AT;
const _WMAG = sqrt(pow(_W.x, 2.0) + pow(_W.y, 2.0) + pow(_W.z, 2.0));
const W = _W / _WMAG;
const _U = cross(VUP, W);
const _UMAG = sqrt(pow(_U.x, 2.0) + pow(_U.y, 2.0) + pow(_U.z, 2.0));
const U = _U / _UMAG;
const V = cross(W, U);
const H = tan(radians(VFOV / 2.0));
const VIEW_HEIGHT = 2.0 * H * FOCAL_LEN;
const VIEW_WIDTH = VIEW_HEIGHT * f32(IMG_W) / f32(IMG_H);
const VIEW_U = VIEW_WIDTH * U;
const VIEW_V = VIEW_HEIGHT * -V;
const PX_DELTA_U = VIEW_U / f32(IMG_W);
const PX_DELTA_V = VIEW_V / f32(IMG_H);
const VIEW_UP_LEFT = LOOK_FROM - (FOCAL_LEN * W) - (VIEW_U / 2.0) - (VIEW_V / 2.0);
const PX_00_LOC = VIEW_UP_LEFT + 0.5 * (PX_DELTA_U + PX_DELTA_V);


@group(0) @binding(0) var<storage, read_write> output: array<u32>;
// bindings seem to be compiled away if you don't use the data
// this results in bind_group erros when creating the pipeline
@group(0) @binding(1) var<storage, read> scene: array<Sphere>;

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>
}

struct Sphere {
    radius: f32,
    location: vec3<f32>,
}

fn ray_cast(px: vec2<f32>) -> Ray {
    let pixel_sample = PX_00_LOC
        + (px.x * PX_DELTA_U)
        + (px.y * PX_DELTA_V);

    let ray_origin = LOOK_FROM;
    let ray_direction = pixel_sample - ray_origin;

    return Ray(ray_origin, ray_direction);
}

fn hit_sphere(s: Sphere, r: Ray) -> bool {
    let oc = s.location - r.origin;
    let a = pow(length(r.direction), 2.0);
    let h = dot(r.direction, oc);
    let c = pow(length(oc), 2.0) - pow(s.radius, 2.0);
    let descriminant = h * h - a * c;
    if (descriminant <= 0.) {
        return false;
    } else {
        return true;
    }
}

@compute
// https://www.w3.org/TR/WGSL/#workgroup-size-attr
@workgroup_size(16,16)
fn main(
    // https://www.w3.org/TR/WGSL/#builtin-inputs-outputs
    @builtin(global_invocation_id) id: vec3<u32>
) {
    if (id.x >= IMG_W || id.y >= IMG_H){
        return;
    }
    let idx = id.y * IMG_H + id.x;
    var ball = false;
    let ray = ray_cast(vec2f(f32(id.x), f32(id.y)));
    for (var i = 0; i < N_OBJECTS; i ++){
        let sphere = scene[i];
        if (hit_sphere(sphere, ray)) {
            ball = true;
        }
    }
    if (ball) {
        output[idx] = 0;
    } else {
        let red = u32(255.0 * f32(idx) / f32(IMG_W * IMG_H));
        output[idx] = red;
    }
}
