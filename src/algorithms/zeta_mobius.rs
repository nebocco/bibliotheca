use crate::utils::algebraic_traits::{Group, SemiGroup};

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