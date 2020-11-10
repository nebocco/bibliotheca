use crate::utils::graph::{UndirectedGraph, Edge};
use crate::data_structures::union_find::UnionFind;

fn kruskal(graph: &mut UndirectedGraph, edges: &[Edge]) -> Vec<Edge> {
    let mut f = edges.clone();
    f.sort_by_key(|x| x.cost);
    let mut res = Vec::with_capacity(graph.size() - 1);
    let uf = UnionFind::new(graph.size());
    for e in f.into_iter() {
        if !uf.same(e.from, e.to) {
            uf.unite(e.from, e.to);
            graph.add_edge(e);
            res.push(e);
        }
    }
    res
}