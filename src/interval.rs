pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn empty() -> Self {
        Self {
            min: f32::MAX,
            max: f32::MIN,
        }
    }
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }
    pub fn size(&self) -> f32 {
        self.max - self.min
    }
    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
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
    min: f32::MAX,
    max: f32::MIN,
};

pub static UNIVERSE_INTERVAL: Interval = Interval {
    min: f32::MIN,
    max: f32::MAX,
};
