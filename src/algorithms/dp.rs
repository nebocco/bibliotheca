use std::ops::Range;
use crate::utils::algebraic_traits::*

pub mod knuth_yao;

pub trait RangeFunc {
	type Output: Element;
	fn len(&self) -> usize;
	fn func(&self, ran: Range<usize>) -> Self::Output;
}

pub trait Monge: RangeFunc {}

pub trait TotallyMonotone {}
impl<T: Monge> TotallyMonotone for T {}

pub trait Monotone {}
impl<T: TotallyMonotone> Monotone for T {}
