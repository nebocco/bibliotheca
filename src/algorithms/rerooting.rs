pub struct Rerooting<Edge, Value, Func> {
    size: usize,
    edge: Vec<(usize, usize, Edge)>,
    initial: Value,
    merge: Func,
}

impl<Edge, Value, Func> Rerooting<Edge, Value, Func>
where
    Edge: Clone,
    Value: Clone,
    Func: Fn(&Value, &Value, &Edge) -> Value,
{
    pub fn new(size: usize, initial: Value, merge: Func) -> Self {
        Self {
            size,
            edge: vec![],
            initial,
            merge,
        }
    }

    pub fn add_edge(&mut self, a: usize, b: usize, cost: Edge) {
        self.edge.push((a, b, cost));
    }

    pub fn solve(&self, root: usize) -> Vec<Value> {
        let mut graph = vec![vec![]; self.size];
        for e in self.edge.iter() {
            let a = e.0;
            let b = e.1;
            graph[a].push((b, e.2.clone()));
            graph[b].push((a, e.2.clone()));
        }

        let mut euler = vec![];
        let mut stack = vec![(root, root)];
        while let Some((v, p)) = stack.pop() {
            euler.push(v);
            if let Some(k) = graph[v].iter().position(|e| e.0 == p) {
                graph[v].swap_remove(k);
            }
            for e in graph[v].iter() {
                stack.push((e.0, v));
            }
        }
        assert!(euler.len() == self.size);

        let mut down = vec![self.initial.clone(); self.size];
        for &v in euler.iter().rev() {
            for e in graph[v].iter() {
                down[v] = (self.merge)(&down[v], &down[e.0], &e.1);
            }
        }
        let mut up = vec![self.initial.clone(); self.size];
        let mut ans = up.clone();
        for &v in euler.iter() {
            ans[v] = up[v].clone();
            for e in graph[v].iter() {
                ans[v] = (self.merge)(&ans[v], &down[e.0], &e.1);
            }
            if graph[v].is_empty() {
                continue;
            }
            let mut stack = vec![(graph[v].as_slice(), up[v].clone())];
            while let Some((g, val)) = stack.pop() {
                if g.len() == 1 {
                    up[g[0].0] = (self.merge)(&self.initial, &val, &g[0].1);
                } else {
                    let (a, b) = g.split_at(g.len() / 2);
                    let mut p = val.clone();
                    for e in a.iter() {
                        p = (self.merge)(&p, &down[e.0], &e.1);
                    }
                    let mut q = val;
                    for e in b.iter() {
                        q = (self.merge)(&q, &down[e.0], &e.1);
                    }
                    stack.push((b, p));
                    stack.push((a, q));
                }
            }
        }
        ans
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
