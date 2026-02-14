use super::vec3::Vec3;
use crate::Float;
pub type Point3 = Vec3;

#[derive(Default, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

// constructors
impl Ray {
    pub fn new() -> Self {
        Self::zero()
    }

    pub fn zero() -> Self {
        Self {
            origin: Point3::zero(),
            direction: Vec3::zero(),
        }
    }

    pub fn at(&self, t: Float) -> Point3 {
        self.origin + (self.direction * t)
    }
}

#[macro_export]
macro_rules! ray {
    ($origin:expr, $direction:expr) => {
        $crate::math::Ray {
            origin: $origin,
            direction: $direction,
        }
    };
}
