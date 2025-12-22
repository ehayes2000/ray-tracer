use crate::math::Vec3;
use crate::v3;
use encase::ShaderType;

#[derive(Clone, Debug, ShaderType)]
pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub material: u32,
}

#[derive(Clone, Debug, ShaderType)]
pub struct Sphere {
    pub radius: f32,
    pub location: Vec3,
    pub material: Material,
}

pub mod MaterialKind {
    pub const LAMBERTIAN: u32 = 0;
    pub const DIELECTRIC: u32 = 1;
    pub const METAL: u32 = 2;
}

#[derive(Clone, Debug, ShaderType)]
pub struct Material {
    /// Lambertian(0) | Dielectric(1) | Metal(2)
    pub kind: u32,
    pub color: Vec3,
    pub roughness: f32,
    pub refractive_index: f32,
}

#[derive(Clone, Debug, ShaderType)]
pub struct SceneBufferEntry {
    pub sphere: Sphere,
}

#[derive(Clone, Debug, ShaderType)]
pub struct RenderParameters {
    pub max_bounces: u32,
    pub samples_per_px: u32,
    pub vfov: f32,
    pub focal_len: f32,
    pub img_w: u32,
    pub img_h: u32,
    pub look_at: Vec3,
    pub look_from: Vec3,
}

impl Default for RenderParameters {
    fn default() -> Self {
        Self {
            img_w: 256,
            img_h: 256,
            max_bounces: 2,
            samples_per_px: 25,
            focal_len: 1.0,
            look_at: v3!(0.0, 0.0, 0.0),
            look_from: v3!(5., 2., 0.),
            vfov: 90.0,
        }
    }
}
