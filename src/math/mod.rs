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

pub type Float = f64;
pub const EPSILON: Float = 0.001;
