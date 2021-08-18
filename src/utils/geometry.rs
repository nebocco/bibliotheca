use crate::utils::algebraic_traits::Zero;
use std::ops::{Add, Div, Mul, Sub};

// ------------ geometry start ------------

#[derive(Clone, Copy)]
pub struct Point(f64, f64);

impl Point {
    pub const EPS: f64 = 0.000_000_001;

    pub fn new<T: Into<f64>>(x: T, y: T) -> Self {
        Self(x.into(), y.into())
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.0
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.1
    }

    #[inline]
    pub fn arg(&self) -> f64 {
        self.1.atan2(self.0)
    }

    #[inline]
    pub fn norm(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2)).powf(0.5)
    }

    #[inline]
    pub fn dist(&self, rhs: &Self) -> f64 {
        (self - rhs).norm()
    }

    #[inline]
    pub fn unit(&self) -> Self {
        assert!(!self.is_zero(), "ゼロベクトルに法線はありませんよ？");
        let d = self.norm();
        Self(self.0 / d, self.1 / d)
    }

    #[inline]
    pub fn normal(&self) -> Self {
        Self(-self.1, self.0)
    }

    #[inline]
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }

    #[inline]
    pub fn cross(&self, rhs: &Self) -> f64 {
        self.0 * rhs.1 - self.1 * rhs.0
    }

    #[inline]
    pub fn area(&self, p: &Self, q: &Self) -> f64 {
        (p - self).cross(&(q - self))
    }

    #[inline]
    pub fn rotate(&self, theta: f64) -> Self {
        Self(
            self.0 * theta.cos() - self.1 * theta.sin(),
            self.0 * theta.sin() + self.1 * theta.cos(),
        )
    }
}

pub struct Line(Point, Point);

impl Line {
    pub fn new(p: Point, q: Point) -> Self {
        Self(p, q)
    }

    /// a * x + b * y = c
    pub fn from_equation(a: f64, b: f64, c: f64) -> Self {
        assert!(
            a.abs() < Point::EPS || a.abs() < Point::EPS,
            "不当な式ではありませんか？"
        );
        if a.abs() < Point::EPS {
            Self(Point::new(0., c / b), Point::new(1., c / b))
        } else if b.abs() < Point::EPS {
            Self(Point::new(c / a, 0.), Point::new(c / a, 1.))
        } else {
            Self(Point::new(0., c / b), Point::new(c / a, 0.))
        }
    }

    #[inline]
    pub fn projection(&self, p: &Point) -> Point {
        self.0
            + (self.0 - self.1)
                * Point::new(
                    (p - self.0).dot(&(self.0 - self.1)) / (self.0 - self.1).norm(),
                    0.,
                )
    }

    #[inline]
    pub fn reflection(&self, p: &Point) -> Point {
        p + (self.projection(p) - p) * Point::new(2., 0.)
    }

    #[inline]
    pub fn is_orthogonal(&self, rhs: &Self) -> bool {
        (self.1 - self.0).dot(&(rhs.1 - rhs.0)) < Point::EPS
    }

    #[inline]
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
            radius: r.into(),
        }
    }
    #[allow(unused_variables)]
    pub fn intersection(&self, rhs: &Self) -> (Option<Point>, Option<Point>) {
        todo!()
    }
}

// ------------ impl arith start ------------

use std::fmt;

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.6}, {:.6})", self.x(), self.y())
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.6}, {:.6})", self.x(), self.y())
    }
}

impl PartialEq for Point {
    fn eq(&self, rhs: &Self) -> bool {
        (self.0 - rhs.0).abs() < Self::EPS && (self.1 - rhs.1).abs() < Self::EPS
    }
}

impl Eq for Point {}

impl PartialOrd for Point {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Point {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        if let Some(v) = self.arg().partial_cmp(&rhs.arg()) {
            v
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul for Point {
    type Output = Point;
    fn mul(self, rhs: Self) -> Self {
        Self(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for Point {
    type Output = Point;
    fn div(self, rhs: Self) -> Self {
        assert!(!rhs.is_zero(), "ゼロベクトルで割ろうとしていませんか？");
        let d = rhs.0.powi(2) + rhs.1.powi(2);
        Self(
            (self.0 * rhs.0 - self.1 * -rhs.1) / d,
            (self.0 * -rhs.1 + self.1 * rhs.0) / d,
        )
    }
}

impl Zero for Point {
    fn zero() -> Self {
        Self(0., 0.)
    }
}

impl<'a> Zero for &'a Point {
    fn zero() -> &'a Point {
        &Point(0., 0.)
    }
}

macro_rules! binop_ref {
    ($(impl $imp:ident, $method:ident)*) => {
        $(
            impl<'a> $imp<Point> for &'a Point {
                type Output = Point;
                fn $method(self, other: Point) -> Self::Output {
                    $imp::$method(*self, other)
                }
            }

            impl<'a> $imp<&'a Point> for Point {
                type Output = Point;
                fn $method(self, other: &Point) -> Self::Output {
                    $imp::$method(self, *other)
                }
            }

            impl<'a> $imp<&'a Point> for &'a Point {
                type Output = Point;
                fn $method(self, other: &Point) -> Self::Output {
                    $imp::$method(*self, *other)
                }
            }
        )*
    };
}

binop_ref! {
    impl Add, add
    impl Sub, sub
    impl Mul, mul
    impl Div, div
}

// ------------ impl arith end ------------
// ------------ geometry end ------------
