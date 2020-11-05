/*
https://gist.github.com/wata-orz/d3037bd0b919c76dd9ddc0379e1e3192
*/

/// Computes single-source unorthodox shortest paths with respect to the given shortest-path tree `sp`.
/// O(m log n) time.
pub fn sssup(g: &UndirectedGraph<G_Edge>, s: usize) -> Vec<i64> {
	let n = g.size();
	let sp = SSSP::new(g, s);
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

struct G_Edge<T: Group> {
	from: usize,
	to: usize,
	cost: i64,
	label: T,
	is_consistet: bool
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
	pub fn new(g: &UndirectedGraph<G_Edge<T>>, s: usize) -> Self {
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

pub fn dijkstra_heap<G: Graph>(g: &G, s: usize) -> Self {
	let n = g.size();
	let mut dist = vec![std::i64::MAX; n];
	let mut depth = vec![std::usize::MAX; n];
	let mut parent = vec![std::usize::MAX; n];
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
				que.push((-d2, v));
			}
		}
	}
	Self { s, dist, parent, depth }
}

/// Computes single-source shortest paths with Dijkstra algorithm
/// O(V^2)
pub fn dijkstra_loop<G: Graph>(g: &G, s: usize) -> Self {
	let n = g.size();
	let mut dist = vec![std::i64::MAX; n];
	let mut depth = vec![std::usize::MAX; n];
	let mut parent = vec![std::usize::MAX; n];
	let mut done = vec![false; n];
	dist[s] = 0;
	depth[s] = 0;
	for _ in 0..n-1 {
		let u = (0..n).filter(|&i| !done[i]).min_by_key(|i| dist[i]).unwrap();
		done[u] = true;
		for e in g.edges_from(u) {
			if dist[e.to] > dist[u] + e.cost {
				dist[e.to] = dist[u] + e.cost;
				depth[e.to] = depth[u] + 1;
				parent[e.to] = u;
			}
		}
	}
	Self { s, dist, parent, depth }
}

/// Computes single-source shortest paths with Bellman-Ford algorithm
/// O(EV)
pub fn bellman_ford<G: Graph>(g: &G, s: usize) -> Result<Self, &str> {
	let n = g.size();
	let mut dist = vec![std::i64::MAX; n];
	let mut depth = vec![std::usize::MAX; n];
	let mut parent = vec![std::usize::MAX; n];
	for _ in 0..n-1 {
		for v in 0..n {
			for e in g.edges_from(v) {
				if dist[e.to] > dist[v] + e.cost {
					dist[e.to] = dist[v] + e.cost;
					depth[e.to] = depth[v] + 1;
					parent[e.to] = v;
				}
			}
		}
	}

	for v in 0..n {
		for e in g.edges_from(v) {
			if dist[e.to] > dist[v] + e.cost {
				Err("graph contains a negative cycle")
			}
		}
	}
	Ok(Self { s, dist, parent, depth })
}