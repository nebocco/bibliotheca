// * verified: https://judge.yosupo.jp/submission/29574
// ------------ Heavy Light Decomposition start ------------

use std::ops::Range;

pub struct HeavyLightDecomposition {
	graph: Vec<Vec<usize>>,
	index: Vec<usize>, // 新しい頂点番号
	parent: Vec<usize>, // 親
	head: Vec<usize>, // 属するHeavy Pathの根
	range: Vec<usize>, // 部分木の開区間右端
}

impl HeavyLightDecomposition {
	pub fn new(n: usize) -> Self {
		Self {
			graph: vec![Vec::new(); n],
			index: Vec::new(),
			parent: Vec::new(),
			head: Vec::new(),
			range: Vec::new(),
		}
	}

	pub fn add_edge(&mut self, u: usize, v: usize) {
		self.graph[u].push(v);
		self.graph[v].push(u);
	}

	pub fn build(&mut self, root: usize)  {
		let graph = &mut self.graph;
		let n = graph.len();
		let mut index = vec![0; n];
		let mut parent = vec![n; n];
		let mut head = vec![root; n];
		let mut range = vec![0; n];
		let mut siz = vec![1; n];
		let mut st = Vec::new();
		st.push(root);
		while let Some(v) = st.pop() {
			if v < n {
				st.push(!v);
				if let Some(k) = graph[v].iter().position(|&u| u == parent[v]) {
					graph.swap_remove(k);
				}
				graph[v].iter().for_each(|&u| { parent[u] = v; st.push(u); });
			} else {
				let v = !v;
				for i in 0..graph[v].len() {
					let u = graph[v][i];
					siz[v] += siz[u];
					if siz[graph[v][0]] < siz[u] {
						graph[v].swap(0, i);
					}
				}
			}
		}
		st.push(root);
		let mut c = 0;
		while let Some(v) = st.pop() {
			if v < n {
				st.push(!v);
				index[v] = c; c += 1;
				for &u in graph[v].iter().skip(1) {
					head[u] = u;
					st.push(u);
				}
				if let Some(&u) = graph[v].get(0) {
					head[u] = head[v];
					st.push(u);
				}
			} else {
				range[!v] = c;
			}
		}
		self.index = index;
		self.parent = parent;
		self.head = head;
		self.range = range;
	}

	pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
		let parent = &self.parent;
		let head = &self.head;
		let index = &self.index;

		while head[u] != head[v] {
			if index[u] < index[v] {
				v = parent[head[v]];
			} else {
				u = parent[head[u]];
			}
		}
		if index[u] < index[v] {
			u
		} else {
			v
		}
	}

	fn for_each(&self, mut u: usize, mut v: usize, b: usize) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
		let parent = &self.parent;
		let head = &self.head;
		let index = &self.index;

		let mut up = Vec::new();
		let mut down = Vec::new();
		while head[u] != head[v] {
			if index[u] < index[v] {
				let h = head[v];
				down.push(index[h]..index[v] + 1);
				v = parent[h];
			} else {
				let h = head[u];
				up.push(index[h]..index[u] + 1);
				u = parent[h];
			}
		}
		if index[u] < index[v] {
			down.push(index[u] + b .. index[v] + 1);
		} else if index[v] + b < index[u] + 1 {
			up.push(index[v] + b .. index[u] + 1);
		}

		down.reverse();
		(up, down)
	}

	pub fn id(&self, v: usize) -> usize {
		self.index[v]
	}

	pub fn for_each_vertex(&self, u: usize, v: usize) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
		self.for_each(u, v, 0)
	}
	pub fn for_each_edge(&self, u: usize, v: usize) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
		self.for_each(u, v, 1)
	}
	pub fn subtree_range(&self, v: usize) -> Range<usize> {
		self.index[v]..self.range[v]
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