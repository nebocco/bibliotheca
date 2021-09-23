pub mod dinic;
pub mod network_simplex;
pub mod primal_dual;

use crate::utils::algebraic_traits::{Element, One, Zero};
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use std::fmt::Display;

pub trait Cost:
    Element
    + Display
    + Clone
    + Copy
    + Eq
    + Ord
    + Zero
    + One
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
{
    fn is_positive(&self) -> bool {
        self > &Self::zero()
    }
    fn is_negative(&self) -> bool {
        self < &Self::zero()
    }

    const MAX: Self;
}

pub trait Flow: Cost + SubAssign {
    fn abs(&self) -> Self {
        if self.is_negative() {
            -*self
        } else {
            *self
        }
    }
}

macro_rules! impl_flow {
    ($($T:ident,)*) => {
		$(
            impl Flow for $T {}

			impl Cost for $T {
                const MAX: Self = std::$T::MAX;
            }
		)*
    };
}

impl_flow!(i8, i16, i32, i64, i128, isize,);
