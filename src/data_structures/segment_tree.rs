#![allow(dead_code)]

use crate::utils::algebraic_traits::Monoid;

pub struct SegmentTree<T: Monoid> {
	size: usize,
	tree: Vec<T>
}

impl<T: Monoid> SegmentTree<T> {
	fn new(n0: usize) -> Self {
		let size = n0.next_power_of_two();
		let tree = vec![T::zero(); size*2];
		SegmentTree {
			size, tree
		}
	}

	fn from(vec: &[T]) -> Self {
		let size = vec.len().next_power_of_two();
		let mut tree = vec![T::zero(); size*2];
		for i in 0..vec.len() {
			tree[i + size] = vec[i].clone();
		}
		for i in (0..size).rev() {
			tree[i] = tree[i * 2].clone() + tree[i * 2 + 1].clone();
		}
		SegmentTree {
			size, tree
		}
	}

	fn add(&mut self, mut i: usize, x: T) {
		i += self.size;
		self.tree[i] += x;
		while i > 0 {
			i >>= 1;
			self.tree[i] = self.tree[i * 2].clone() + self.tree[i * 2 + 1].clone();
		}
	}
}
