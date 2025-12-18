use super::types::{Material, MaterialKind, Triangle};

#[derive(Clone, Debug, Default)]
pub struct Scene {
    triangles: Vec<Triangle>,
    materials: Vec<Material>,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_triangles<T>(mut self, triangles: T) -> Self
    where
        T: IntoIterator<Item = Triangle>,
    {
        self.triangles.append(triangles);
        self
    }
}
