//! axis aligned bounding box

use crate::{
    interval::Interval,
    math::{Point, Ray},
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
        let epsilon = 0.001;
        if self.x.size() < epsilon {
            self.x = self.x.expand(epsilon);
        }
        if self.y.size() < epsilon {
            self.y = self.y.expand(epsilon);
        }
        if self.z.size() < epsilon {
            self.z = self.z.expand(epsilon);
        }
        self
    }

    pub fn axis(&self, a: usize) -> &Interval {
        match a {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("axis out of bounds"),
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<Interval> {
        let mut hit_t = ray_t.to_owned();
        for axis in 0..3 {
            let ax = self.axis(axis);
            let adinv = 1.0 / r.direction[axis];
            let t0 = (ax.min - r.origin[axis]) * adinv;
            let t1 = (ax.max - r.origin[axis]) * adinv;

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
}

impl Default for Aabb {
    fn default() -> Self {
        Self::empty()
    }
}
