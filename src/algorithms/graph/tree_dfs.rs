use crate::utils::graph::{ Cost, Graph };

pub fn tree_dfs<C: Cost, G: Graph<C>>(g: &G, root: usize) -> (Vec<C>, Vec<Option<usize>>) {
    let n = g.size();
	let mut dist = vec![C::MAX; n];
	dist[root] = C::zero();
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