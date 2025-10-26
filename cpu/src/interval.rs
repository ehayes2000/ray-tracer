pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn empty() -> Self {
        Self {
            min: f64::MAX,
            max: f64::MIN,
        }
    }
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

pub static EMPTY_INTERVAL: Interval = Interval {
    min: f64::MAX,
    max: f64::MIN,
};

pub static UNIVERSE_INTERVAL: Interval = Interval {
    min: f64::MIN,
    max: f64::MAX,
};
