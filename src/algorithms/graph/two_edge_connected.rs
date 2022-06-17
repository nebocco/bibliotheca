use crate::utils::graph::{Edge, Graph, UndirectedGraph};
use std::marker::PhantomData;

// 2-edge-connected: 
// Bi-connected: https://judge.yosupo.jp/submission/92453
// FIXME: stack overflow

pub struct LowLink<'a, C, G> {
    graph: &'a G,
    used: Vec<bool>,
    ord: Vec<usize>,
    low: Vec<usize>,
    articulation: Vec<usize>,
    bridge: Vec<(usize, Edge<C>)>,
    _phantom_data: PhantomData<C>
}

impl<'a, C: Clone, G: 'a + Graph<C>> LowLink<'a, C, G> {
    fn dfs(&mut self, idx: usize, k: &mut usize, par_edge: Option<&Edge<C>>) {
        self.used[idx] = true;
        self.ord[idx] = *k;
        *k += 1;
        self.low[idx] = self.ord[idx];
        let mut is_articulation = false;
        let mut cnt = 0;
        for e in self.graph.edges_from(idx) {
            if !self.used[e.to] {
                cnt += 1;
                self.dfs(e.to, k, Some(e));
                self.low[idx] = self.low[idx].min(self.low[e.to]);
                is_articulation |= par_edge.is_some() && self.low[e.to] >= self.ord[idx];
                if self.ord[idx] < self.low[e.to] {
                    self.bridge.push((idx, e.clone()));
                }
            } else if e.id != par_edge.map(|e| e.id).unwrap_or(std::usize::MAX) {
                self.low[idx] = self.low[idx].min(self.ord[e.to]);
            }
        }
        is_articulation |= par_edge.is_none() && cnt > 1;
        if is_articulation {
            self.articulation.push(idx);
        }
    }

    pub fn new(graph: &'a G) -> Self {
        let n = graph.size();
        let mut low_link = Self {
            graph,
            used: vec![false; n],
            ord: vec![0; n],
            low: vec![0; n],
            articulation: Vec::new(),
            bridge: Vec::new(),
            _phantom_data: PhantomData
        };
        let mut k = 0;
        for i in 0..n {
            if !low_link.used[i] {
                low_link.dfs(i, &mut k, None);
            }
        }
        low_link
    }
}

pub struct TwoEdgeConnectedComponents<'a, C, G> {
    low_link: LowLink<'a, C, G>,
    comp: Vec<usize>,
}

impl<'a, C: Clone, G: 'a + Graph<C>> TwoEdgeConnectedComponents<'a, C, G>  {
    pub fn new(graph: &'a G) -> Self {
        let n = graph.size();
        let mut tecc = Self {
            low_link: LowLink::new(graph),
            comp: vec![std::usize::MAX; n],
        };
        tecc.build();
        tecc
    }

    fn dfs(&mut self, idx: usize, k: &mut usize, par: usize) {
        if !par > 0 && self.low_link.ord[par] >= self.low_link.low[idx] {
            self.comp[idx] = self.comp[par];
        } else {
            self.comp[idx] = *k;
            *k += 1; 
        }
        for e in self.low_link.graph.edges_from(idx) {
            if self.comp[e.to] == std::usize::MAX {
                self.dfs(e.to, k, idx);
            }
        }
    }

    fn build(&mut self) {
        let n = self.low_link.graph.size();
        let mut k = 0;
        for i in 0..n {
            if self.comp[i] == std::usize::MAX {
                self.dfs(i, &mut k, std::usize::MAX);
            }
        }
    }

    pub fn create_graph(&self) -> UndirectedGraph<C> {
        let k = *self.comp.iter().max().unwrap() + 1;
        let mut res = UndirectedGraph::new(k);
        for (u, e) in self.low_link.bridge.iter() {
            let u = self.comp[*u];
            let v = self.comp[e.to];
            res.add_edge(u, v, e.cost.clone());
        }
        res
    }

    pub fn get_group_id(&self, i: usize) -> usize {
        self.comp[i]
    }

    pub fn get_groups(&self) -> Vec<Vec<usize>> {
        let k = *self.comp.iter().max().unwrap();
        let mut res = vec![Vec::new(); k + 1];
        for (i, &k) in self.comp.iter().enumerate() {
            res[k].push(i);
        }
        res
    }
}

pub struct BiConnectedComponents<'a, C, G> {
    low_link: LowLink<'a, C, G>,
    used: Vec<bool>,
    bc: Vec<Vec<usize>>,
    tmp: Vec<&'a Edge<C>>
}

impl<'a, C: Clone, G: 'a + Graph<C>> BiConnectedComponents<'a, C, G>  {
    pub fn new(graph: &'a G) -> Self {
        let n = graph.size();
        let mut bicc = Self {
            low_link: LowLink::new(graph),
            used: vec![false; n],
            bc: Vec::new(),
            tmp: Vec::new()
        };
        bicc.build();
        bicc
    }

    fn dfs(&mut self, idx: usize, par_edge: Option<&Edge<C>>) {
        self.used[idx] = true;
        for e in self.low_link.graph.edges_from(idx) {
            if e.id == par_edge.map(|e| e.id).unwrap_or(std::usize::MAX) {
                continue;
            }
            if !self.used[e.to] || self.low_link.ord[e.to] < self.low_link.ord[idx] {
                self.tmp.push(e);
            }
            if !self.used[e.to] {
                self.dfs(e.to, Some(e));
                if self.low_link.low[e.to] >= self.low_link.ord[idx] {
                    let mut b = Vec::new();
                    while let Some(val) = self.tmp.pop() {
                        b.push(val.id);
                        if e.id == val.id {
                            break;
                        }
                    }
                    self.bc.push(b);
                }
            }
        }
    }

    fn build(&mut self) {
        let n = self.low_link.graph.size();
        for i in 0..n {
            if !self.used[i] {
                self.dfs(i, None);
            }
        }
    }

    pub fn get_groups(&self) -> &Vec<Vec<usize>> {
        &self.bc
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

        let bh = TwoEdgeConnectedComponents::new(&g);
        let gr = bh.get_groups();
        assert!(gr.contains(&vec![0, 1, 4, 5, 9, 11]));
        assert!(gr.contains(&vec![2, 3, 10, 12]));
        assert!(gr.contains(&vec![6, 7, 8]));
    }
}
