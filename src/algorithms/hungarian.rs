#![allow(dead_code)]

fn hungarian(a: &[Vec<i64>]) -> (i64, Vec<usize>) {
	let n = a.len();
	let (mut p, mut q): (usize, usize);
	let mut x = vec![n; n];
	let mut y = vec![n; n];
	assert!(a[0].len() == n,
	"given matrix is not square: {} rows, {} columns", n, a[0].len());
	let mut fx = (0..n).map(
		|k| a[k].iter().max().unwrap().clone()
	).collect::<Vec<_>>();
	let mut fy = vec![0; n];
	let mut i = 0;
	while i < n {
		let mut t = vec![n; n];
		let mut s = vec![i; n+1];
		p = 0; q = 0;
		while p <= q && x[i] == n {
			let mut k = s[p];
			let mut j = 0;
			while j < n && x[i] == n {
				if fx[k] + fy[j] == a[k][j] && t[j] == n {
					q += 1;
					s[q] = y[j];
					t[j] = k;
					if s[q] == n {
						p = j;
						while p != n {
							y[j] = t[j];
							k = t[j];
							p = x[k];
							x[k] = j;
							j = p;
						}
					}
				}
				j += 1;
			}
			p += 1;
		}
		// println!("\nt: {:?}", t);
		// println!("s: {:?}", s);
		// println!("x: {:?}", x);
		// println!("y: {:?}", y);
		// println!("fx: {:?}", fx);
		// println!("fy: {:?}", fy);
		if x[i] == n {
			println!("x[{}] = n, q = {}", i, q);
			let mut d = std::i64::MAX;
			for k in 0..=q {
				for j in 0..n {
					if t[j] == n {
						d = std::cmp::min(d, fx[s[k]] + fy[j] - a[s[k]][j]);
					}
				}
			}
			for j in 0..n {
				fy[j] += if t[j] == n { 0 } else { d };
			}
			for k in 0..=q {
				fx[s[k]] -= d;
			}
		} else {
			i += 1;
		}
	}
	let ret = (0..n).map(|i| a[i][x[i]]).sum::<i64>();
	(ret, x)
}

#[cfg(test)]
mod tests {
	use super::*;
	// ! test failed
    #[test]
    fn test_4x4() {
		let a = vec![
			vec![0, 1, 2, 3],
			vec![2, 3, 0, 1],
			vec![3, 0, 1, 2],
			vec![1, 2, 3, 0],
		];
		let (sc, x) = hungarian(&a);
		assert_eq!(sc, 0);
		assert_eq!(x, vec![0, 2, 1, 3]);
    }
}