/*
https://gist.github.com/wata-orz/d3037bd0b919c76dd9ddc0379e1e3192
*/

/*

/// Computes single-source unorthodox shortest paths with respect to the given shortest-path tree `sp`.
/// O(m log n) time.
pub fn sssup<T: Group>(g: &UndirectedGraph<GEdge>, s: usize) -> Vec<i64> {
	let n = g.size();
	let sp = SSSP::<T>::new(g, s);
	let mut dist = vec![std::i64::MAX; n];
	let mut uf = UnionFind::new(n);
	let mut que = std::collections::BinaryHeap::new();
	for u in 0..n {
		for (i, e) in g.edges_from(u).enumerate() {
			if u < e.to && sp.label[u].clone() + e.label.clone() != sp.label[e.to].clone() {
				que.push((-(sp.dist[u] + sp.dist[e.to] + e.cost), u, e.to));
			}
		}
	}
	while let Some((h, u0, v0)) = que.pop() {
		let h = -h;
		let mut u = uf.find(u0);
		let mut v = uf.find(v0);
		let mut bs = vec![];
		while u != v {
			if sp.depth[u] > sp.depth[v] {
				bs.push(u);
				u = uf.find(sp.parent[u]);
			} else {
				bs.push(v);
				v = uf.find(sp.parent[v]);
			}
		}
		for v in bs {
			uf.unite(u, v);
			dist[v] = h - sp.dist[v];
			for e in g.edges_from(v) {
				if sp.label[v].clone() + e.label.clone() == sp.label[e.to].clone() {
					que.push((-(dist[v] + sp.dist[e.to] + e.cost), v, e.to));
				}
			}
		}
	}
	dist
}

/// Single-Source Shortest Paths
#[derive(Clone, Debug)]
pub struct SSSP<T: Group> {
	pub s: usize,
	pub dist: Vec<i64>,
	pub parent: Vec<usize>,
	pub depth: Vec<usize>,
	pub label: Vec<T>
}

impl<T: Group> SSSP<T> {
	/// Computes single-source shortest paths with Dijkstra algorithm
	/// O(ElogV)
	pub fn new(g: &UndirectedGraph<GEdge<T>>, s: usize) -> Self {
		let n = g.size();
		let mut dist = vec![std::i64::MAX; n];
		let mut depth = vec![std::usize::MAX; n];
		let mut parent = vec![std::usize::MAX; n];
		let mut label = vec![T::zero(); n];
		let mut que = std::collections::BinaryHeap::new();
		dist[s] = 0;
		depth[s] = 0;
		que.push((0, s));
		while let Some((d, u)) = que.pop() {
			let d = -d;
			if dist[u] < d { continue; }
			for e in g.edges_from(u) {
				let v = e.to;
				let d2 = d + e.cost;
				if dist[v] > d2 {
					dist[v] = d2;
					depth[v] = depth[u] + 1;
					parent[v] = u;
					label[v] = label[u].clone() + e.label.clone();
					que.push((-d2, v));
				}
			}
		}
		Self { s, dist, parent, depth, label }
	}
}

*/