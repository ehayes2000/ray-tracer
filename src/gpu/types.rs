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

#[derive(Clone, Debug)]
pub enum MaterialKind {
    Lambertian = 0,
    Dielectric = 1,
    Metal = 2,
}

impl MaterialKind {
    pub fn try_from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::Lambertian),
            1 => Some(Self::Dielectric),
            2 => Some(Self::Metal),
            _ => None,
        }
    }
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
            img_w: 512,
            img_h: 512,
            max_bounces: 2,
            samples_per_px: 10,
            focal_len: 1.0,
            look_at: v3!(0.0, 0.0, 0.0),
            look_from: v3!(5., 2., 0.),
            vfov: 90.0,
        }
    }
}
