use super::*;
use crate::algorithms::traversal::*;

// * verified: https://judge.yosupo.jp/submission/26465
// ------------ 2-SAT start ------------

// * verified: https://judge.yosupo.jp/submission/26463
// ------------ Strongly Connected Components start ------------
// ! DirectedGraph::reverse() is too heavy

pub trait SCC {
    fn strongly_connected(&self) -> (usize, Vec<usize>);
    fn groups(&self) -> Vec<Vec<usize>>;
}

impl<C: Cost> SCC for DirectedGraph<C> {
    fn strongly_connected(&self) -> (usize, Vec<usize>) {
        fn _scc_dfs<C: Cost>(graph: &DirectedGraph<C>, x: usize, res: &mut [Option<usize>]) {
            for y in graph.edges_from(x) {
                if res[y.to].is_none() {
                    res[y.to] = res[x];
                    _scc_dfs(graph, y.to, res);
                }
            }
        }
        let n = self.size();
        let post_backward = Traversal::post_order(&self.backward);
        let mut res: Vec<Option<usize>> = vec![None; n];
        let mut cnt = 0;
        for &x in post_backward.index.iter().rev() {
            if res[x].is_none() {
                res[x] = Some(cnt);
                _scc_dfs(self, x, &mut res);
                cnt += 1;
            }
        }
        (cnt, res.iter().map(|x| cnt - 1 - x.unwrap()).collect())
    }

    fn groups(&self) -> Vec<Vec<usize>> {
        let (c, g) = self.strongly_connected();
        let mut res = vec![Vec::new(); c];
        for (i, &x) in g.iter().enumerate() {
            res[x].push(i);
        }
        res
    }
}

// ------------ Strongly Connected Components end ------------

pub struct TwoSat(DirectedGraph<Void>);

impl TwoSat {
    pub fn new(n: usize) -> Self {
        Self(DirectedGraph::new(2 * n))
    }

    pub fn add_clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
        self.0
            .add_edge(2 * i + !f as usize, 2 * j + g as usize, Void);
        self.0
            .add_edge(2 * j + !g as usize, 2 * i + f as usize, Void);
    }

    pub fn solve(&self) -> Option<Vec<bool>> {
        self.0
            .strongly_connected()
            .1
            .chunks_exact(2)
            .map(|v| {
                use std::cmp::Ordering::*;
                match v[0].cmp(&v[1]) {
                    Equal => None,
                    Less => Some(true),
                    Greater => Some(false),
                }
            })
            .collect()
    }
}

// ------------ 2-SAT end ------------

#[cfg(test)]
mod tests {
    // TODO: make tests
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
