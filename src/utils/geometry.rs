mod geometry_arith;
use crate::utils::algebraic_traits::Zero;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    pub const EPS: f64 = 0.000_000_001;

    pub fn new<T: Into<f64>>(x: T, y: T) -> Self {
        Self { x: x.into(), y: y.into() }
    }

    pub fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).powf(0.5)
    }

    pub fn dist(&self, rhs: &Self) -> f64 {
        (self - rhs).norm()
    }

    pub fn unit(&self) -> Self {
        assert!(!self.is_zero(), "ゼロベクトルに法線はありませんよ？");
        let d = self.norm();
        Self { x: self.x / d, y: self.y / d }
    }

    pub fn normal(&self) -> Self {
        Self{ x: -self.y, y: self.x }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn cross(&self, rhs: &Self) -> f64 {
        self.x * rhs.y - self.y * rhs.x
    }

    pub fn area(&self, p: &Self, q: &Self) -> f64 {
        (p - self).cross(&(q - self))
    }

    pub fn rotate(&self, theta: f64) -> Self {
        Self {
            x: self.x * theta.cos() - self.y * theta.sin(),
            y: self.x * theta.sin() + self.y * theta.cos(),
        }
    }

}

pub struct Line(Point, Point);

impl Line {
    pub fn new(p: Point, q: Point) -> Self {
        Self(p, q)
    }

    /// a * x + b * y = c
    pub fn from_equation(a: f64, b: f64, c: f64) -> Self {
        assert!(a.abs() < Point::EPS || a.abs() < Point::EPS, "不当な式ではありませんか？");
        if a.abs() < Point::EPS {
            Self(Point::new(0., c / b), Point::new(1., c / b))
        } else if b.abs() < Point::EPS {
            Self(Point::new(c / a, 0.), Point::new(c / a, 1.))
        } else {
            Self(Point::new(0., c / b), Point::new(c / a, 0.))
        }
    }

    pub fn projection(&self, p: &Point) -> Point {
        let t = (p - self.0).dot(&(self.0 - self.1)) / (self.0 - self.1).norm();
        self.0 + (self.0 - self.1) * Point::new(t, 0.)
    }

    pub fn reflection(&self, p: &Point) -> Point {
        p + (self.projection(p) - p) * Point::new(2., 0.)
    }

    pub fn is_orthogonal(&self, rhs: &Self) -> bool {
        (self.1 - self.0).dot(&(rhs.1 - rhs.0)) < Point::EPS
    }

    pub fn is_parallel(&self, rhs: &Self) -> bool {
        (self.1 - self.0).cross(&(rhs.1 - rhs.0)) < Point::EPS
    }

    pub fn crosspoint(&self, rhs: &Self) -> Option<Point> {
        let d1 = (self.1 - self.0).cross(&(rhs.1 - rhs.0));
        let d2 = (self.1 - self.0).cross(&(rhs.1 - rhs.0));
        if self.is_parallel(rhs) {
            if d1.abs() < Point::EPS && d2.abs() < Point::EPS {
                Some(self.0)
            } else {
                None
            }
        } else {
            Some(rhs.0 + (rhs.1 - rhs.0) * Point::new(d2 / d1, 0.))
        }

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