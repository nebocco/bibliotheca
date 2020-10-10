#![allow(dead_code)]

use crate::utils::algebraic_traits::*;

pub struct SegmentTree<T: Monoid + Copy> {
	size: usize,
	tree: Vec<T>
}

impl<T: Monoid + Copy> SegmentTree<T> {
	fn new(n0: usize) -> Self {
		let size = n0.next_power_of_two();
		let tree = vec![T::zero(); size * 2];
		SegmentTree {
			size, tree
		}
	}

	fn from(vec: &[T]) -> Self {
		let size = vec.len().next_power_of_two();
		let mut tree = vec![T::zero(); size * 2];
		for i in 0..vec.len() {
			tree[i + size] = vec[i];
		}
		for i in (0..size).rev() {
			tree[i] = tree[i << 1] + tree[(i << 1) + 1];
		}
		SegmentTree {
			size, tree
		}
	}

	fn add(&mut self, mut i: usize, x: T) {
		i += self.size;
		self.tree[i] = self.tree[i] + x;
		while i > 0 {
			i >>= 1;
			self.tree[i] = self.tree[i << 1] + self.tree[(i << 1) + 1];
		}
	}

	fn update(&mut self, mut i: usize, x: T) {
		i += self.size;
		self.tree[i] = x;
		while i > 0 {
			i >>= 1;
			self.tree[i] = self.tree[i << 1] + self.tree[(i << 1) + 1];
		}
	}

	fn query(&self, l0: usize, r0:usize) -> T {
		let mut vl = T::zero();
		let mut vr = T::zero();
		let mut l = l0 + self.size;
		let mut r = r0 + self.size;
		while l < r {
			if l & 1 > 0 {
				vl = vl + self.tree[l];
				l += 1;
			}
			if r & 1 > 0 {
				r -= 1;
				vr = self.tree[r] + vr;
			}
			l >>= 1;
			r >>= 1;
		}
		vl + vr
	}
}
