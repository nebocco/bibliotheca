use crate::utils::algebraic_traits::{Element, Group};
use crate::algorithms::graph::{DirectedGraph, Graph};

pub fn solve_lsi<C: Element, E: Group>(g: &DirectedGraph<C>, c: &[E]) -> Option<Vec<E>> {
    let n = g.forward.len();
    let mut seen = vec![false; n];
    let mut res = vec![E::zero(); g.count];
    for u in 0..n {
        if !seen[u] {
            let y = lsi_dfs(u, g, &mut seen, &mut res, c);
            if !y.is_zero() {
                return None;
            }
        }
    }
    Some(res)
}

fn lsi_dfs<C: Element, E: Group>(
    u: usize,
    g: &DirectedGraph<C>,
    seen: &mut [bool],
    res: &mut [E],
    c: &[E],
) -> E {
    seen[u] = true;
    let mut r = c[u].clone();
    for e in g.edges_from(u) {
        if seen[e.to] {
            continue;
        }
        let y = lsi_dfs(e.to, g, seen, res, c);
        res[e.id] = res[e.id].clone() + y.clone();
        r = r + y;
    }
    for e in g.edges_to(u) {
        if seen[e.to] {
            continue;
        }
        let y = -lsi_dfs(e.to, g, seen, res, c);
        res[e.id] = res[e.id].clone() + y.clone();
        r = r + y;
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::algebraic_traits::{Associative, Zero};
    use crate::algorithms::graph::{DirectedGraph, Graph, Void};
    use std::ops::{Add, Neg};

    #[derive(Clone, Copy, PartialEq, Debug)]
    struct Xor(bool);

    impl Zero for Xor {
        fn zero() -> Self {
            Xor(false)
        }
        fn is_zero(&self) -> bool {
            !self.0
        }
    }

    impl Add for Xor {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Self(self.0 ^ rhs.0)
        }
    }

    impl Neg for Xor {
        type Output = Self;
        fn neg(self) -> Self {
            self
        }
    }

    impl Associative for Xor {}

    fn execute_even_degrees(n: usize, edges: Vec<(usize, usize)>) {
        let mut c = vec![Xor(false); n];
        let mut g = DirectedGraph::new(n);
        for &(u, v) in &edges {
            g.add_edge(u, v, Void);
            c[u] = c[u] + Xor(true);
        }
        let res = solve_lsi(&g, &c).unwrap();
        let mut ans = vec![true; n];
        for v in 0..n {
            for e in g.edges_from(v) {
                if res[e.id].0 {
                    ans[e.to] ^= true;
                } else {
                    ans[v] ^= true;
                }
            }
        }
        assert!(ans.iter().all(|&x| x));
    }

    #[test]
    fn simple_even_degrees() {
        let n = 5;
        let edges = vec![(0, 1), (1, 2), (2, 3), (2, 4), (3, 4), (1, 4)];
        execute_even_degrees(n, edges);
    }

    #[test]
    fn random_even_degrees() {
        use rand::prelude::*;

        const REPEAT: usize = 10;
        const SIZE: usize = 1000;
        const EDGES: usize = 4000;
        let mut rng = rand::thread_rng();
        for _ in 0..REPEAT {
            let edges = (0..EDGES)
                .map(|_| (rng.gen_range(0..SIZE), rng.gen_range(0..SIZE)))
                .collect::<Vec<_>>();
            execute_even_degrees(SIZE, edges);
        }
    }
}
