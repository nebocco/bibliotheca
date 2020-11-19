use crate::utils::graph::*;

#[derive(Debug, Clone)]
pub struct Traversal {
    pub index: Vec<usize>,
    pub time: Vec<usize>,
}

impl Traversal {
    pub fn pre_order<C: Cost>(graph: &[Vec<Edge<C>>]) -> Self {
        fn _dfs<C: Cost>(graph: &[Vec<Edge<C>>], x: usize, res: &mut PermutationBuilder) {
            res.visit(x);
            for &y in graph[x].iter() {
                if !res.on_stack(y.to) {
                    _dfs(graph, y.to, res);
                }
            }
        }

        let n = graph.len();
        let mut res = PermutationBuilder::new(n);
        for i in 0..n {
            if !res.on_stack(i) {
                _dfs(graph, i, &mut res);
            }
        }
        res.build()
    }

    pub fn post_order<C: Cost>(graph: &[Vec<Edge<C>>]) -> Self {
        fn _dfs<C: Cost>(graph: &[Vec<Edge<C>>], x: usize, ckd: &mut [bool], res: &mut PermutationBuilder) {
            for &y in graph[x].iter() {
                if !std::mem::replace(&mut ckd[y.to], true) {
                    _dfs(graph, y.to, ckd, res);
                }
            }
            res.visit(x);
        }

        let n = graph.len();
        let mut ckd = vec![false; n];
        let mut res = PermutationBuilder::new(n);
        for i in 0..n {
            if !std::mem::replace(&mut ckd[i], true) {
                _dfs(graph, i, &mut ckd, &mut res);
            }
        }
        res.build()
    }
}

#[derive(Debug, Clone)]
struct PermutationBuilder {
    index: Vec<usize>,
    time: Vec<usize>,
}

impl PermutationBuilder {
    fn new(n: usize) -> Self {
        Self {
            index: Vec::with_capacity(n),
            time: vec![n; n],
        }
    }

    fn build(self) -> Traversal {
        Traversal {
            index: self.index,
            time: self.time,
        }
    }

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.time.is_empty()
    }

    fn len(&self) -> usize {
        self.time.len()
    }

    fn time(&self) -> usize {
        self.index.len()
    }

    fn visit(&mut self, x: usize) {
        assert!(!self.on_stack(x));
        self.time[x] = self.time();
        self.index.push(x);
    }

    fn on_stack(&self, x: usize) -> bool {
        self.time[x] != self.len()
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