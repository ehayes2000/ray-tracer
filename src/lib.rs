pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod color;
#[cfg(feature = "gpu")]
pub mod gpu;
pub mod hittable;
pub mod material;
pub mod math;
pub mod mesh;
pub mod sphere;

pub type Float = f64;
pub const EPSILON: Float = 0.1;
