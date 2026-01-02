use super::vec3::Vec3;
pub type Point3 = Vec3;

#[derive(Default)]
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

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + (self.direction * t)
    }
}
