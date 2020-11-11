#![allow(dead_code)]
use crate::utils::graph::Tree;

fn tree_dfs<T: Tree>(g: &T, root: usize) -> (Vec<i64>, Vec<Option<usize>>) {
    let n = g.size();
	let mut dist = vec![std::i64::MAX; n];
	dist[root] = 0;
    let mut par = vec![None; n];
    let mut q = vec![root];
    while let Some(v) = q.pop() {
        for e in g.edges_from(v) {
            if par[v] == Some(e.to) { continue; }
            par[e.to] = Some(v);
            dist[e.to] = dist[v] + e.cost;
            q.push(e.to);
        }
    }
    (dist, par)
}

#[cfg(test)]
mod tests {
    // TODO: make tests
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}