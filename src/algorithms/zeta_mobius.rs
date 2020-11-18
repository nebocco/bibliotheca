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
pub fn subset_convolution<T: Ring>(a: &[T], b: &[T]) -> Vec<T> {
	assert_eq!(a.len(), b.len(), "given 2 Vecs have different length");
	assert!(a.len().is_power_of_two(), "length of Vec should be power of 2");
	let n = a.len();
	let m = n.trailing_zeros() as usize;
	let mut f = vec![vec![T::zero(); n]; m+1];
	let mut g = vec![vec![T::zero(); n]; m+1];
	for i in 0..n {
		f[i.count_ones() as usize][i] = a[i].clone();
		g[i.count_ones() as usize][i] = b[i].clone();
	}
	for p in (0..m).map(|i| 1 << i) {
		for i in 0..n {
			if i & p != 0 { continue; }
			for k in 0..=m {
				f[k][i|p] = f[k][i|p].clone() + f[k][i].clone();
				g[k][i|p] = g[k][i|p].clone() + g[k][i].clone();
			}
		}
	}
	let mut res = vec![vec![T::zero(); n]; m+1];
	for k in 0..=m {
		for l in 0..=k {
			for i in 0..n {
				res[k][i] += f[l][i].clone() * g[k-l][i].clone();
			}
		}
	}
	for p in (0..m).rev().map(|i| 1 << i) {
		for i in 0..n {
			if i & p != 0 { continue; }
			for k in 0..=m {
				res[k][i|p] = res[k][i|p].clone() + -res[k][i].clone();
			}
		}
	}
	(0..n).map(|i| res[i.count_ones() as usize][i].clone()).collect()
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
	}
}
