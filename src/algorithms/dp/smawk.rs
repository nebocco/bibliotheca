use crate::utils::algebraic_traits::Element;
use crate::algorithms::dp::Monge;

pub fn smawk_row_minima<T: Element + Ord, M: Monge<T>>(matrix: &M) -> Vec<usize> {
	let (n, m) = matrix.size();
	let mut res = vec![0; m];
	smawk_inner(
		&|i, j| matrix.index(i, j),
		&(0..n).collect::<Vec<usize>>(),
		&(0..m).collect::<Vec<usize>>(),
		&mut res
	);
	res
}

pub fn smawk_column_minima<T: Element + Ord, M: Monge<T>>(matrix: &M) -> Vec<usize> {
	let (n, m) = matrix.size();
	let mut res = vec![0; n];
	smawk_inner(
		&|j, i| matrix.index(i, j),
		&(0..m).collect::<Vec<usize>>(),
		&(0..n).collect::<Vec<usize>>(),
		&mut res
	);
	res
}

fn smawk_inner<T: Element + Ord, F: Fn(usize, usize) -> T>(
	matrix: &F,
	rows: &[usize],
	cols: &[usize],
	mut minima: &mut [usize]
) {
	if cols.is_empty() {
		return;
	}
	let mut stack = Vec::<usize>::with_capacity(rows.len());
	for &c in cols {
		while !stack.is_empty()
            && matrix(rows[stack.len() - 1], stack[stack.len() - 1])
                > matrix(rows[stack.len() - 1], c)
        {
            stack.pop();
        }
        if stack.len() != rows.len() {
            stack.push(c);
        }
	}

	let cols = &stack;
	let odd_rows = rows
		.iter()
		.copied()
		.skip(1)
		.step_by(2)
		.collect::<Vec<usize>>();

	smawk_inner(matrix, &odd_rows, cols, &mut minima);

	let mut c = 0;
	for (r, &row) in rows.iter().enumerate().step_by(2) {
		let mut col = cols[c];
		let last_col = if r == rows.len() - 1 {
			cols.last().unwrap().clone()
		} else {
			minima[rows[r + 1]]
		};
		let mut pair = (matrix(row, col), col);
		while col != last_col {
			c += 1;
			col = cols[c];
			pair = std::cmp::min(pair, (matrix(row, col), col));
		}
		minima[row] = pair.1;
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