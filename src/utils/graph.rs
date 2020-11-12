use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct Edge {
    // pub from: usize,
    pub to: usize,
    pub cost: i64,
    // pub id: usize
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

pub struct UndirectedGraph(pub Vec<Vec<Edge>>);
pub struct DirectedGraph{
    pub forward: Vec<Vec<Edge>>,
    pub backward: Vec<Vec<Edge>>
}

pub trait Graph {
    fn new(size: usize) -> Self;
    fn size(&self) -> usize;
    fn add_edge(&mut self, u: usize, v: usize, cost: i64);
    fn edges_from(&self, v: usize) -> std::slice::Iter<Edge>;
}

pub trait Tree: Graph {}

impl Graph for UndirectedGraph {
    fn new(size: usize) -> Self {
        Self(vec![Vec::<Edge>::new(); size])
    }

    fn size(&self) -> usize {
        self.0.len()
    }

    fn add_edge(&mut self, u: usize, v: usize, cost: i64) {
        self.0[u].push(Edge{ to: v, cost });
        self.0[v].push(Edge{ to: u, cost });
    }

    fn edges_from(&self, v: usize) -> std::slice::Iter<Edge> {
        self.0[v].iter()
    }
}

impl Graph for DirectedGraph {
    fn new(size: usize) -> Self {
        Self {
            forward: vec![Vec::<Edge>::new(); size],
            backward: vec![Vec::<Edge>::new(); size],
        }
    }

    fn size(&self) -> usize {
        self.forward.len()
    }

    fn add_edge(&mut self, u: usize, v: usize, cost: i64) {
        self.forward[u].push(Edge{ to: v, cost });
        self.backward[v].push(Edge{ to: u, cost });
    }

    fn edges_from(&self, v: usize) -> std::slice::Iter<Edge> {
        self.forward[v].iter()
    }
}

impl DirectedGraph {
    pub fn reverse(&self) -> Self {
        Self {
            forward: self.backward.clone(),
            backward: self.forward.clone(),
        }
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
