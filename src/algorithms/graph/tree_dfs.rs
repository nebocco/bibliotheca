use crate::utils::graph::{Cost, Graph};

pub fn tree_dfs<C: Cost, G: Graph<C>>(
    g: &G,
    root: usize,
) -> (Vec<C>, Vec<Option<usize>>, Vec<usize>, Vec<usize>) {
    let n = g.size();
    let mut euler = Vec::with_capacity(n);
    let mut dist = vec![C::MAX; n];
    dist[root] = C::zero();
    let mut par = vec![None; n];
    let mut size = vec![1; n];
    let mut q = vec![root];
    while let Some(v) = q.pop() {
        euler.push(v);
        for e in g.edges_from(v) {
            if par[v] == Some(e.to) {
                continue;
            }
            par[e.to] = Some(v);
            dist[e.to] = dist[v] + e.cost;
            q.push(e.to);
        }
    }
    for &v in euler.iter().skip(1).rev() {
        size[par[v].unwrap()] += size[v];
    }
    (dist, par, size, euler)
}

#[cfg(test)]
mod tests {
    use super::tree_dfs;
    use crate::utils::graph::{Graph, UndirectedGraph};

    #[test]
    fn test_tree_dfs() {
        let mut g = UndirectedGraph::<i32>::new(5);
        let ed = vec![(0, 3, 8), (3, 1, 2), (2, 4, 1), (4, 3, 9)];
        for &(u, v, c) in &ed {
            g.add_edge(u, v, c);
        }
        let (dist, par, size, euler) = tree_dfs(&g, 0);
        assert_eq!(dist, vec![0, 10, 18, 8, 17]);
        assert_eq!(par, vec![None, Some(3), Some(4), Some(0), Some(3)]);
        assert_eq!(size, vec![5, 1, 1, 4, 2]);
        assert_eq!(euler, vec![0, 3, 4, 2, 1]);
    }
}
