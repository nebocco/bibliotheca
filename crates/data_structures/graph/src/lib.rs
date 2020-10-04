use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct Edge {
    to: usize,
    cost: usize
}

#[derive(Copy, Clone)]
pub struct State {
    cost: usize,
    position: usize
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

pub struct Graph {
    size: usize,
    graph: Vec<Vec<Edge>>
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Graph { size, graph: vec![Vec::<Edge>::new(); size] }
    }

    pub fn build(&mut self, graph:Vec<Vec<Edge>>) {
        self.graph = graph;
    }

    pub fn add_edge_directed(&mut self, fr: usize, to: usize, cost: usize) {
        self.graph[fr].push(Edge{ to, cost });
    }

    pub fn add_edge_undirected(&mut self, u: usize, v: usize, cost: usize) {
        self.graph[u].push(Edge{ to: v, cost });
        self.graph[v].push(Edge{ to: u, cost });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
