#![allow(dead_code)]
use crate::utils::algebraic_traits::Element;

struct DoublePriorityHeap<T: Element + Ord>(Vec<T>);

impl<T: Element + Ord> DoublePriorityHeap<T> {
	pub fn new() -> Self {
		Self(Vec::new())
	}

	pub fn from(vec: &[T]) -> Self {
		let mut l = Self(vec.to_vec());
		l.build();
		l
	}

	pub fn push(&mut self, x: &T) {
		self.0.push(x.clone());
		self.up(self.0.len() - 1, 1);
	}

	pub fn pop_min(&mut self) -> Option<T> {
		if self.0.len() < 3 {
			self.0.pop()
		} else {
			let n = self.0.len();
			self.0.swap(1, n - 1);
			let ret = self.0.pop();
			let k = self.down(1);
			self.up(k, 1);
			ret
		}
	}

	pub fn pop_max(&mut self) -> Option<T> {
		if self.0.len() < 2 {
			self.0.pop()
		} else {
			let n = self.0.len();
			self.0.swap(1, n - 1);
			let ret = self.0.pop();
			let k = self.down(0);
			self.up(k, 1);
			ret
		}
	}

	pub fn get_min(&self) -> Option<&T> {
		if self.0.len() < 2 {
			self.0.get(0)
		} else {
			self.0.get(1)
		}
	}

	pub fn get_max(&self) -> Option<&T> {
		self.0.get(0)
	}

	fn build(&mut self) {
		let n = self.0.len();
		for i in (0..n).rev() {
			if i & 1 == 1 && self.0[i-1] < self.0[i] {
				self.0.swap(i-1, i);
			}
			let k = self.down(i);
			self.up(k, i);
		}
	}

	#[inline]
	fn parent(k: usize) -> usize {
		(k >> 1).wrapping_sub(1) & 1usize.wrapping_neg()
	}

	fn down(&mut self, mut k: usize) -> usize {
		let n = self.0.len();
		if k & 1 == 1 {
			while 2 * k + 1 < n {
				let mut c = 2 * k + 4;
				if n <= c || self.0[c-2] < self.0[c] {
					c -= 2;
				}
				if c < n && self.0[c] < self.0[k] {
					self.0.swap(c, k);
					k = c;
				} else {
					break
				}
			}
		}
		k
	}

	fn up(&mut self, mut k: usize, root: usize) {
		if (k | 1) < self.0.len() && self.0[k & 1usize.wrapping_neg()] < self.0[k | 1] {
			self.0.swap(k & 1usize.wrapping_neg(), k | 1);
			k ^= 1;
		}
		let mut p = Self::parent(k);
		while root < k && self.0[p] < self.0[k] {
			self.0.swap(k, p);
			k = p;
			p = Self::parent(k)
		}
		p |= 1;
		while root < k && self.0[k] < self.0[p] {
			self.0.swap(k, p);
			k = p;
			p = Self::parent(k) | 1;
		}
	}
}