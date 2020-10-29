#![allow(dead_code)]
use std::ops::Range;
use crate::utils::algebraic_traits::{Element, Monoid};

pub struct SegmentTree<T: Monoid> {
	size: usize,
	node: Vec<T>
}

impl<T: Monoid> SegmentTree<T> {
	fn new(n0: usize) -> Self {
		let size = n0.next_power_of_two();
		let node = vec![T::zero(); size * 2];
		SegmentTree {
			size, node
		}
	}

	fn from(vec: &[T]) -> Self {
		let size = vec.len().next_power_of_two();
		let mut node = vec![T::zero(); size << 1];
		for i in 0..vec.len() {
			node[i + size] = vec[i].clone();
		}
		for i in (1..size).rev() {
			node[i] = node[i << 1].clone() + node[(i << 1) + 1].clone();
		}
		SegmentTree {
			size, node
		}
	}

	pub fn get(&self, i: usize) -> &T { &self.node[i + self.size] }

	pub fn set(&mut self, mut i: usize, x: T) {
		i += self.size;
		self.node[i] = x;
		self.fix(i);
	}

	fn fix(&mut self, mut i: usize) {
        while i > 0 {
			i >>= 1;
			self.node[i] = self.node[i << 1].clone() + self.node[(i << 1) + 1].clone();
		}
    }

	pub fn fold(&self, rng: Range<usize>) -> T {
		let mut vl = T::zero();
		let mut vr = T::zero();
		let mut l = rng.start + self.size;
		let mut r = rng.end + self.size;
		while l < r {
			if l & 1 == 1 {
				vl = vl + self.node[l].clone();
				l += 1;
			}
			if r & 1 == 1 {
				r -= 1;
				vr = self.node[r].clone() + vr;
			}
			l >>= 1;
			r >>= 1;
		}
		vl + vr
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::utils::algebraic_traits::*;
	use std::ops::Add;
	use num_traits::Zero;

	#[derive(Clone, PartialEq)]
    struct Am(usize);

    impl Add for Am {
		type Output = Self;
        fn add(self, right: Self) -> Self { Am(self.0 + right.0) }
	}

    impl Associative for Am {}

    impl Zero for Am {
		fn zero() -> Self { Am(0) }
		fn is_zero(&self) -> bool { self.0 == 0 }
	}

    #[test]
    fn rsq_test() {
        let mut seg = SegmentTree::from(&vec![Am(1), Am(2), Am(3)]);
        assert!(seg.fold(0..2).0 == 3);
        assert!(seg.fold(1..2).0 == 2);
        seg.set(1, Am(5));
        assert!(seg.fold(0..2).0 == 6);
        assert!(seg.fold(1..2).0 == 5);
        seg.set(1, seg.get(1).clone() + Am(5));
        assert!(seg.fold(0..2).0 == 11);
        assert!(seg.fold(1..2).0 == 10);
	}

	#[test]
    fn corner_test() {
        let seg = SegmentTree::from(&vec![Am(1)]);
        assert!(seg.fold(0..1).0 == 1);
    }
}


pub struct SegmentTree2<T: Element, F: Fn(&T, &T) -> T> {
	size: usize,
	node: Vec<T>,
	zero: T,
	func: F
}

impl<T: Element, F: Fn(&T, &T) -> T> SegmentTree2<T, F> {
	fn new(n0: usize, zero: T, func: F) -> Self {
		let size = n0.next_power_of_two();
		let node = vec![zero.clone(); size * 2];
		Self {
			size, node, zero, func
		}
	}

	fn from(vec: &[T], zero: T, func: F) -> Self {
		let size = vec.len().next_power_of_two();
		let mut node = vec![zero.clone(); size << 1];
		for i in 0..vec.len() {
			node[i + size] = vec[i].clone();
		}
		for i in (1..size).rev() {
			node[i] = func(&node[i << 1], &node[(i << 1) + 1]);
		}
		Self {
			size, node, zero, func
		}
	}

	pub fn get(&self, i: usize) -> &T { &self.node[i + self.size] }

	pub fn set(&mut self, mut i: usize, x: T) {
		i += self.size;
		self.node[i] = x;
		self.fix(i);
	}

	fn fix(&mut self, mut i: usize) {
        while i > 0 {
			i >>= 1;
			self.node[i] = (self.func)(&self.node[i << 1], &self.node[(i << 1) + 1]);
		}
    }

	pub fn fold(&self, rng: Range<usize>) -> T {
		let mut vl = self.zero.clone();
		let mut vr = self.zero.clone();
		let mut l = rng.start + self.size;
		let mut r = rng.end + self.size;
		while l < r {
			if l & 1 == 1 {
				vl = (self.func)(&vl, &self.node[l]);
				l += 1;
			}
			if r & 1 == 1 {
				r -= 1;
				vr = (self.func)(&self.node[r], &vr);
			}
			l >>= 1;
			r >>= 1;
		}
		(self.func)(&vl, &vr)
	}
}