#![allow(dead_code)]

#[derive(Clone, Debug)]
pub struct Line {
	slope: i64,
	intercept: i64,
}

impl Line {
	pub fn new(slope: i64, intercept: i64) -> Self {
		Line { slope, intercept }
	}

	pub fn eval(&self, x: i64) -> i64 {
		self.slope * x + self.intercept
	}
}

// pub struct Segment {
// 	line: Line,
// 	left: i64,
// 	right: i64
// }

// impl Segment {
// 	pub fn new(slope: i64, intercept: i64, left: i64, right: i64) -> Self {
// 		Segment { line: Line::new(slope, intercept), left, right }
// 	}

// 	pub fn eval(&self, x: i64) -> Option<i64> {
// 		if self.is_valid(x) {
// 			Some(self.line.eval(x))
// 		} else {
// 			None
// 		}
// 	}

// 	pub fn is_valid(&self, x: i64) -> bool {
// 		self.left <= x && x <= self.right
// 	}
// }

pub struct LiChaoTree {
	n: usize,
	points: Vec<i64>,
	tree: Vec<Line>,
	u: Vec<bool>
}

impl LiChaoTree {
	const INF: i64 = 1 << 60;

	fn new(n0: usize, xs: &[i64]) -> Self {
		let n = n0.next_power_of_two();
		let u = vec![false; 2*n];
		let mut points = vec![Self::INF ; 2*n];
		for i in 0..n0 {
			points[i] = xs[i];
		}
		LiChaoTree {
			n, points, u,
			tree: vec![Line::new(0, Self::INF); 2*n],
		}
	}

	fn _add_line(&mut self, mut line: Line, mut k: usize, mut l: usize, mut r:usize) {
		while r - l > 0 {
			let m = (l + r) >> 1;
			if !self.u[k] {
				self.tree[k] = line;
				self.u[k] = true;
				return;
			}
			let lx = self.points[l];
			let mx = self.points[m];
			let rx = self.points[r-1];
			let cur = &self.tree[k];
			let lb = line.eval(lx) < cur.eval(lx);
			let mb = line.eval(mx) < cur.eval(mx);
			let rb = line.eval(rx) < cur.eval(rx);
			if lb && rb {
				self.tree[k] = line;
				return;
			} else if !lb && !rb {
				return;
			}
			if mb {
				std::mem::swap(&mut line, &mut self.tree[k]);
			}
			if lb ^ mb {
				k = (k << 1) + 1;
				r = m;
			} else {
				k = (k << 1) + 2;
				l = m;
			}
		}
	}

	fn _query(&self, mut k: usize, x: i64) -> i64 {
		k += self.n - 1;
		let mut s = if self.u[k] { self.tree[k].eval(x) } else { Self::INF };
		while k > 0 {
			k = (k - 1) >> 1;
			if self.u[k] {
				s = std::cmp::min(s, self.tree[k].eval(x));
			}
		}
		s
	}

	pub fn add_line(&mut self, a: i64, b: i64) {
		let line = Line::new(a, b);
		self._add_line(line, 0, 0, self.n);
	}

	pub fn add_segment(&mut self, a: i64, b: i64, l: usize, r: usize) {
		let line = Line::new(a, b);
		let mut l0 = l + self.n;
		let mut r0 = r + self.n;
		let mut s0 = l;
		let mut t0 = r;
		let mut sz = 1;
		while l0 < r0 {
			if r0 & 1 > 0 {
				r0 -= 1; t0 -= sz;
				self._add_line(line.clone(), r0-1, t0, t0+sz);
			}
			if l0 & 1 > 0 {
				self._add_line(line.clone(), l0-1, s0, s0+sz);
				l0 += 1; s0 += sz;
			}
			l0 >>= 1; r0 >>= 1;
			sz <<= 1;
		}
	}

	pub fn query(&self, i: usize) -> i64 {
		return self._query(i, self.points[i])
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