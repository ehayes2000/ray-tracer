use crate::Float;

#[derive(Debug, Clone)]
pub struct Interval {
    pub min: Float,
    pub max: Float,
}

impl Interval {
    pub fn empty() -> Self {
        Self {
            min: Float::MAX,
            max: Float::MIN,
        }
    }

    pub fn full() -> Self {
        Self {
            min: Float::MIN,
            max: Float::MAX,
        }
    }

    pub fn new(min: Float, max: Float) -> Self {
        Self { min, max }
    }

    pub fn expand(self, delta: Float) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub fn size(&self) -> Float {
        self.max - self.min
    }
    pub fn contains(&self, x: Float) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: Float) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: Float) -> Float {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn union(mut self, other: &Self) -> Self {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self
    }

    pub fn center(&self) -> Float {
        self.min + (self.size() / 2.0)
    }
}
