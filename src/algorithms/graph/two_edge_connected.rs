use crate::utils::graph::{Graph, UndirectedGraph};

// ERROR: BROKEN

/// decomposes vertices into two-edge connected components
/// and enumerates bridges using LowLink.
pub fn two_edge_connected_components<C: Clone>(g: &UndirectedGraph<C>) -> BridgeHelper {
    let n = g.size();
    let mut ord = vec![std::usize::MAX; n];
    let mut low = vec![0; n];

    fn dfs<C: Clone>(
        g: &UndirectedGraph<C>,
        v: usize,
        mut k: usize,
        ord: &mut [usize],
        low: &mut [usize],
    ) -> usize {
        ord[v] = k;
        k += 1;
        low[v] = ord[v];
        for e in g.edges_from(v) {
            let u = e.to;
            if ord[u] == std::usize::MAX {
                k = dfs(g, u, k, ord, low);
                low[v] = low[v].min(low[u]);
            }
            low[v] = low[v].min(ord[u]);
        }
        k
    }

    dfs(g, 0, 0, &mut ord, &mut low);
    let mut bh = BridgeHelper::new(ord, low);
    for v in 0..n {
        for e in g.edges_from(v) {
            let u = e.to;
            if u < v && !bh.is_bridge((u, v)) {
                bh.unite(u, v).ok();
            }
        }
    }
    bh
}

pub struct BridgeHelper {
    ord: Vec<usize>,
    low: Vec<usize>,
    uf: Vec<isize>,
    group: Vec<usize>,
}

impl BridgeHelper {
    pub fn new(ord: Vec<usize>, low: Vec<usize>) -> Self {
        let n = ord.len();
        Self {
            ord,
            low,
            uf: vec![-1; n],
            group: (0..n).collect(),
        }
    }

    pub fn find(&mut self, i: usize) -> usize {
        self._climb(i).0
    }

    pub fn size(&mut self, i: usize) -> usize {
        self._climb(i).1
    }

    fn unite(&mut self, u: usize, v: usize) -> Result<(), ()> {
        let (mut u, su) = self._climb(u);
        let (mut v, sv) = self._climb(v);
        if u == v {
            return Err(());
        }
        if su < sv {
            std::mem::swap(&mut u, &mut v);
        }
        self.uf[u] += self.uf[v];
        self.uf[v] = u as isize;
        self.group.swap(u, v);
        Ok(())
    }

    pub fn is_same(&mut self, u: usize, v: usize) -> bool {
        self.find(u) == self.find(v)
    }

    fn _climb(&mut self, i: usize) -> (usize, usize) {
        assert!(i < self.uf.len());
        let mut v = i;
        while self.uf[v] >= 0 {
            let p = self.uf[v] as usize;
            if self.uf[p] >= 0 {
                self.uf[v] = self.uf[p];
                v = self.uf[p] as usize;
            } else {
                v = p;
            }
        }
        (v, -self.uf[v] as usize)
    }

    pub fn is_bridge(&self, (mut u, mut v): (usize, usize)) -> bool {
        if self.ord[u] > self.ord[v] {
            std::mem::swap(&mut u, &mut v);
        }
        self.ord[u] < self.low[v]
    }

    pub fn group(&self, u: usize) -> Vec<usize> {
        let mut v = self.group[u];
        let mut res = Vec::new();
        res.push(u);
        while v != u {
            res.push(v);
            v = self.group[v]
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::graph::*;

    #[test]
    fn test_two_edge_connected_components() {
        let n = 13;
        let edges = [
            (4, 5),
            (8, 7),
            (12, 3),
            (3, 10),
            (1, 5),
            (10, 2),
            (0, 0),
            (11, 4),
            (2, 12),
            (9, 1),
            (9, 0),
            (7, 8),
            (7, 6),
            (9, 1),
            (8, 2),
            (12, 10),
            (11, 0),
            (8, 6),
            (3, 2),
            (5, 9),
            (4, 11),
        ];
        let mut g = UndirectedGraph::new(n);
        for &(u, v) in &edges {
            g.add_edge(u, v, Void);
        }

        let bh = two_edge_connected_components(&g);
        let mut gr = bh.group(0);
        gr.sort();
        assert_eq!(gr, vec![0, 1, 4, 5, 9, 11]);
        let mut gr = bh.group(2);
        gr.sort();
        assert_eq!(gr, vec![2, 3, 10, 12]);
        let mut gr = bh.group(6);
        gr.sort();
        assert_eq!(gr, vec![6, 7, 8]);
    }
}
