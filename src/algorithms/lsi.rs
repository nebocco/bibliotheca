/*
use crate::utils::{
    graph::{Graph, DirectedGraph},
    algebraic_traits::Group
};

fn solve_lsi<E: Group>(g: &DirectedGraph, c: &[E]) -> Option<Vec<E>> {
    let n = g.size();
    let mut seen = vec![false; n];
    let mut res = vec![E::zero(); g.edges()];
    for u in 0..n {
        if !seen[u] {
            let y = dfs(u, g, &mut seen, &mut res, c);
            if y != 0 {
                return None;
            }
        }
    }
    Some(res)
}

fn dfs<E: Group>(u: usize, g: &DirectedGraph, seen: &mut [bool], res: &mut [E], c: &[E]) -> E {
    seen[u] = true;
    let mut r = c[u].clone();
    for e in g.edges_from(u) {
        if seen[e.to] { continue; }
        let y = dfs(e.to, g, seen, res, c);
        res[e.id] = res[e.id].clone() + y.clone();
        r = r + y;
    }
    for e in g.edges_to(u) {
        if seen[e.to] { continue; }
        let y = dfs(e.to, g, seen, res, c);
        res[e.id] = res[e.id].clone() + -y.clone();
        r = r + -y;
    }
    r
}

*/

#[cfg(test)]
mod tests {
    // TODO: make tests
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
