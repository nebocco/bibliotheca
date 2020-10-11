#![allow(dead_code)]

use crate::utils::algebraic_traits::*;

pub struct SegmentTree<T: Monoid> {
	size: usize,
	tree: Vec<T>
}

impl<T: Monoid> SegmentTree<T> {
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
			tree[i + size] = vec[i].clone();
		}
		for i in (0..size).rev() {
			tree[i] = tree[i << 1].clone() + tree[(i << 1) + 1].clone();
		}
		SegmentTree {
			size, tree
		}
	}

	fn add(&mut self, mut i: usize, x: T) {
		i += self.size;
		self.tree[i] = self.tree[i].clone() + x;
		while i > 0 {
			i >>= 1;
			self.tree[i] = self.tree[i << 1].clone() + self.tree[(i << 1) + 1].clone();
		}
	}

	fn update(&mut self, mut i: usize, x: T) {
		i += self.size;
		self.tree[i] = x;
		while i > 0 {
			i >>= 1;
			self.tree[i] = self.tree[i << 1].clone() + self.tree[(i << 1) + 1].clone();
		}
	}

	fn query(&self, l0: usize, r0:usize) -> T {
		let mut vl = T::zero();
		let mut vr = T::zero();
		let mut l = l0 + self.size;
		let mut r = r0 + self.size;
		while l < r {
			if l & 1 > 0 {
				vl = vl + self.tree[l].clone();
				l += 1;
			}
			if r & 1 > 0 {
				r -= 1;
				vr = self.tree[r].clone() + vr;
			}
			l >>= 1;
			r >>= 1;
		}
		vl + vr
	}
}


pub struct SegmentTree2<T, F> {
	size: usize,
	tree: Vec<T>,
	func: F
}

impl<T, F> SegmentTree2<T, F> where
	T: Monoid, F: Fn(T, T) -> T
{
	fn new(n0: usize, func: F) -> Self {
		let size = n0.next_power_of_two();
		let tree = vec![T::zero(); size * 2];
		SegmentTree2 {
			size, tree, func
		}
	}

	fn from(vec: &[T], func: F) -> Self {
		let size = vec.len().next_power_of_two();
		let mut tree = vec![T::zero(); size * 2];
		for i in 0..vec.len() {
			tree[i + size] = vec[i].clone();
		}
		for i in (0..size).rev() {
			tree[i] = func(tree[i << 1].clone(), tree[(i << 1) + 1].clone());
		}
		SegmentTree2 {
			size, tree, func
		}
	}

	fn add(&mut self, mut i: usize, x: T) {
		i += self.size;
		self.tree[i] = (self.func)(self.tree[i].clone(), x);
		while i > 0 {
			i >>= 1;
			self.tree[i] = (self.func)(self.tree[i << 1].clone(), self.tree[(i << 1) + 1].clone());
		}
	}

	fn update(&mut self, mut i: usize, x: T) {
		i += self.size;
		self.tree[i] = x;
		while i > 0 {
			i >>= 1;
			self.tree[i] = (self.func)(self.tree[i << 1].clone(), self.tree[(i << 1) + 1].clone());
		}
	}

	fn query(&self, l0: usize, r0:usize) -> T {
		let mut vl = T::zero();
		let mut vr = T::zero();
		let mut l = l0 + self.size;
		let mut r = r0 + self.size;
		while l < r {
			if l & 1 > 0 {
				vl = (self.func)(vl, self.tree[l].clone());
				l += 1;
			}
			if r & 1 > 0 {
				r -= 1;
				vr = (self.func)(self.tree[r].clone(), vr);
			}
			l >>= 1;
			r >>= 1;
		}
		vl + vr
	}
}

#[cfg(test)]
mod tests {
    // TODO: make tests
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}