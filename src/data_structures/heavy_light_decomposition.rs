use crate::utils::graph::Tree;
use std::collections::VecDeque;
use std::ops::Range;
pub struct HeavyLightDecomposition<'a, T: Tree> {
	graph: &'a T,
	id: Vec<usize>,
	par: Vec<usize>, // 親
	heavy: Vec<usize>, // 重い子
	ss: Vec<Vec<usize>>, // 分解されたHeavy Path
	depth: Vec<usize>, // Heavy Pathの深さ
	group: Vec<usize>, // どのHeavy Pathに属するか
	range: Vec<usize>, // 部分木内最大のid
}

impl<'a, T: Tree> HeavyLightDecomposition<'a, T> {
	pub fn new(graph: &'a T) -> Self {
		let n = graph.size();
		let mut ret = Self {
			graph,
			id: vec![0; n],
			par: vec![n; n],
			heavy: vec![n; n],
			ss: Vec::new(),
			depth: Vec::new(),
			group: Vec::new(),
			range: vec![0; n],
		};
		ret.build(0);
		ret
	}

	fn build(&mut self, root: usize) {
		let n = self.graph.size();
		let mut siz = vec![1; n];
		let mut st = Vec::new();
		st.push(root);
		while let Some(v) = st.pop() {
			if v < n {
				st.push(!v);
				for e in self.graph.edges_from(v) {
					if self.par[v] == e.to { continue; }
					self.par[e.to] = v;
					st.push(e.to);
				}
			} else {
				let v = !v;
				let mut h = n;
				let mut m = 0;
				for e in self.graph.edges_from(v) {
					if self.par[v] == e.to { continue; }
					siz[v] += siz[e.to];
					if m < siz[e.to] {
						m = siz[e.to];
						h = e.to;
					}
				}
				self.heavy[v] = h;
			}
		}
		let mut deq = VecDeque::new();
		deq.push_back((0, 0));
		let mut i = 0;
		while let Some((mut v, d)) = deq.pop_front() {
			let mut s = Vec::new();
			let k = self.ss.len();
			while v < n {
				s.push(v);
				self.group[v] = k;
				self.id[v] = i; self.range[v] = i; i += 1;
				let h = self.heavy[v];
				for e in self.graph.edges_from(v) {
					if h != e.to { deq.push_back((e.to, d+1))}
				}
				v = h;
			}
			self.ss.push(s);
			self.depth.push(d);
		}

		let mut st = Vec::new();
		st.push(root);
		while let Some(v) = st.pop() {
			if v < n {
				st.push(!v);
				for e in self.graph.edges_from(v) {
					if self.par[v] == e.to { continue; }
					st.push(e.to);
				}
			} else {
				let v = !v;
				let mut h = n;
				let mut m = 0;
				for e in self.graph.edges_from(v) {
					if self.par[v] == e.to { continue; }
					siz[v] += siz[e.to];
					if m < siz[e.to] {
						m = siz[e.to];
						h = e.to;
					}
				}
				self.heavy[v] = h;
			}
		}
		for i in (0..self.ss.len()).rev() {
			for &v in self.ss[i].iter().rev() {
				let p = self.par[v];
				if p < n && self.range[p] < self.range[v] {
					self.range[p] = self.range[v];
				}
			}
		}
	}

	fn for_each(&self, mut u: usize, mut v: usize, b: usize) -> Vec<Range<usize>> {
		let mut res = Vec::new();
		loop {
			if self.id[u] > self.id[v] {
				std::mem::swap(&mut u, &mut v);
			}
			if self.group[u] == self.group[v] {
				if u != v {
					res.push(self.id[u]+b..self.id[v]+1);
				}
				break;
			} else {
				let head = self.ss[self.group[v]][0];
				res.push(self.id[head]..self.id[v]+1);
				v = self.par[head];
			}
		}
		res
	}

	pub fn for_each_vertex(&self, u: usize, v: usize) -> Vec<Range<usize>> {
		self.for_each(u, v, 0)
	}
	pub fn for_each_edge(&self, u: usize, v: usize) -> Vec<Range<usize>> {
		self.for_each(u, v, 1)
	}
	pub fn subtree_range(&self, v: usize) -> Range<usize> {
		self.id[v]..self.range[v]+1
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