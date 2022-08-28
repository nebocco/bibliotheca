use super::{Cost, Graph, UndirectedGraph};
use crate::data_structures::union_find::UnionFind;

// ------------ Kruskal's algorithm start ------------

pub fn kruskal<C: Cost>(
    graph: &mut UndirectedGraph<C>,
    edges: &mut [(usize, usize, C)],
) -> Vec<(usize, usize, C)> {
    edges.sort_by_key(|x| x.2);
    let mut res = Vec::with_capacity(graph.size() - 1);
    let mut uf = UnionFind::new(graph.size());
    for &e in edges.iter() {
        if uf.unite(e.0, e.1).is_ok() {
            graph.add_edge(e.0, e.1, e.2);
            res.push(e);
        }
    }
    res
}

// ------------ Kruskal's algorithm end ------------
