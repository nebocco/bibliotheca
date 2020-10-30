#[allow(dead_code, unused_mut, unused_variables)]
fn hungarian(a: &[Vec<i64>]) -> (i64, Vec<usize>) {
	let n = a.len();
	let mut p: usize;
	let mut q: usize;
	let mut x = vec![-1; n];
	let mut y = vec![-1; n];
	assert!(a[0].len() == n,
	"given matrix is not square: {} rows, {} columns", n, a[0].len());
	let fs = (0..n).map(
		|k| a[k].iter().max().unwrap().clone()
	).collect::<Vec<_>>();
	let mut i = 0;
	while i < n {
		let mut t = vec![-1; n];
		let mut s = vec![i; n+1];
		p = 0; q = 0;
		while p <= q && x[i] < 0 {
			p += 1;
		}
	}
	(0, vec![0])
}
