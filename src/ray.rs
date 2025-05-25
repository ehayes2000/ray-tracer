use crate::vec3::Vec3;
pub type Point3 = Vec3;

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

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &(&self.direction * t)
    }
}
