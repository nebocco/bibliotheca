#![allow(dead_code, non_snake_case)]

use std::collections::BinaryHeap;
use crate::utils::graph::{Edge, State, Graph};


// * verified: https://judge.yosupo.jp/submission/28367
fn dijkstra_ElogV<T: Graph>(graph: &T, start: usize) -> (Vec<i64>, Vec<usize>) {
	let n = graph.size();
	let mut dist = vec![std::i64::MAX; n];
	let mut path = (0..n).collect::<Vec<_>>();
    dist[start] = 0;
    let mut que: BinaryHeap<State> = BinaryHeap::new();
    que.push(State{cost:0, position:start});
    while let Some(State{cost:c, position:v}) = que.pop() {
        if dist[v] < c { continue; };
        for &Edge{to: x, cost: d} in graph.edges_from(v) {
            if dist[x] > c + d {
				dist[x] = c + d;
				path[x] = v;
                que.push(State{ cost: c + d, position: x })
            }
        }
    }
    (dist, path)
}

fn dijkstra_V2<T: Graph>(graph: &T, start: usize) -> Vec<i64> {
    let n = graph.size();
    let mut dist = vec![std::i64::MAX; n];
    dist[start] = 0;
    let mut seen = vec![false; n];
    while let Some(v) = (0..n).filter(|&i| !seen[i]).min_by_key(|&i| dist[i]) {
        seen[v] = true;
        for &Edge{to: x, cost: d} in graph.edges_from(v) {
            if dist[x] > dist[v] + d {
                dist[x] = dist[v] + d;
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    // TODO: make tests
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}