use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::math::random_f64;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);
pub type Point = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn len(&self) -> f64 {
        f64::sqrt(self.len_squared())
    }

    pub fn len_squared(&self) -> f64 {
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
                return v / f64::sqrt(lensq);
            }
        }
    }

    pub fn random_mm(min: f64, max: f64) -> Self {
        Vec3(
            random_f64(min, max),
            random_f64(min, max),
            random_f64(min, max),
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

    pub fn near_zero(&self) -> bool {
        let e = 1e-8;
        self.0.abs() < e && self.1.abs() < e && self.2.abs() < e
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        self - 2. * dot(self, normal) * normal
    }

    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = f64::min(dot(&-self, n), 1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.len_squared())) * n;
        r_out_perp + r_out_parallel
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.5} {:.5} {:.5}", self.0, self.1, self.2)
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    (u.0 * v.0) + (u.1 * v.1) + (u.2 * v.2)
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
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

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
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
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        *self * rhs
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        *rhs * self
    }
}

#[macro_export]
macro_rules! v3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3($x as f64, $y as f64, $z as f64)
    };
}
