use std::ops::Range;

use crate::utils::algebraic_traits::Element;
use crate::algorithms::dp::Monge;

pub fn smawk_row_minima<T: Element, M: Monge<T>>(matrix: &M) -> Vec<usize> {
	let (n, m) = matrix.size();
	let mut res = vec![0; m];
	smawk_inner(
		&|i, j| matrix.index(i, j),
		0..n,
		0..m,
		&mut res
	);
	res
}

pub fn smawk_column_minima<T: Element, M: Monge<T>>(matrix: &M) -> Vec<usize> {
	let (n, m) = matrix.size();
	let mut res = vec![0; n];
	smawk_inner(
		&|j, i| matrix.index(i, j),
		0..m,
		0..n,
		&mut res
	);
	res
}

fn smawk_inner<T: Element, F: Fn(usize, usize) -> T>(
	matrix: &F,
	rows: Range<usize>,
	cols: Range<usize>,
	mut minima: &mut[usize]
) {
	if cols.start == cols.end {
		return;
	}
	let mut stack = Vec::<usize>::with_capacity(rows.len());
	for c in cols {
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