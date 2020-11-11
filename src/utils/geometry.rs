pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    pub const EPS: f64 = 0.000_000_001;

    pub fn new<T: Into<f64>>(x: T, y: T) -> Self {
        Self { x: x.into(), y: y.into() }
    }

    pub fn dist2(&self, rhs: &Self) -> f64 {
        ((self.x - rhs.x).powi(2) + (self.y - rhs.y).powi(2)).powf(0.5)
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn cross(&self, p: &Self, q: &Self) -> f64 {
        (p.x - self.x) * (q.y - self.y) - (p.y - self.y) * (q.x - self.x)
    }
}

pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new<T: Into<f64>>(x: T, y: T, r: T) -> Self {
        Self {
            center: Point::new(x, y),
            radius: r.into()
        }
    }
    #[allow(unused_variables)]
    pub fn intersection(&self, rhs: &Self) -> (Option<Point>, Option<Point>) {
        todo!()
    }
}