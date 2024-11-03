pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    fn new() -> Self {
        Self {
            min: -f64::INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn with_bounds(min: f64, max: f64) -> Self {
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

    const EMPTY: Self = Self {
        min: f64::INFINITY,
        max: -f64::INFINITY,
    };

    const UNIVERSE: Self = Self {
        min: -f64::INFINITY,
        max: f64::INFINITY,
    };
}
