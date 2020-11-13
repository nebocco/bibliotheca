use crate::utils::algebraic_traits::Element;
use crate::algorithms::dp::Monge;

pub fn smawk_row_minima<T: Element + Ord, M: Monge<T>>(matrix: &M) -> Vec<usize> {
	let (n, m) = matrix.size();
	let mut res = vec![0; n];
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
	let mut res = vec![0; m];
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
			*cols.last().unwrap()
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

// TODO: write fn online_row/column_minima

#[cfg(test)]
mod tests {
	use super::*;
	impl<T: Element> Monge<T> for Vec<Vec<T>> {}

	#[test]
    fn smawk_1x1() {
        let matrix = vec![vec![2]];
        assert_eq!(smawk_row_minima(&matrix), vec![0]);
        assert_eq!(smawk_column_minima(&matrix), vec![0]);
    }

    #[test]
    fn smawk_2x1() {
        let matrix = vec![
            vec![3], //
            vec![2],
        ];
        assert_eq!(smawk_row_minima(&matrix), vec![0, 0]);
        assert_eq!(smawk_column_minima(&matrix), vec![1]);
    }

    #[test]
    fn smawk_1x2() {
        let matrix = vec![vec![2, 1]];
        assert_eq!(smawk_row_minima(&matrix), vec![1]);
        assert_eq!(smawk_column_minima(&matrix), vec![0, 0]);
    }

    #[test]
    fn smawk_2x2() {
        let matrix = vec![
            vec![3, 2], //
            vec![2, 1],
        ];
        assert_eq!(smawk_row_minima(&matrix), vec![1, 1]);
        assert_eq!(smawk_column_minima(&matrix), vec![1, 1]);
    }

    #[test]
    fn smawk_3x3() {
        let matrix = vec![
            vec![3, 4, 4], //
            vec![3, 4, 4],
            vec![2, 3, 3],
        ];
        assert_eq!(smawk_row_minima(&matrix), vec![0, 0, 0]);
        assert_eq!(smawk_column_minima(&matrix), vec![2, 2, 2]);
    }

    #[test]
    fn smawk_4x4() {
        let matrix = vec![
            vec![4, 5, 5, 5], //
            vec![2, 3, 3, 3],
            vec![2, 3, 3, 3],
            vec![2, 2, 2, 2],
        ];
        assert_eq!(smawk_row_minima(&matrix), vec![0, 0, 0, 0]);
        assert_eq!(smawk_column_minima(&matrix), vec![1, 3, 3, 3]);
    }

    #[test]
    fn smawk_5x5() {
        let matrix = vec![
            vec![3, 2, 4, 5, 6],
            vec![2, 1, 3, 3, 4],
            vec![2, 1, 3, 3, 4],
            vec![3, 2, 4, 3, 4],
            vec![4, 3, 2, 1, 1],
        ];
        assert_eq!(smawk_row_minima(&matrix), vec![1, 1, 1, 1, 3]);
        assert_eq!(smawk_column_minima(&matrix), vec![1, 1, 4, 4, 4]);
    }
}