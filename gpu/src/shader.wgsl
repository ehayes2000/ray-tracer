const N_OBJECTS = 5;
const IMG_W = 512;
const IMG_H = 512;
const MAX_BOUNCES = 50;
const SAMPLES_PER_PX = 1000;
const VFOV = 90.0;
const FOCAL_LEN = 1.0;
const LOOK_FROM = vec3f(5., 2.,0.0);
const LOOK_AT = vec3f(0., 0., 0.);
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
const PX_SAMPLES_SCALE = 1.0 / f32(SAMPLES_PER_PX);
const MIN = 1.17549435082228750797e-38f;
const MAX = 3.40282346638528859812e+38f;
const EPSILON = 1e-8;


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
    color: vec3<f32>,
    // metal only
    roughness: f32,
    // glass only
    refraction_index: f32,
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
    material: Material,
}

struct Sphere {
    radius: f32,
    center: vec3<f32>,
    material: Material,
}

// ___ random ___
// https://www.shadertoy.com/view/4djSRW
fn rand_hash12(p: f32) -> vec2<f32>
{
	var p3 = fract(vec3f(p) * vec3f(.1031, .1030, .0973));
	p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.xx+p3.yz)*p3.zy);
}


fn rand_hash21(p: vec2<f32>) -> f32 {
	var p3  = fract(vec3f(p.xyx) * .1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

fn rand_hash22(p: vec2<f32>) -> vec2<f32>
{
	var p3 = fract(vec3f(p.xyx) * vec3f(.1031, .1030, .0973));
    p3 += dot(p3, p3.yzx+33.33);
    return fract((p3.xx+p3.yz)*p3.zy);
}

fn rand_hash31(p: vec3<f32>) -> f32
{
	var p3 = fract(p * .1031);
    p3 += dot(p3, p3.zyx + 31.32);
    return fract((p3.x + p3.y) * p3.z);
}

fn rand_hash33(p: vec3<f32>) -> vec3<f32>
{
	var p3 = fract(p * vec3f(.1031, .1030, .0973));
    p3 += dot(p3, p3.yxz+33.33);
    return fract((p3.xxy + p3.yxx)*p3.zyx);
}

fn rand_vec_unit(p: vec3<f32>) -> vec3<f32> {
    var seed = p;
    for (var i = 0; i < 1000000; i++) {
        let v = rand_hash33(seed);
        let lensq = pow(length(v), 2.0);
        if (EPSILON < lensq && lensq <= 1.0) {
            return v / sqrt(lensq);
        } else {
            seed.x += .1;
            seed.y += .1;
            seed.z += .1;
        }
    }
    return p;
}

// ____ scatter ____
fn scatter_zero() -> Scatter {
    return Scatter(false, ray_zero(), vec_zero());
}

// ____ material ____

fn material_scatter(
    obj: Material,
    ray: Ray,
    hit: HitRecord
) -> Scatter {

    if (obj.kind == 0 ){
        return material_scatter_lambertian(obj, ray, hit);
    } else if (obj.kind == 1) {
        return material_scatter_dielectric(obj, ray, hit);
    } else {
        return material_scatter_metal(obj, ray, hit);
    }
}

fn material_scatter_metal(
    obj: Material,
    ray: Ray,
    hit: HitRecord
) -> Scatter {
    var direction = reflect(ray.direction, hit.normal);
    direction = normalize(direction) + (obj.roughness * rand_vec_unit(ray.direction));
    if (dot(direction, hit.normal) > 0.0) {
        var scatter = scatter_zero();
        scatter.scatter = true;
        scatter.color = obj.color;
        scatter.ray = Ray(hit.point, direction);
        return scatter;
    } else {
        return scatter_zero();
    }
}

fn material_scatter_dielectric(
    obj: Material,
    ray: Ray,
    hit: HitRecord
) -> Scatter {
    var scatter = scatter_zero();
    let attenuation = obj.color;
    let unit_direction = normalize(ray.direction);
    var ri: f32;
    if (hit.front_face) {
        ri = 1.0 / obj.refraction_index;
    } else {
        ri = obj.refraction_index;
    }
    let cos_theta = min(dot(-unit_direction, hit.normal), 1.0);
    let sin_theta = sqrt(1.0 - pow(cos_theta, 2.0));
    var direction: vec3<f32>;
    if (ri * sin_theta > 1.0 || material_dielectric_reflectance(obj, cos_theta) > rand_hash31(ray.direction)) {
        direction = reflect(unit_direction, hit.normal);
    } else {
        direction = refract(unit_direction, hit.normal, ri);
    }
    scatter.scatter = true;
    scatter.ray = Ray(hit.point, direction);
    scatter.color = attenuation;
    return scatter;
}

fn material_dielectric_reflectance(
    obj: Material,
    theta: f32
) -> f32 {
    let r0 = pow(((1.0 - obj.refraction_index) / (1.0 + obj.refraction_index)), 2.0);
    return r0 + (1.0 - r0) * pow(1.0 - theta, 5.0);
}


fn material_scatter_lambertian(
    obj: Material,
    ray: Ray,
    hit: HitRecord
) -> Scatter {
    var scatter = scatter_zero();
    var direction = hit.normal + rand_vec_unit(ray.direction);
    if (vec_near_zero(direction)) {
        direction = hit.normal;
    }

    scatter.scatter = true;

    scatter.ray = Ray(
        hit.point,
        direction
    );
    scatter.color = obj.color;
    return scatter;
}
// util
fn material_zero() -> Material {
    return Material(0, vec_zero(), 0.0, 0.0);
}

// ___ hit ___


fn hit_zero() -> HitRecord {
    return HitRecord(false, 0.0, vec_zero(), vec_zero(), false, material_zero());
}

// ____ ray ____

fn ray_cast(px: vec2<f32>, offset: vec2<f32>) -> Ray {
    let pixel_sample = PX_00_LOC
        + ((px.x + offset.x) * PX_DELTA_U)
        + ((px.y + offset.y) * PX_DELTA_V);

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

fn vec_one() -> vec3f {
    return vec3f(1.0,1.0,1.0);
}

fn vec_near_zero(v: vec3<f32>) -> bool {
    return abs(v.x) < EPSILON && abs(v.y) < EPSILON && abs(v.z) < EPSILON;
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
        hit.t = root_a;
        hit.material = obj.material;
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
        hit.t = root_b;
        hit.material = obj.material;
        return hit;
    }
    return hit_zero();
}

fn sky_color(ray: Ray) -> vec3<f32> {
    let unit_dir = normalize(ray.direction);
    let a = 0.5 * (unit_dir.x + 1.0);
    return (1.0 - a) * vec3f(1., 1., 1.) + a * vec3f(0.5, 0.7, 1.0);
}

fn world_hit(r: Ray) -> HitRecord {
    let t = Interval(0.01, MAX);
    var any_hit: HitRecord = hit_zero();
    var closest = t.max;
    for (var i = 0; i < N_OBJECTS; i ++) {
        let hit = sphere_hit(scene[i], r, Interval(t.min, closest));
        if (hit.hit) {
            closest = hit.t;
            any_hit = hit;
        }
    }
    return any_hit;
}


fn compute_color(r: Ray) -> vec3<f32> {
    var interval = Interval(0.001, MAX);
    var bounces = 0;
    var color = vec3f(1.0, 1.0, 1.0);
    var ray = r;

    while(bounces < MAX_BOUNCES) {
        bounces ++;
        let hit = world_hit(ray);
        if (hit.hit) {
            let scatter = material_scatter(hit.material, ray, hit);
            ray = scatter.ray;
            // bounce
            if (scatter.scatter) {
                color = color * scatter.color;
            // ray absorbed
            } else {
                return vec_zero();
            }
        // nothing hit
        } else {
            color = color * sky_color(ray);
            return color;
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
    var color = vec_zero();
    for (var i = 0; i < SAMPLES_PER_PX; i ++) {
        var offset = rand_hash12(f32(i));
        offset.x -= 0.5;
        offset.y -= 0.5;
        let ray = ray_cast(vec2f(f32(id.x), f32(id.y)), offset);
        color += compute_color(ray);
    }
    color = color * PX_SAMPLES_SCALE;
    let packed_color = u32(color.x * 255.0) << 16 | u32(color.y * 255.0)  << 8 | u32(color.z * 255.0);
    output[idx] = packed_color;
}
