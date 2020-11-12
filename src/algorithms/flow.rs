pub mod dinic;
pub mod network_simplex;

use crate::utils::algebraic_traits::{ Element, Zero, One };

use std::fmt::Display;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

pub trait Cost:
	Element
	+ Display
    + Copy
    + Eq
    + Ord
    + Zero
    + One
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + Neg<Output = Self>
{
    fn is_positive(&self) -> bool {
        self > &Self::zero()
    }
    fn is_negative(&self) -> bool {
        self < &Self::zero()
    }
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
    ($($T:ty,)*) => {
		$(
			impl Flow for $T {}
			impl Cost for $T {}
		)*
    };
}

impl_flow!(
	i8, i16, i32, i64, i128, isize,
);