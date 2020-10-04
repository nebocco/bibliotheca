use std::collections::BinaryHeap;
use graph::{Edge, State, Graph};

// ! cannot define inherent `impl` for a type outside of the crate where the type is defined

fn dijkstra(graph: &Graph, start: usize, goal: usize) -> i64 {
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
