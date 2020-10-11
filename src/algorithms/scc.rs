use crate::utils::graph::*;
use crate::algorithms::traversal::*;

pub trait SCC {
    fn strongly_connected(&self) -> Vec<usize>;
}

impl SCC for DirectedGraph {
    fn strongly_connected(&self) -> Vec<usize> {
        fn _scc_dfs(graph: &DirectedGraph, x: usize, res: &mut [Option<usize>]) {
            for y in graph.edges_from(x) {
                if res[y.to].is_none() {
                    res[y.to] = res[x];
                    _scc_dfs(graph, y.to, res);
                }
            }
        }
        let n = self.size();
        let post_backward = Traversal::post_order(self);
        let mut res: Vec<Option<usize>> = vec![None; n];
        let mut cnt = 0;
        for &x in post_backward.index.iter().rev() {
            if res[x].is_none() {
                res[x] = Some(cnt);
                _scc_dfs(self, x, &mut res);
                cnt += 1;
            }
        }
        res.iter().map(|x| cnt - 1 - x.unwrap()).collect()
    }
}

pub struct TwoSat(DirectedGraph);

impl TwoSat {
    pub fn new(n: usize) -> Self {
        Self(DirectedGraph::new(2 * n))
    }

    pub fn add_clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
        self.0.add_edge(2 * i + !f as usize, 2 * j + g as usize, 0);
        self.0.add_edge(2 * j + !g as usize, 2 * i + f as usize, 0);
    }

    pub fn solve(&self) -> Option<Vec<bool>> {
        self.0
            .strongly_connected()
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

#[cfg(test)]
mod tests {
    // TODO: make tests
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}