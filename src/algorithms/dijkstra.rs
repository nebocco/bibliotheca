#![allow(dead_code)]

use std::collections::BinaryHeap;
use crate::utils::graph::{Edge, State, Graph};


fn dijkstra<T: Graph>(graph: &T, start: usize, goal: usize) -> i64 {
    let mut dist = vec![std::i64::MAX; graph.size()];
    dist[start] = 0;
    let mut que: BinaryHeap<State> = BinaryHeap::new();
    que.push(State{cost:0, position:start});
    while let Some(State{cost:c, position:v}) = que.pop() {
        if dist[v] > c { continue; };
        for &Edge{to: x, cost: d} in graph.edges_from(v) {
            if dist[x] > c + d {
                dist[x] = c + d;
                que.push(State{ cost: c + d, position: x })
            }
        }
    }
    dist[goal]
}

#[cfg(test)]
mod tests {
    // TODO: make tests
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}