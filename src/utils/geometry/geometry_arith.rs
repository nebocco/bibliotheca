use super::Point;
use std::ops::{Add, Sub, Mul, Div};
use crate::utils::algebraic_traits::Zero;

impl PartialEq for Point {
    fn eq(&self, rhs: &Self) -> bool {
        (self.x - rhs.x).abs() < Self::EPS &&
        (self.y - rhs.y).abs() < Self::EPS
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Mul for Point {
    type Output = Point;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x - self.y * rhs.y,
            y: self.x * rhs.y + self.y * rhs.x,
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for Point {
    type Output = Point;
    fn div(self, rhs: Self) -> Self {
		assert!(!rhs.is_zero(), "ゼロベクトルで割ろうとしていませんか？");
		let d = rhs.x.powi(2) + rhs.y.powi(2);
        Self {
            x: (self.x * rhs.x - self.y * -rhs.y) / d,
            y: (self.x * -rhs.y + self.y * rhs.x) / d
        }
    }
}

impl Zero for Point {
    fn zero() -> Self {
        Self { x: 0., y: 0. }
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl<'a> Zero for &'a Point {
    fn zero() -> &'a Point {
        &Point { x: 0., y: 0. }
    }

    fn is_zero(&self) -> bool {
        *self == &Point::zero()
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