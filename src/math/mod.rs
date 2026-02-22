mod axis;
mod interval;
mod ray;
mod util;
mod vec3;

pub use axis::*;
pub use interval::*;
pub use ray::*;
pub use util::*;
pub use vec3::*;

#[cfg(not(feature = "gpu"))]
pub type Float = f64;

#[cfg(feature = "gpu")]
pub type Float = f32;

pub const EPSILON: Float = 1E-9;
