use crate::math::Vec3;
use crate::v3;
use encase::ShaderType;
// use encase::vector::impl_vector;

// // #[macro_export]
// // macro_rules! v3 {
// //     ($a:expr, $b:expr , $c:expr) => {
// //         V3([$a as f32, $b as f32, $c as f32])
// //     };
// // }

// impl V3 {
//     pub fn normalize(self) -> Self {
//         let l = self.len();
//         Self([self.0[0] / l, self.0[1] / l, self.0[2] / l])
//     }
//     pub fn len(&self) -> f32 {
//         f32::sqrt(self.0[0].powi(2) + self.0[1].powi(2) + self.0[2].powi(2))
//     }
// }

// impl From<[f32; 3]> for V3 {
//     fn from(value: [f32; 3]) -> Self {
//         V3(value)
//     }
// }

// impl std::ops::Mul<f32> for V3 {
//     type Output = Self;
//     fn mul(self, rhs: f32) -> Self::Output {
//         V3([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
//     }
// }

// impl std::ops::Sub<V3> for V3 {
//     type Output = Self;
//     fn sub(self, rhs: V3) -> Self::Output {
//         V3([
//             self.0[0] - rhs.0[0],
//             self.0[1] - rhs.0[1],
//             self.0[2] - rhs.0[2],
//         ])
//     }
// }

// impl std::ops::Add<V3> for V3 {
//     type Output = Self;
//     fn add(self, rhs: V3) -> Self::Output {
//         V3([
//             self.0[0] + rhs.0[0],
//             self.0[1] + rhs.0[1],
//             self.0[2] + rhs.0[2],
//         ])
//     }
// }

// pub fn cross(u: V3, v: V3) -> V3 {
//     let u = &u.0;
//     let v = &v.0;
//     V3([
//         u[1] * v[2] - u[2] * v[1],
//         u[2] * v[0] - u[0] * v[2],
//         u[0] * v[1] - u[1] * v[0],
//     ])
// }

// impl AsRef<[f32; 3]> for V3 {
//     fn as_ref(&self) -> &[f32; 3] {
//         &self.0
//     }
// }

// impl AsMut<[f32; 3]> for V3 {
//     fn as_mut(&mut self) -> &mut [f32; 3] {
//         &mut self.0
//     }
// }

// impl_vector!(3, V3, f32; using AsRef AsMut From);

#[derive(Clone, Debug, ShaderType)]
pub struct Sphere {
    pub radius: f32,
    pub location: Vec3,
    pub material: Material,
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
            max_bounces: 15,
            samples_per_px: 10,
            focal_len: 1.0,
            look_at: v3!(0.0, 0.0, 0.0),
            look_from: v3!(5., 2., 0.),
            vfov: 90.0,
        }
    }
}
