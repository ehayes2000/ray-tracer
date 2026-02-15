//! axis aligned bounding box

use crate::{
    EPSILON,
    math::{Axis, Interval, Point, Ray, Vec3},
};

#[derive(Clone, Debug)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn empty() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_corners(a: Point, b: Point) -> Self {
        let x = if a.0 < b.0 {
            Interval::new(a.0, b.0)
        } else {
            Interval::new(b.0, a.0)
        };

        let y = if a.1 < b.1 {
            Interval::new(a.1, b.1)
        } else {
            Interval::new(b.1, a.1)
        };

        let z = if a.2 < b.2 {
            Interval::new(a.2, b.2)
        } else {
            Interval::new(b.2, a.2)
        };

        Self { x, y, z }
    }

    pub fn union_pt(mut self, pt: &Point) -> Self {
        self.x.max = self.x.max.max(pt.0);
        self.x.min = self.x.min.min(pt.0);
        self.y.max = self.y.max.max(pt.1);
        self.y.min = self.y.min.min(pt.1);
        self.z.max = self.z.max.max(pt.2);
        self.z.min = self.z.min.min(pt.2);
        self
    }

    pub fn union(mut self, other: &Self) -> Self {
        self.x = self.x.union(&other.x);
        self.y = self.y.union(&other.y);
        self.z = self.z.union(&other.z);
        self
    }

    pub fn pad(mut self) -> Self {
        if self.x.size() < EPSILON {
            self.x = self.x.expand(EPSILON);
        }
        if self.y.size() < EPSILON {
            self.y = self.y.expand(EPSILON);
        }
        if self.z.size() < EPSILON {
            self.z = self.z.expand(EPSILON);
        }
        self
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<Interval> {
        let mut hit_t = ray_t.to_owned();
        for axis in Axis::iter() {
            let interval = &self[axis];
            let adinv = 1.0 / r.direction[axis];
            let t0 = (interval.min - r.origin[axis]) * adinv;
            let t1 = (interval.max - r.origin[axis]) * adinv;

            if t0 < t1 {
                if t0 > hit_t.min {
                    hit_t.min = t0;
                }
                if t1 < hit_t.max {
                    hit_t.max = t1;
                }
            } else {
                if t1 > hit_t.min {
                    hit_t.min = t1;
                }
                if t0 < hit_t.max {
                    hit_t.max = t0;
                }
            }
            if hit_t.max <= hit_t.min {
                return None;
            }
        }
        Some(hit_t)
    }

    pub fn center(&self) -> Vec3 {
        Vec3(
            self.x.max.midpoint(self.x.min),
            self.y.max.midpoint(self.y.min),
            self.z.max.midpoint(self.z.min),
        )
    }

    pub fn longest(&self) -> Axis {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                Axis::X
            } else {
                Axis::Z
            }
        } else if self.y.size() > self.z.size() {
            Axis::Y
        } else {
            Axis::Z
        }
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Self::empty()
    }
}

impl std::ops::Index<Axis> for Aabb {
    type Output = Interval;
    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}

impl std::ops::IndexMut<Axis> for Aabb {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        match index {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::v3;

    #[test]
    fn bounds() {
        let bbox = Aabb::from_corners(v3!(0, 0, 0), v3!(1, 1, 0));
        assert_eq!(bbox.x.min, 0.);
        assert_eq!(bbox.x.max, 1.);
        assert_eq!(bbox.y.min, 0.);
        assert_eq!(bbox.y.max, 1.);
        assert_eq!(bbox.z.min, 0.);
        assert_eq!(bbox.z.max, 0.);

        let bbox = Aabb::from_corners(v3!(0, 0, 0), v3!(1, 1, 0)).pad();
        assert_eq!(bbox.x.min, 0.);
        assert_eq!(bbox.x.max, 1.);
        assert_eq!(bbox.y.min, 0.);
        assert_eq!(bbox.y.max, 1.);
        assert_eq!(bbox.z.min, -EPSILON / 2.0);
        assert_eq!(bbox.z.max, EPSILON / 2.0);
    }

    #[test]
    fn center() {
        let bbox = Aabb::from_corners(v3!(0, 0, 0), v3!(1, 1, 1));
        assert_eq!(bbox.center(), v3!(0.5, 0.5, 0.5));

        let bbox = Aabb::from_corners(v3!(0, 0, 0), v3!(1, 1, 0));
        assert_eq!(bbox.center(), v3!(0.5, 0.5, 0));
    }

    #[test]
    fn pad() {
        let bbox = Aabb::from_corners(v3!(0, 0, 0), v3!(1, 1, 0)).pad();
        assert_eq!(bbox.center(), v3!(0.5, 0.5, 0));
    }

    #[test]
    fn hit() {
        let r = Ray {
            direction: v3!(0, 0, 1),
            origin: v3!(0.5, 0.5, -1),
        };
        let i = Interval::full();
        let bbox = Aabb::from_corners(v3!(0, 0, 0), v3!(1, 1, 0)).pad();
        assert!(bbox.hit(&r, &i).is_some());
        assert!(bbox.hit(&Ray::zero(), &i).is_none());
        assert!(
            bbox.hit(
                &Ray {
                    direction: v3!(0.01, 1.01, 0),
                    origin: v3!(0.5, 0.5, 0.5)
                },
                &i
            )
            .is_none()
        );

        assert!(
            bbox.hit(
                &Ray {
                    direction: v3!(0, 0, 1),
                    origin: v3!(-2, 0.5, -1)
                },
                &i
            )
            .is_none()
        )
    }
}
