
const N_OBJECTS = 3;
const IMG_W = 512;
const IMG_H = 512;
const MAX_BOUNCES = 50;
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
const MIN = 1.17549435082228750797e-38f;
const MAX = 3.40282346638528859812e+38f;


@group(0) @binding(0) var<storage, read_write> output: array<u32>;
// bindings seem to be compiled away if you don't use the data
// this results in bind_group erros when creating the pipeline
@group(0) @binding(1) var<storage, read> scene: array<Sphere>;

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>
}

struct Material {
    // Lambertian(0) | Dielectric(1) | Metal(2)
    kind: u32,
    // roughness or refractive_index
    roughness_refractive: f32,
    color: vec3<f32>,
}

struct Scatter {
    scatter: bool,
    ray: Ray,
    color: vec3<f32>
}

struct Interval {
    min: f32,
    max: f32
}

struct HitRecord {
    hit: bool,
    t: f32,
    point: vec3<f32>,
    normal: vec3<f32>,
    front_face: bool,
}

struct Sphere {
    radius: f32,
    center: vec3<f32>,
    material: Material,
}

// ____ scatter ____
fn scatter_zero() -> Scatter {
    return Scatter(false, ray_zero(), vec_zero());
}

// ____ material ____

fn material_scatter(
    obj: Material,
    hit: HitRecord
) -> Scatter {


    if (obj.kind == 0 ){
        return material_scatter_lambertian(obj, hit );
    } else if (obj.kind == 1) {
        return material_scatter_dielectric(obj, hit);
    } else {
        return material_scatter_metal(obj, hit);
    }
}

fn material_scatter_metal(
    obj: Material,
    hit: HitRecord
) -> Scatter {
    var scatter = scatter_zero();
    scatter.color = vec3f(0.0, 1.0, 0.0);
    scatter.scatter = true;
    return scatter;
}

fn material_scatter_dielectric(
    obj: Material,
    hit: HitRecord
) -> Scatter {
    var scatter = scatter_zero();
    scatter.color = vec3f(0.0, 0.0, 1.0);
    scatter.scatter = true;
    return scatter;
}


fn material_scatter_lambertian(
    obj: Material,
    hit: HitRecord
) -> Scatter {
    // TODO random gen
    var scatter = scatter_zero();
    scatter.scatter = true;
    // wrong? // why ray at normal? should be ai = ao
    scatter.ray = Ray(
        hit.point,
        hit.normal
    );
    scatter.color = obj.color;
    return scatter;
}


fn hit_zero() -> HitRecord {
    return HitRecord(false, 0.0, vec_zero(), vec_zero(), false);
}

// ____ ray ____

fn ray_cast(px: vec2<f32>) -> Ray {
    let pixel_sample = PX_00_LOC
        + (px.x * PX_DELTA_U)
        + (px.y * PX_DELTA_V);

    let ray_origin = LOOK_FROM;
    let ray_direction = pixel_sample - ray_origin;

    return Ray(ray_origin, ray_direction);
}

fn ray_at(r: Ray, t: f32) -> vec3<f32> {
 return r.origin + r.direction * t;
}

fn ray_zero() -> Ray {
    return Ray(vec_zero(), vec_zero());
}

fn vec_zero() -> vec3f {
    return vec3f(0.0,0.0,0.0);
}

// remove
fn hit_sphere(s: Sphere, r: Ray) -> bool {
    let oc = s.center - r.origin;
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

// ____ sphere ____
fn sphere_hit(
    obj: Sphere,
    ray: Ray,
    t: Interval,
) -> HitRecord {
    let oc = obj.center - ray.origin;
    let a = pow(length(ray.direction), 2.0);
    let h = dot(ray.direction, oc);
    let c = pow(length(oc), 2.0) - pow(obj.radius, 2.0);
    let descriminant = h * h - a * c;
    if (descriminant <= 0.) {
        var hit = hit_zero();
        // hit.hit = true;
        return hit;
    }
    let root_a = (h - sqrt(descriminant)) / a;
    if (t.min < root_a && root_a < t.max){
        let p = ray_at(ray, root_a);
        let normal = (p - obj.center) / obj.radius;
        var hit = hit_zero();
        hit.hit = true;
        hit.point = p;
        let front_face = dot(ray.direction, normal) < 0.0;
        if (front_face) {
            hit.normal = normal;
        } else {
            hit.normal = -normal;
        }
        hit.front_face = front_face;
        return hit;
    }
    let root_b = (h + sqrt(descriminant)) / a;
    if (t.min < root_b && root_b < t.max) {
        let p = ray_at(ray, root_b);
        let normal = (p - obj.center) / obj.radius;
        var hit = hit_zero();
        hit.hit = true;
        hit.point = p;
        let front_face = dot(ray.direction, normal) < 0.0;
        if (front_face) {
            hit.normal = normal;
        } else {
            hit.normal = -normal;
        }
        hit.front_face = front_face;
        return hit;
    }
    return hit_zero();
}

fn sky_color(ray: Ray) -> vec3<f32> {
    let unit_dir = normalize(ray.direction);
    let a = 0.5 * (unit_dir.x + 1.0);
    return (1.0 - a) * vec3f(1., 1., 1.) + vec3f(0.5, 0.7, 1.0);
}


fn compute_color(r: Ray) -> vec3<f32> {

    var interval = Interval(0.001, MAX);
    var is_hit = false;
    var bounces = 0;
    var color = vec3f(1.0, 1.0, 1.0);
    var ray = r;

    while(bounces < MAX_BOUNCES) {
        bounces ++;
        for (var i = 0; i < N_OBJECTS; i ++){
            let hit = sphere_hit(scene[i], ray, interval);
            if (hit.hit) {
                is_hit = true;
                let scatter = material_scatter(scene[i].material, hit);
                ray = scatter.ray;
                if (scatter.scatter) {
                    color = color * scatter.color;
                } else {
                    return vec_zero();
                }
                break;
            }
        }
        if (!is_hit) {
            color = color * sky_color(ray);
            return color;
        } else {
            is_hit = false;
        }
    }
    return color;
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
    let ray = ray_cast(vec2f(f32(id.x), f32(id.y)));
    let color = compute_color(ray);
    let packed_color = u32(color.x * 255.0) << 16 | u32(color.y * 255.0)  << 8 | u32(color.z * 255.0);
    output[idx] = packed_color;
    //
    //
    // working single cast
    // let idx = id.y * IMG_H + id.x;
    // var ball = false;
    // let ray = ray_cast(vec2f(f32(id.x), f32(id.y)));
    // for (var i = 0; i < N_OBJECTS; i ++){
    //     let sphere = scene[i];
    //     if (hit_sphere(sphere, ray)) {
    //         ball = true;
    //     }
    // }
    // if (ball) {
    //     output[idx] = 0;
    // } else {
    //     let red = u32(255.0 * f32(idx) / f32(IMG_W * IMG_H));
    //     output[idx] = red;
    // }
}
