#![allow(dead_code)]

use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct Edge {
    pub to: usize,
    pub cost: i64
}

#[derive(Copy, Clone)]
pub struct State {
    pub cost: i64,
    pub position: usize
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.cost == other.cost
    }
}

impl Eq for State {}

pub struct Graph(Vec<Vec<Edge>>);

impl Graph {
    pub fn new(size: usize) -> Self {
        Graph(vec![Vec::<Edge>::new(); size])
    }

    pub fn build(&mut self, graph:Vec<Vec<Edge>>) {
        self.0 = graph;
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn add_edge_directed(&mut self, fr: usize, to: usize, cost: i64) {
        self.0[fr].push(Edge{ to, cost });
    }

    pub fn add_edge_undirected(&mut self, u: usize, v: usize, cost: i64) {
        self.0[u].push(Edge{ to: v, cost });
        self.0[v].push(Edge{ to: u, cost });
    }

    pub fn edges_from(&self, v: usize) -> std::slice::Iter<Edge> {
        self.0[v].iter()
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
