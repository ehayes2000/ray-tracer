const MIN = 1.17549435082228750797e-38f;
const MAX = 3.40282346638528859812e+38f;
const EPSILON = 1e-8;
const VUP = vec3f(0.0, 1.0, 0.0);
const LEAF = 1;
const INTERIOR = 0;


struct Bounds {
    p_min: vec3<f32>,
    p_max: vec3<f32>
}

struct BvhNode {
    // Interior(0) | Leaf(1)
    kind: u32,
    axis: i32,
    second_child_offset: i32,
    bounds: Bounds,
    centroid: vec3<f32>,
    primitive: Triangle
}

struct Triangle {
    a: vec3<f32>,
    b: vec3<f32>,
    c: vec3<f32>,
    material: u32
}

struct ImagePlane {
    px_delta_u: vec3<f32>,
    px_delta_v: vec3<f32>,
    px_00_loc: vec3<f32>,
}

struct Params {
    max_bounces: u32,
    samples_per_px: u32,
    vfov: f32,
    focal_len: f32,
    img_w: u32,
    img_h: u32,
    look_at: vec3<f32>,
    look_from: vec3<f32>,
}

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>
}

const LAMBERTIAN: u32 = 0;
const DIELECTRIC: u32 = 1;
const METAL: u32 = 1;

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


struct SceneEntry {
    sphere: Sphere
}

struct Sphere {
    radius: f32,
    center: vec3<f32>,
    material: Material,
}

fn image_plane() -> ImagePlane {
    let W = normalize(params.look_from - params.look_at);
    let U = normalize(cross(VUP, W));
    let V = cross(W, U);
    let H = tan(radians(params.vfov / 2.0));
    let VIEW_HEIGHT = 2.0 * H * params.focal_len;
    let VIEW_WIDTH = VIEW_HEIGHT * f32(params.img_w) / f32(params.img_h);
    let VIEW_U = VIEW_WIDTH * U;
    let VIEW_V = VIEW_HEIGHT * -V;
    let PX_DELTA_U = VIEW_U / f32(params.img_w);
    let PX_DELTA_V = VIEW_V / f32(params.img_h);
    let VIEW_UP_LEFT = params.look_from - (params.focal_len * W) - (VIEW_U / 2.0) - (VIEW_V / 2.0);
    let PX_00_LOC = VIEW_UP_LEFT + 0.5 * (PX_DELTA_U + PX_DELTA_V);
    return ImagePlane(
        PX_DELTA_U,
        PX_DELTA_V,
        PX_00_LOC
    );
}

// ___ random ___
// shoutout
// https://www.reedbeta.com/blog/quick-and-easy-gpu-random-numbers-in-d3d11/
// https://en.wikipedia.org/wiki/Xorshift
var<private> seed: u32;

fn xorshift32() -> u32
{
    var x = seed;
	x ^= x << 13;
	x ^= x >> 17;
	x ^= x << 5;
	seed = x;
	return seed;
}

const MAX_U32 = 4294967296.0;
fn rand() -> f32 {
    return f32(xorshift32()) / MAX_U32;
}

fn rand_vec_unit() -> vec3<f32> {
    for (var i = 0; i < 10; i++) {
        let v = vec3f(rand(), rand(), rand());
        let lensq = pow(length(v), 2.0);
        if (EPSILON < lensq && lensq <= 1.0) {
            return v / sqrt(lensq);
        }
    }
    return vec_zero();
}

// ____ interval ____
fn interval_surrounds(
    interval: Interval,
    t: f32
) -> bool {
    return t < interval.max && t > interval.min;
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

    if (obj.kind == LAMBERTIAN ){
        return material_scatter_lambertian(obj, ray, hit);
    } else if (obj.kind == DIELECTRIC) {
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
    direction = normalize(direction) + (obj.roughness * rand_vec_unit());
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
    if (ri * sin_theta > 1.0 || material_dielectric_reflectance(obj, cos_theta) > rand()) {
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
    var direction = hit.normal + rand_vec_unit();
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

fn material_green() -> Material  {
    return Material(1, vec3f(1.0, 1.0, 1.0),1.5,1.5);
}


// ___ hit ___


fn hit_zero() -> HitRecord {
    return HitRecord(false, 0.0, vec_zero(), vec_zero(), false, material_zero());
}

// ____ ray ____

fn ray_cast(px: vec2<f32>, offset: vec2<f32>) -> Ray {
    let img = image_plane();
    let pixel_sample = img.px_00_loc
        + ((px.x + offset.x) * img.px_delta_u)
        + ((px.y + offset.y) * img.px_delta_v);

    let ray_origin = params.look_from;
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

// ____ Bounds ____
fn bounds_select(bounds: Bounds, i: bool) -> vec3<f32> {
    return select(bounds.p_min, bounds.p_max, i);
}

fn bounds_intersect(
    bounds: Bounds,
    interval: Interval,
    ray: Ray,
    inv_dir: vec3<f32>,
    is_neg: vec3<i32>,
) -> bool {
    var t_min = (bounds_select(bounds, bool(is_neg.x)).x - ray.origin.x) * inv_dir.x;
    var t_max = (bounds_select(bounds, bool(1 - is_neg.x)).x - ray.origin.x) * inv_dir.x;
    let ty_min = (bounds_select(bounds, bool(is_neg.y)).y - ray.origin.y) * inv_dir.y;
    let ty_max = (bounds_select(bounds, bool(1 - is_neg.y)).y - ray.origin.y) * inv_dir.y;
    if (t_min > ty_max || ty_min > t_max) {
        return false;
    }
    if (ty_min > t_min) {
        t_min = ty_min;
    }
    if (ty_max < t_max){
        t_max = ty_max;
    }

    let tz_min = (bounds_select(bounds, bool(is_neg.z)).z - ray.origin.z) * inv_dir.z;
    let tz_max = (bounds_select(bounds, bool(1 - is_neg.z)).z - ray.origin.z) * inv_dir.z;
    if (t_min > tz_max || tz_min > t_max) {
        return false;
    }
    if (tz_min > t_min) {
        t_min = tz_min;
    }
    if (tz_max < t_max) {
        t_max = tz_max;
    }
    return (t_min < interval.max) && (t_max > 0.0);
}

// ____ triangle ____

fn tri_quad_material(i: i32) -> Material {
    var m = material_zero();
    let tri = triangles[i].primitive;
    if (tri.a[0] < 0.0 && tri.b[0] < 0.0 && tri.c[0]< 0.0) {
        m.color = vec3f(0.0, 1.0, 0.0);
    } else if (tri.a[1] < 0.0 && tri.b[1] < 0.0 && tri.c[1] < 0.0){
        m.color = vec3f(0.0, 0.0, 1.0);
    } else {
        m.color = vec3f(1.0, 0.0, 0.0);
    }
    return m;
}

fn tri_moller_trumbore_intersection(
    i: i32,
    r: Ray) -> HitRecord {
    let e1 = triangles[i].primitive.b - triangles[i].primitive.a;
    let e2 = triangles[i].primitive.c - triangles[i].primitive.a;
    let ray_cross_e2 = cross(r.direction, e2);
    let det = dot(e1, ray_cross_e2);

    if (det > -EPSILON && det < EPSILON) {
        return hit_zero();
    }

    let inv_det = 1.0 / det;
    let s = r.origin - triangles[i].primitive.a;
    let u = inv_det * dot(s, ray_cross_e2);

    if (u < 0.0 || u > 1.0) {
        return hit_zero();
    }

    let s_cross_e1 = cross(s, e1);
    let v = inv_det * dot(r.direction, s_cross_e1);

    if (v < 0.0 || u + v > 1.0) {
        return hit_zero();
    }

    let normal = normalize(cross(e1, e2));
    // let front_face = dot(r.direction, normal) < 0.0;
    let front_face = true;
    let t = inv_det * dot(e2, s_cross_e1);

    if (t > EPSILON) {
        let intersection_point = r.origin + r.direction * t;
        var hit = hit_zero();
        hit.hit = true;
        hit.front_face = front_face;
        hit.material = materials[triangles[i].primitive.material];
        hit.normal = normal;
        hit.t = t;
        hit.point = intersection_point;
        return hit;
    } else {
        return hit_zero();
    }
}

fn tri_hit(
    i: i32,
    r: Ray,
    t: Interval
) -> HitRecord {
    let hit = tri_moller_trumbore_intersection(i, r);
    if (hit.hit && interval_surrounds(t, hit.t)) {
        return hit;
    }
    return hit_zero();
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
    if (interval_surrounds(t, root_a)) {
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
    if (interval_surrounds(t, root_b)) {
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
    let a = 0.5 * (unit_dir.y + 1.0);
    return (1.0 - a) * vec_one() + a * vec3f(0.5, 0.7, 1.0);
}

fn world_hit_bvh(ray: Ray) -> HitRecord {
    let inv_dir = 1.0 / ray.direction;
    let is_neg = vec3i( i32(inv_dir.x < 0.), i32(inv_dir.y < 0.), i32(inv_dir.z < 0.));
    var to_visit_offset = 0;
    var current_node_i = 0;
    var nodes_to_visit = array<i32, 64>();
    var interval = Interval(EPSILON, MAX);
    var hit = hit_zero();
    // var a =0;
    while (true) {
    // if (a > 100){
    // break;
    // }
    // a +=1;
        let node = &triangles[current_node_i];
        if (bounds_intersect(node.bounds, interval, ray, inv_dir, is_neg)){
            if (node.kind == LEAF) {
                let leaf_hit = tri_moller_trumbore_intersection(current_node_i, ray);
                if (leaf_hit.hit && leaf_hit.t < interval.max){
                    hit = leaf_hit;
                    interval.max = hit.t;
                }
                if (to_visit_offset == 0) {
                    break;
                }
                to_visit_offset -= 1;
                current_node_i = nodes_to_visit[to_visit_offset];
            } else {
                if (bool(is_neg[node.axis])) {
                    nodes_to_visit[to_visit_offset] = current_node_i + 1;
                    to_visit_offset += 1;
                    current_node_i = node.second_child_offset;
                } else {
                    nodes_to_visit[to_visit_offset] = node.second_child_offset;
                    to_visit_offset += 1;
                    current_node_i += 1;
                }
            }
        } else {
            if(to_visit_offset == 0) {
                break;
            }
            to_visit_offset -= 1;
            current_node_i = nodes_to_visit[to_visit_offset];
        }
    }
    return hit;
}

fn world_hit(r: Ray) -> HitRecord {
    let t = Interval(0.01, MAX);
    var any_hit: HitRecord = hit_zero();
    var closest = t.max;
    for (var i = 0; i < i32(arrayLength(&triangles)); i ++) {
        if (triangles[i].kind == 0){
            continue;
        }
        let hit = tri_hit(i, r, Interval(t.min, closest));
        if (hit.hit) {
            closest = hit.t;
            any_hit = hit;
        }
    }
    return any_hit;
}

fn ray_trace(r: Ray) -> vec3<f32> {
    var bounces = 0;
    var color = vec3f(1.0, 1.0, 1.0);
    var ray = r;

    while(bounces < i32(params.max_bounces)) {
        bounces ++;
        let hit = world_hit_bvh(ray);
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

@group(0) @binding(0) var<uniform> params: Params
@group(0) @binding(1) var<storage, read> triangles: array<BvhNode>;
@group(0) @binding(2) var<storage, read> materials: array<Material>;

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> @builtin(position) vec4<f32> {
    if (in_vertex_index == 0u) {
        return vec4<f32>(-1.0, -1.0, 0.0, 1.0);
    } else if (in_vertex_index == 1u) {
        return vec4<f32>(3.0, -1.0, 0.0, 1.0);
    } else {
        return vec4<f32>(-1.0, 3.0, 0.0, 1.0);
    }
}

@fragment
fn fs_main(
    @builtin(position) px: vec4<f32>
) -> @location(0) vec4<f32> {
    seed = u32(ceil(px.x * px.y) + 1);
    var color = vec_zero();
    for (var i = 0; i < i32(params.samples_per_px); i ++) {
        var offset = vec2f(rand(), rand());
        offset.x -= 0.5;
        offset.y -= 0.5;
        let ray = ray_cast(px.xy, offset);
        color += ray_trace(ray);
    }
    color = color * 1.0 / f32(params.samples_per_px);
    return vec4f(color.x, color.y, color.z, 0.0);
}
