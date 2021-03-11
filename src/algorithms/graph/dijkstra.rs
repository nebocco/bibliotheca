use crate::utils::graph::{ Cost, Graph };

#[allow(clippy::many_single_char_names)]
pub fn dijkstra_01<C: Cost, G: Graph<C>>(g: &G, s: usize) -> Vec<C> {
	let n = g.size();
	let mut dist = vec![C::MAX; n];
	let mut depth = vec![std::usize::MAX; n];
	let mut parent = vec![std::usize::MAX; n];
	let mut que = std::collections::VecDeque::new();
	dist[s] = C::zero();
	depth[s] = 0;
	que.push_front(s);
	while let Some(u) = que.pop_front() {
		for e in g.edges_from(u) {
			let v = e.to;
			if dist[v] > dist[u] + e.cost {
				dist[v] = dist[u] + e.cost;
				depth[v] = depth[u] + 1;
				parent[v] = u;
				if e.cost == C::zero() {
					que.push_front(v);
				} else {
					que.push_back(v);
				}
			}
		}
	}
	dist
}

// ------------ Dijkstra's algorithm start ------------

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

// ------------ Dijkstra's algorithm end ------------


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
				if dist[v] != C::MAX && dist[e.to] > dist[v] + e.cost {
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
	use super::*;
	use crate::utils::graph::UndirectedGraph;

    #[test]
    fn test_3_ways() {
		let mut g = UndirectedGraph::new(6);
		g.add_edge(0, 1, 15);
		g.add_edge(1, 2, 3);
		g.add_edge(4, 1, 58);
		g.add_edge(3, 5, 10);
		g.add_edge(0, 5, 8);
		g.add_edge(2, 5, 1);

		let ans = vec![0, 12, 9, 18, 70, 8];
		let res = dijkstra_heap(&g, 0);
		assert_eq!(res, ans);

		let res = dijkstra_loop(&g, 0);
		assert_eq!(res, ans);

		let res = bellman_ford(&g, 0);
		assert_eq!(res.ok().unwrap(), ans);
	}
}