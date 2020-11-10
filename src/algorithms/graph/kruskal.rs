#![allow(dead_code)]
use crate::utils::graph::{Graph, UndirectedGraph};
use crate::data_structures::union_find::UnionFind;

fn kruskal(graph: &mut UndirectedGraph, edges: &mut [(usize, usize, i64)]) -> Vec<(usize, usize, i64)> {
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