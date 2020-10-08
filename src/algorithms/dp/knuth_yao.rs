use std::ops::Add;
use crate::utils::algebraic_traits::Magma;
use crate::algorithms::dp::*;

pub fn speed_up<F: Monge>(init: &[F::Output], f: &F) -> Vec<Vec<Option<F::Output>>>
where F::Output: Magma {
	let n = f.len();
	let mut dp: Vec<Vec<Option<F::Output>>> = (0..n).map(|i| vec[None; n - i]).collect::<Vec<_>>();
	let mut k: Vec<Vec<usize>> = (0..n).map(|i| vec[0; n - i]).collect::<Vec<_>>();

	for i in 0..n {
		dp[i][0] = Some(init[i]);
	}

	for d in 1..n {
		for i in 0..n-d {
			k[i][d] = (k[i][d-1]..k[i+1][d-1] + if d == 1 { 1 } else { 2 }).min_by_key(
				|&s| {dp[i][s].unwrap() + dp[i + s + 1][d - s - 1].unwrap() }).unwrap();
			dp[i][d] = Some(dp[i][k[i][d]].unwrap() + dp[i + k[i][d] + 1][d - k[i][d] - 1].unwrap() + f.func(i..i + d + 1));
		}
	}
	dp
}
