pub mod lazy_segment_tree;

use crate::utils::{
	algebraic_traits::{ Element, Monoid },
	bounds::bounds_within,
};

use std::ops::{ Index, Range, RangeBounds };

// * verified: https://judge.yosupo.jp/submission/28323, https://judge.yosupo.jp/submission/28333
// ------------ Segment Tree start ------------

pub struct SegmentTree<T: Monoid> {
    n: usize,
	size: usize,
	node: Vec<T>
}

impl<T: Monoid> SegmentTree<T> {
	pub fn new(n: usize) -> Self {
		let size = n.next_power_of_two();
		let node = vec![T::zero(); size * 2];
		SegmentTree {
			n, size, node
		}
	}

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

	pub fn fold<R: RangeBounds<usize>>(&self, rng: R) -> T {
		let Range { start, end } = bounds_within(rng, self.size);
		let mut vl = T::zero();
		let mut vr = T::zero();
		let mut l = start + self.size;
		let mut r = end + self.size;
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

    /// (j, t) => pred(j-1) = true, pred(j) = false
    pub fn partition(&self, pred: impl Fn(usize, &T) -> bool) -> (usize, T) {
        assert!(pred(0, &T::zero()), "need to be pred(0, T::zero())");
        if pred(self.n - 1, &self.node[1]) {
            return (self.n - 1, self.node[1].clone())
        }
        let mut j = 1;
        let mut current = T::zero();
        let mut idx = 0;
        let mut f = self.size;
        while j < self.size {
            j <<= 1;
            f >>= 1;
            let next = current.clone() + self.node[j].clone();
            if pred(idx + f - 1, &next) {
                current = next;
                j |= 1;
                idx += f;
            }
        }
        (idx, current)
    }
}

impl<T: Monoid> From<Vec<T>> for SegmentTree<T> {
	fn from(vec: Vec<T>) -> Self {
        let n = vec.len();
		let size = n.next_power_of_two();
		let mut node = vec![T::zero(); size << 1];
		for (i, e) in vec.iter().cloned().enumerate() {
			node[i + size] = e;
		}
		for i in (1..size).rev() {
			node[i] = node[i << 1].clone() + node[(i << 1) + 1].clone();
		}
		SegmentTree {
			n, size, node
		}
	}
}

impl<T: Monoid> Index<usize> for SegmentTree<T> {
	type Output = T;
	fn index(&self, i: usize) -> &Self::Output {
		assert!(i < self.size, "index out of range: length is {}, but given {}.", self.size, i);
		&self.node[i + self.size]
	}
}

// ------------ Segment Tree end ------------

// ------------ Segment Tree with function start ------------

pub struct SegmentTree2<T: Element, F: Fn(&T, &T) -> T> {
	size: usize,
	node: Vec<T>,
	zero: T,
	func: F
}

impl<T: Element, F: Fn(&T, &T) -> T> SegmentTree2<T, F> {
	pub fn new(n0: usize, zero: T, func: F) -> Self {
		let size = n0.next_power_of_two();
		let node = vec![zero.clone(); size * 2];
		Self {
			size, node, zero, func
		}
	}

	pub fn from(vec: &[T], zero: T, func: F) -> Self {
		let size = vec.len().next_power_of_two();
		let mut node = vec![zero.clone(); size << 1];
		node[size..(vec.len() + size)].clone_from_slice(&vec[..]);
		for i in (1..size).rev() {
			node[i] = func(&node[i << 1], &node[(i << 1) + 1]);
		}
		Self {
			size, node, zero, func
		}
	}

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

	pub fn fold<R: RangeBounds<usize>>(&self, rng: R) -> T {
		let Range { start, end } = bounds_within(rng, self.size);
		let mut vl = self.zero.clone();
		let mut vr = self.zero.clone();
		let mut l = start + self.size;
		let mut r = end + self.size;
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

impl<T: Element, F: Fn(&T, &T) -> T> Index<usize> for SegmentTree2<T, F> {
	type Output = T;
	fn index(&self, i: usize) -> &Self::Output {
		assert!(i < self.size, "index out of range: length is {}, but given {}.", self.size, i);
		&self.node[i + self.size]
	}
}

// ------------ Segment Tree with function end ------------


#[cfg(test)]
mod tests {
	use super::*;
	use crate::utils::algebraic_traits::*;
	use std::ops::Add;

	#[derive(Clone, PartialEq)]
    struct Am(usize);

    impl Add for Am {
		type Output = Self;
        fn add(self, right: Self) -> Self { Am(self.0.min(right.0)) }
	}

    impl Associative for Am {}

    impl Zero for Am {
		fn zero() -> Self { Am(std::usize::MAX) }
		fn is_zero(&self) -> bool { self.0 == std::usize::MAX }
	}

    #[test]
    fn rmq_test() {
		let vec = vec![Am(1), Am(2), Am(3)];
        let mut seg = SegmentTree::from(vec);
        assert!(seg.fold(0..2).0 == 1);
        assert!(seg.fold(1..2).0 == 2);
        seg.set(1, Am(5));
        assert!(seg.fold(0..2).0 == 1);
        assert!(seg.fold(2..3).0 == 3);
        seg.set(2, Am(0));
        assert!(seg.fold(0..2).0 == 1);
        assert!(seg.fold(1..3).0 == 0);
	}

    #[test]
	fn i32_test() {
		let vec = vec![1, 2, 3];
        let mut seg = SegmentTree::from(vec);
        assert!(seg.fold(0..2) == 3);
        assert!(seg.fold(1..2) == 2);
        seg.set(1, 5);
        assert!(seg.fold(0..2) == 6);
        assert!(seg.fold(1..2) == 5);
        seg.set(1, seg[1].clone() + 5);
        assert!(seg.fold(0..2) == 11);
        assert!(seg.fold(1..2) == 10);
	}

	#[test]
    fn corner_test() {
        let seg = SegmentTree::from(vec![Am(1)]);
        assert!(seg.fold(0..1).0 == 1);
    }
}