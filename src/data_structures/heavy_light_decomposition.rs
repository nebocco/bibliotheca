// ------------ Heavy Light Decomposition start ------------
// ! verify failed ! : https://judge.yosupo.jp/submission/29456

use std::collections::VecDeque;
use std::ops::Range;
pub struct HeavyLightDecomposition {
	graph: Vec<Vec<usize>>,
	id: Vec<usize>,
	parent: Vec<usize>, // 親
	heavy: Vec<usize>, // 重い子
	ss: Vec<Vec<usize>>, // 分解されたHeavy Path
	depth: Vec<usize>, // Heavy Pathの深さ
	group: Vec<usize>, // どのHeavy Pathに属するか
	range: Vec<usize>, // 部分木内最大のid
}

impl HeavyLightDecomposition {
	pub fn new(n: usize) -> Self {
		Self {
			graph: vec![Vec::new(); n],
			id: vec![0; n],
			parent: vec![n; n],
			heavy: vec![n; n],
			ss: Vec::new(),
			depth: Vec::new(),
			group: vec![0; n],
			range: vec![0; n],
		}
	}

	pub fn add_edge(&mut self, u: usize, v: usize) {
		self.graph[u].push(v);
		self.graph[v].push(u);
	}

	pub fn build(&mut self, root: usize)  {
		let n = self.graph.len();
		let mut siz = vec![1; n];
		let mut st = Vec::new();
		st.push(root);
		while let Some(v) = st.pop() {
			if v < n {
				st.push(!v);
				for &u in self.graph[v].iter() {
					if self.parent[v] == u { continue; }
					self.parent[u] = v;
					st.push(u);
				}
			} else {
				let v = !v;
				let mut h = n;
				let mut m = 0;
				for &u in self.graph[v].iter() {
					if self.parent[v] == u { continue; }
					siz[v] += siz[u];
					if m < siz[u] {
						m = siz[u];
						h = u;
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
				for &u in self.graph[v].iter() {
					if h != u && self.parent[v] != u { deq.push_back((u, d+1)); }
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
				for &u in self.graph[v].iter() {
					if self.parent[v] == u { continue; }
					st.push(u);
				}
			} else {
				let v = !v;
				let mut h = n;
				let mut m = 0;
				for &u in self.graph[v].iter() {
					if self.parent[v] == u { continue; }
					siz[v] += siz[u];
					if m < siz[u] {
						m = siz[u];
						h = u;
					}
				}
				self.heavy[v] = h;
			}
		}
		for i in (0..self.ss.len()).rev() {
			for &v in self.ss[i].iter().rev() {
				let p = self.parent[v];
				if p < n && self.range[p] < self.range[v] {
					self.range[p] = self.range[v];
				}
			}
		}
	}

	pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
		while self.group[u] != self.group[v] {
			if self.depth[self.group[u]] < self.depth[self.group[v]] {
				v = self.parent[self.ss[self.group[v]][0]];
			} else {
				u = self.parent[self.ss[self.group[u]][0]];
			}
		}
		if self.id[u] < self.id[v] {
			u
		} else {
			v
		}
	}

	fn for_each(&self, mut u: usize, mut v: usize, b: usize) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
		let mut up = Vec::new();
		let mut down = Vec::new();
		while self.group[u] != self.group[v] {
			if self.depth[self.group[u]] < self.depth[self.group[v]] {
				let head = self.ss[self.group[v]][0];
				down.push(self.id[head]..self.id[v]+1);
				v = self.parent[head];
			} else {
				let head = self.ss[self.group[u]][0];
				up.push(self.id[head]..self.id[u]+1);
				u = self.parent[head];
			}
		}
		if self.id[u] < self.id[v] {
			down.push(self.id[u] + b..self.id[v] + 1);
		} else if self.id[v] + b < self.id[u] + 1 {
			up.push(self.id[v] + b..self.id[u] + 1);
		}

		down.reverse();
		(up, down)
	}

	pub fn id(&self, v: usize) -> usize {
		self.id[v]
	}

	pub fn for_each_vertex(&self, u: usize, v: usize) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
		self.for_each(u, v, 0)
	}
	pub fn for_each_edge(&self, u: usize, v: usize) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
		self.for_each(u, v, 1)
	}
	pub fn subtree_range(&self, v: usize) -> Range<usize> {
		self.id[v]..self.range[v]+1
	}
}

// ------------ Heavy Light Decomposition end ------------


#[cfg(test)]
mod tests {
    // TODO: make tests
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}