use crate::utils::algebraic_traits::{SemiGroup, Group, Ring};

pub fn subset_zeta<T: SemiGroup>(l: &mut Vec<T>) {
	let n = l.len();
	assert!(n.is_power_of_two());
	for p in (0..n.trailing_zeros()).map(|i| 1 << i) {
		for i in 0..n {
			if i & p != 0 {
				l[i] = l[i ^ p].clone() + l[i].clone();
			}
		}
	}
}

pub fn subset_mobius<T: Group>(l: &mut Vec<T>) {
	let n = l.len();
	assert!(n.is_power_of_two());
	for p in (0..n.trailing_zeros()).rev().map(|i| 1 << i) {
		for i in 0..n {
			if i & p != 0 {
				l[i] = -l[i ^ p].clone() + l[i].clone();
			}
		}
	}
}

pub fn superset_zeta<T: SemiGroup>(l: &mut Vec<T>) {
	let n = l.len();
	assert!(n.is_power_of_two());
	for p in (0..n.trailing_zeros()).map(|i| 1 << i) {
		for i in 0..n {
			if i & p == 0 {
				l[i] = l[i].clone() + l[i ^ p].clone();
			}
		}
	}
}

pub fn superset_mobius<T: Group>(l: &mut Vec<T>) {
	let n = l.len();
	assert!(n.is_power_of_two());
	for p in (0..n.trailing_zeros()).rev().map(|i| 1 << i) {
		for i in 0..n {
			if i & p == 0 {
				l[i] = l[i].clone() + -l[i ^ p].clone();
			}
		}
	}
}

/// c[v] = sum _ {i|j = v, i&j = 0} a[i] * b[j];
pub fn subset_convolution<T: Ring + Copy>(a: &[T], b: &[T]) -> Vec<T> {
	assert_eq!(a.len(), b.len(), "given 2 Vecs have different length");
	assert!(a.len().is_power_of_two(), "length of Vec should be power of 2");
	let n = a.len();
	let m = n.trailing_zeros() as usize;
	let mut f = vec![vec![T::zero(); n]; m+1];
	let mut g = vec![vec![T::zero(); n]; m+1];
	for i in 0..n {
		f[i.count_ones() as usize][i] = a[i];
		g[i.count_ones() as usize][i] = b[i];
	}
	for p in (0..m).map(|i| 1 << i) {
		for i in 0..n {
			if i & p != 0 { continue; }
			for k in 0..=m {
				f[k][i|p] = f[k][i|p] + f[k][i];
				g[k][i|p] = g[k][i|p] + g[k][i];
			}
		}
	}
	let mut res = vec![vec![T::zero(); n]; m+1];
	for k in 0..=m {
		for l in 0..=k {
			for i in 0..n {
				res[k][i] += f[l][i] * g[k-l][i];
			}
		}
	}
	for p in (0..m).rev().map(|i| 1 << i) {
		for i in 0..n {
			if i & p != 0 { continue; }
			for k in 0..=m {
				res[k][i|p] = res[k][i|p] + -res[k][i];
			}
		}
	}
	(0..n).map(|i| res[i.count_ones() as usize][i]).collect()
}


macro_rules! define_transform {
	($name: tt, $expr: expr) => {
		pub fn $name(&self, f: &mut [R]) {
			let mut h = 1;
			while h < self.n {
				for chunk in f.chunks_mut(2 * h) {
					let (fst, snd) = chunk.split_at_mut(h);
					fst.iter_mut().zip(snd).for_each($expr);
				}
				h *= 2;
			}
		}
	};
}
macro_rules! define_convolution {
	($name: tt, $transform: tt, $inverse_transform: tt) => {
		pub fn $name(&self, f: &[R], g: &[R]) -> Vec<R> {
		let mut f = f.to_vec();
		let mut g = g.to_vec();
		self.$transform(&mut f);
		self.$transform(&mut g);
		f.iter_mut().zip(g).for_each(|(a, b)| *a = *a * b);
		self.$inverse_transform(&mut f);
		f
		}
	};
}

pub struct SubsetConvolution<R> {
	n: usize,
	depth: usize,
	pct: Vec<Vec<usize>>,
	phantom: std::marker::PhantomData<R>
}

impl<R: Ring + Copy> SubsetConvolution<R> {
	#[inline]
	pub fn new(n: usize) -> Self {
		let depth = n.trailing_zeros() as usize;
		let mut pct = vec![Vec::new(); depth+1];
		(0..n).for_each(|i| { pct[i.count_ones() as usize].push(i); });
		Self {
			n, depth, pct, phantom: std::marker::PhantomData
		}
	}

	// Walsh transform.
	define_transform!(walsh_transform, |(x, y)| {
		let (u, v) = (*x, *y);
		*x = u + v;
		*y = u + -v;
	});
	// Arithmetic Transform (Plus), a.k.a., the Mobius transform.
	define_transform!(arithmetic_transform_plus, |(x, y)| *y += *x);

	// Arithmetic Transform (Minus), a.k.a., the Inverse Mobius transform.
	define_transform!(arithmetic_transform_minus, |(x, y)| *y += -*x);

	// Arithmetic Transform (Plus), a.k.a., the Mobius transform.
	define_transform!(reverse_arithmetic_transform_plus, |(x, y)| *x += *y);

	// Arithmetic Transform (Minus), a.k.a., the Inverse Mobius transform.
	define_transform!(reverse_arithmetic_transform_minus, |(x, y)| *x += -*y);

	// Or-convolution (a.k.a. Covering product)
	// h[X] = \sum_{S, T: S \cup T = X} f[S] g[T].
	define_convolution!(
		or_convolution,
		arithmetic_transform_plus,
		arithmetic_transform_minus
	);

	// And-convolution (a.k.a. Packing product)
	// h[X] = \sum_{S, T: S \cap T = X} f[S] g[T].
	define_convolution!(
		and_convolution,
		reverse_arithmetic_transform_plus,
		reverse_arithmetic_transform_minus
	);

	// Xor-convolution
	// h[X] = \sum_{S, T: T xor S = X} f[S] g[T].
	define_convolution!(
		xor_convolution,
		walsh_transform,
		walsh_transform
	);

	/// Subset-convolution
	/// h[X] = sum_{S \subseteq X} f[S] g[X-S]
	pub fn subset_convolution(&self, f: &[R], g: &[R]) -> Vec<R> {
		let mut f_ranked = vec![vec![R::zero(); self.n]; self.depth + 1];
		let mut g_ranked = vec![vec![R::zero(); self.n]; self.depth + 1];

		for (k, list) in self.pct.iter().enumerate() {
			list.iter().for_each(|&i| {
				f_ranked[k][i] = f[i];
				g_ranked[k][i] = g[i];
			});
		}
		f_ranked
			.iter_mut()
			.for_each(|v| self.arithmetic_transform_plus(v));
		g_ranked
			.iter_mut()
			.for_each(|v| self.arithmetic_transform_plus(v));

		let mut h = vec![R::zero(); self.n];
		for (k, list) in self.pct.iter().enumerate() {
			let mut h_ranked = vec![R::zero(); self.n];
			for j in 0..=k {
				h_ranked
					.iter_mut()
					.zip(f_ranked[j].iter().zip(g_ranked[k - j].iter()))
					.for_each(|(h, (f, g))| { *h += *f * *g; });
			}
			self.arithmetic_transform_minus(&mut h_ranked);
			list.iter().for_each(|&i| { h[i] = h_ranked[i] });
		}
		h
	}
}

#[cfg(test)]
mod tests {
	// TODO: make tests

	use super::*;

    #[test]
    fn it_works() {
		assert_eq!(2 + 2, 4);
	}

	#[test]
	fn test_subset_conv() {
		let a = vec![1, 2, 3, 4, 5, 6, 7, 8];
		let b = vec![9, 10, 11, 12, 13, 14, 15, 16];
		let c = subset_convolution(&a, &b);
		assert_eq!(c, vec![9, 28, 38, 100, 58, 144, 172, 408]);
		let s = SubsetConvolution::new(a.len());
		let d = s.subset_convolution(&a, &b);
		assert_eq!(d, vec![9, 28, 38, 100, 58, 144, 172, 408]);
	}
}
