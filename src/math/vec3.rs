use encase::impl_vector;
use std::f32;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::util::{random, random_f32};

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Vec3(pub f32, pub f32, pub f32);
pub type Point = Vec3;
pub type Color = Vec3;

#[macro_export]
macro_rules! v3 {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::math::Vec3($x as f32, $y as f32, $z as f32)
    };
}

impl AsRef<[f32; 3]> for Vec3 {
    fn as_ref(&self) -> &[f32; 3] {
        unsafe { &*(self as *const Vec3 as *const [f32; 3]) }
    }
}

impl AsMut<[f32; 3]> for Vec3 {
    fn as_mut(&mut self) -> &mut [f32; 3] {
        unsafe { &mut *(self as *mut Vec3 as *mut [f32; 3]) }
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(value: [f32; 3]) -> Self {
        Self(value[0], value[1], value[2])
    }
}

#[cfg(test)]
mod test_array_conversions {
    use super::*;
    #[test]
    fn test_as_ref() {
        let v = v3!(5., 5., 5.);
        let arr = v.as_ref();
        assert_eq!(arr, &[5., 5., 5.]);
    }

    #[test]
    fn test_as_mut() {
        let mut v = v3!(1., 1., 1.);
        let arr = v.as_mut();
        arr[0] = 5.;
        arr[1] = 5.;
        arr[2] = 5.;
        assert_eq!(arr, &[5., 5., 5.]);
        assert_eq!(v, v3!(5., 5., 5.));
    }

    #[test]
    fn test_from() {
        let arr = [5., 4., 3.];
        let v = Vec3::from(arr);
        assert_eq!(v3!(5., 4., 3.), v);
    }
}

// this lets Vec3vector  be packed as a wgsl vec3<f32>
impl_vector!(3, Vec3, f32; using AsRef AsMut From);

impl Vec3 {
    pub fn len(&self) -> f32 {
        f32::sqrt(self.len_squared())
    }

    pub fn len_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn zero() -> Self {
        Vec3(0., 0., 0.)
    }

    pub fn one() -> Self {
        Vec3(1., 1., 1.)
    }

    pub fn unit_random() -> Self {
        loop {
            let v = Self::random_mm(-1.0, 1.0);

            let lensq = v.len_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return v / f32::sqrt(lensq);
            }
        }
    }

    pub fn random_mm(min: f32, max: f32) -> Self {
        Vec3(
            random_f32(min, max),
            random_f32(min, max),
            random_f32(min, max),
        )
    }

    pub fn random_from_normal(normal: &Vec3) -> Self {
        let random = Self::unit_random();
        if dot(&random, normal) < 0.0 {
            -random
        } else {
            random
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit = Self::unit_random();
        if dot(&on_unit, normal) > 0.0 {
            on_unit
        } else {
            -on_unit
        }
    }

    pub fn random_on_disk() -> Self {
        loop {
            let p = Vec3(random(), random(), 0.0);
            if p.len_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn near_zero(&self) -> bool {
        let e = 1e-8;
        self.0.abs() < e && self.1.abs() < e && self.2.abs() < e
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        self - 2. * dot(self, normal) * normal
    }

    pub fn refract(&self, n: &Vec3, etai_over_etat: f32) -> Self {
        let cos_theta = f32::min(dot(&-self, n), 1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.len_squared())) * n;
        r_out_perp + r_out_parallel
    }

    pub fn normalize(self) -> Self {
        self / self.len()
    }

    pub fn max() -> Self {
        Self(f32::MAX, f32::MAX, f32::MAX)
    }

    pub fn min() -> Self {
        Self(f32::MIN, f32::MIN, f32::MIN)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.5} {:.5} {:.5}", self.0, self.1, self.2)
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    (u.0 * v.0) + (u.1 * v.1) + (u.2 * v.2)
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3(
        u.1 * v.2 - u.2 * v.1,
        u.2 * v.0 - u.0 * v.2,
        u.0 * v.1 - u.1 * v.0,
    )
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.len()
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self += &rhs;
    }
}

impl Add for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        self + *rhs
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;
    fn add(mut self, rhs: &Vec3) -> Self::Output {
        self += rhs;
        self
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, mut rhs: Vec3) -> Self::Output {
        rhs += self;
        rhs
    }
}

#[cfg(test)]
#[test]
#[allow(clippy::op_ref)]
fn test_add() {
    let a = Vec3::zero();
    let b = Vec3::one();
    let expected = b;

    let mut d = a;
    d += b;
    let mut e = a;
    e += &b;

    assert_eq!(d, expected);
    assert_eq!(e, expected);

    assert_eq!(a + b, expected);
    assert_eq!(b + a, expected);
    assert_eq!(a + &b, expected);

    assert_eq!(&b + a, expected);
    assert_eq!(&a + b, expected);

    assert_eq!(&a + b, expected);
    assert_eq!(b + &a, expected);

    assert_eq!(&a + &b, expected);
    assert_eq!(&b + &a, expected);
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(mut self, rhs: &Vec3) -> Self::Output {
        self -= *rhs;
        self
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        *self - rhs
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        -*self
    }
}

#[cfg(test)]
#[test]
#[allow(clippy::op_ref)]
fn test_negate() {
    let a = Vec3::one();
    let b = Vec3::one();
    let c = Vec3(-1., -1., -1.);
    assert_eq!(-a, c);
    assert_eq!(-&a, c);
    assert_eq!(a, b);
    assert_eq!(-a, -b);
    assert_eq!(a - b, Vec3::zero());
    assert_eq!(a - &b, Vec3::zero());
    assert_eq!(b - &a, Vec3::zero());
    assert_eq!(&a - b, Vec3::zero());
    assert_eq!(&b - a, Vec3::zero());
    assert_eq!(&a - &b, Vec3::zero());
    assert_eq!(&b - &a, Vec3::zero());
}

#[cfg(test)]
#[test]
fn test_divide() {
    let a = Vec3(2., 2., 2.);
    let expected = Vec3::one();

    assert_eq!(a / 2., expected);
    assert_eq!(&a / 2., expected);
    let mut b = a;
    b /= 2.;
    assert_eq!(b, expected);
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

#[cfg(test)]
#[test]
fn test_multiply() {
    let a = Vec3::one();
    let expected = Vec3(2., 2., 2.);
    assert_eq!(a * 2., expected);
    assert_eq!(&a * 2., expected);
    assert_eq!(2. * a, expected);
    assert_eq!(2. * &a, expected);
    let mut b = a;
    b *= 2.;
    assert_eq!(b, expected)
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        *self * rhs
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        *rhs * self
    }
}

#[cfg(test)]
/// tests for indexing
/// ```compile_fail
/// let c = Vec3::one();
/// let dne = c.4;
/// ```
mod index_test {
    use super::*;

    #[test]
    fn test_index() {
        let v = Vec3::one();
        assert_eq!(v.0, 1.0);
        assert_eq!(v.1, 1.0);
        assert_eq!(v.2, 1.0);
    }

    #[test]
    fn test_mut() {
        let mut v = Vec3::one();
        v.0 = 0.0;
        v.1 += 1.0;
        assert_eq!(v.0, 0.0);
        assert_eq!(v.1, 2.0);
    }
}
