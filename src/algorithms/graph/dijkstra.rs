use crate::utils::graph::{ Cost, Graph };

#[allow(clippy::many_single_char_names)]
pub fn dijkstra_heap<C: Cost, G: Graph<C>>(g: &G, s: usize) -> Vec<C> {
	let n = g.size();
	let mut dist = vec![C::MAX; n];
	let mut depth = vec![std::usize::MAX; n];
	let mut parent = vec![std::usize::MAX; n];
	let mut que = std::collections::BinaryHeap::new();
	dist[s] = C::zero();
	depth[s] = 0;
	que.push((C::zero(), s));
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
	dist
}

/// Computes single-source shortest paths with Dijkstra algorithm
/// O(V^2)
pub fn dijkstra_loop<C: Cost, G: Graph<C>>(g: &G, s: usize) -> Vec<C> {
	let n = g.size();
	let mut dist = vec![C::MAX; n];
	let mut depth = vec![std::usize::MAX; n];
	let mut parent = vec![std::usize::MAX; n];
	let mut done = vec![false; n];
	dist[s] = C::zero();
	depth[s] = 0;
	for _ in 0..n-1 {
		let u = (0..n).filter(|&i| !done[i]).min_by_key(|&i| dist[i]).unwrap();
		done[u] = true;
		for e in g.edges_from(u) {
			if dist[e.to] > dist[u] + e.cost {
				dist[e.to] = dist[u] + e.cost;
				depth[e.to] = depth[u] + 1;
				parent[e.to] = u;
			}
		}
	}
	dist
}

/// Computes single-source shortest paths with Bellman-Ford algorithm
/// O(EV)
pub fn bellman_ford<C: Cost, G: Graph<C>>(g: &G, s: usize) -> Result<Vec<C>, &str> {
	let n = g.size();
	let mut dist = vec![C::MAX; n];
	dist[s] = C::zero();
	let mut depth = vec![std::usize::MAX; n];
	depth[s] = 0;
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
				return Err("graph contains a negative cycle");
			}
		}
	}
	Ok(dist)
}

#[cfg(test)]
mod tests {
    // TODO: make tests
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}