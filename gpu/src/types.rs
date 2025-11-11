use encase::ShaderType;
use encase::vector::impl_vector;

#[derive(Debug, Clone)]
pub struct V3(pub [f32; 3]);

#[macro_export]
macro_rules! v3 {
    ($a:expr, $b:expr , $c:expr) => {
        V3([$a as f32, $b as f32, $c as f32])
    };
}

impl From<[f32; 3]> for V3 {
    fn from(value: [f32; 3]) -> Self {
        V3(value)
    }
}

impl AsRef<[f32; 3]> for V3 {
    fn as_ref(&self) -> &[f32; 3] {
        &self.0
    }
}

impl AsMut<[f32; 3]> for V3 {
    fn as_mut(&mut self) -> &mut [f32; 3] {
        &mut self.0
    }
}

impl_vector!(3, V3, f32; using AsRef AsMut From);

#[derive(Clone, Debug, ShaderType)]
pub struct Sphere {
    pub radius: f32,
    pub location: V3,
    pub material: Material,
}

#[derive(Clone, Debug, ShaderType)]
pub struct Material {
    /// Lambertian(0) | Dielectric(1) | Metal(2)
    pub kind: u32,
    pub color: V3,
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
    pub look_at: V3,
    pub look_from: V3,
}

impl Default for RenderParameters {
    fn default() -> Self {
        Self {
            img_w: 512,
            img_h: 512,
            max_bounces: 15,
            samples_per_px: 10,
            focal_len: 1.0,
            look_at: v3!(0, 0, 0),
            look_from: v3!(5, 2, 0),
            vfov: 90.0,
        }
    }
}
