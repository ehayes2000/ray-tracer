use super::Float;

#[derive(Debug, Clone)]
pub struct Interval {
    pub min: Float,
    pub max: Float,
}

impl Interval {
    pub const fn empty() -> Self {
        Self {
            min: Float::MAX,
            max: Float::MIN,
        }
    }

    pub const fn full() -> Self {
        Self {
            min: 0.0,
            max: Float::MAX,
        }
    }

    pub const fn new(min: Float, max: Float) -> Self {
        Self { min, max }
    }

    pub const fn expand(self, delta: Float) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub const fn size(&self) -> Float {
        self.max - self.min
    }
    pub const fn contains(&self, x: Float) -> bool {
        self.min <= x && x <= self.max
    }

    pub const fn surrounds(&self, x: Float) -> bool {
        self.min < x && x < self.max
    }

    pub const fn clamp(&self, x: Float) -> Float {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub const fn union(mut self, other: &Self) -> Self {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self
    }

    pub const fn center(&self) -> Float {
        self.min + (self.size() / 2.0)
    }
}
