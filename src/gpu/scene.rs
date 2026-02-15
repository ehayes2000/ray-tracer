use super::{
    mesh::Mesh,
    types::{Material, Triangle},
};

#[derive(Clone, Debug, Default)]
pub struct Scene {
    meshes: Vec<(Mesh, u32)>,
    pub materials: Vec<Material>,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_mesh(mut self, mesh: Mesh, material: u32) -> Self {
        self.meshes.push((mesh, material));
        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.materials.push(material);
        self
    }

    pub fn triangles(&self) -> Vec<Triangle> {
        self.meshes
            .iter()
            .flat_map(|(mesh, material)| mesh.clone().into_triangles(*material))
            .collect()
    }
}
