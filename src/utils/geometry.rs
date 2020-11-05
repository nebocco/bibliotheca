pub struct Point {
    x: i64,
    y: i64
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn dist(&self, rhs: &Self) -> i64 {
        (self.x - rhs.x).pow(2) + (self.y - rhs.y).pow(2)
    }

    pub fn dot(&self, rhs: &Self) -> i64 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn cross(&self, p: &Self, q: &Self) -> i64 {
        (p.x - self.x) * (q.y - self.y) - (p.y - self.y) * (q.x - self.x)
    }
}