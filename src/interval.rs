pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f32::MAX,
            max: f32::MIN,
        }
    }
}

impl Interval {
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
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }

    pub const UNIVERSE: Interval = Interval {
        min: f32::MIN,
        max: f32::MAX,
    };

    pub const EMPTY: Interval = Interval {
        min: f32::MAX,
        max: f32::MIN,
    };
}

pub fn interval(min: f32, max: f32) -> Interval {
    Interval { min, max }
}
