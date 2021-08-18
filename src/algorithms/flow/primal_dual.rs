use super::{Cost, Flow};

pub struct MinCostFlow<F: Flow, C: Cost> {
    g: Vec<Vec<Edge<F, C>>>,
}

#[derive(Clone, Copy)]
struct Edge<F, C> {
    dst: usize,
    rev: usize,
    cap: F,
    cost: C,
}

impl<F: Flow + Into<C>, C: Cost> MinCostFlow<F, C> {
    pub fn new(n: usize) -> Self {
        Self {
            g: vec![Vec::new(); n],
        }
    }

    pub fn add_edge(&mut self, src: usize, dst: usize, cap: F, cost: C) {
        let rev_s = self.g[dst].len();
        let rev_d = self.g[src].len() + if src == dst { 1 } else { 0 };
        self.g[src].push(Edge {
            dst: dst,
            rev: rev_s,
            cap,
            cost,
        });
        self.g[dst].push(Edge {
            dst: src,
            rev: rev_d,
            cap: F::zero(),
            cost: -cost,
        })
    }

    pub fn run(&mut self, s: usize, t: usize, limit: F) -> (F, C) {
        let n = self.g.len();
        let mut flow = F::zero();
        let mut cost = C::zero();
        let mut p = vec![C::zero(); n];
        let mut dist = vec![C::zero(); n];
        let mut que = std::collections::BinaryHeap::new();
        let mut prev = vec![(0, 0); n];
        while flow < limit {
            dist.clear();
            dist.resize(n, C::MAX);
            dist[s] = C::zero();
            que.push((C::zero(), s));
            prev[t].0 = !0;
            while let Some((d, u)) = que.pop() {
                let d = -d;
                if dist[u] < d {
                    continue;
                }
                for (i, e) in self.g[u].iter().enumerate() {
                    let dd = d + e.cost - p[e.dst] + p[u];
                    if e.cap.is_positive() && dd < dist[e.dst] {
                        dist[e.dst] = dd;
                        que.push((-dd, e.dst));
                        prev[e.dst] = (u, i);
                    }
                }
            }
            if prev[t].0 == !0 {
                break;
            }

            for u in 0..n {
                if dist[u] != C::MAX {
                    p[u] += dist[u];
                }
            }
            let mut v = t;
            let mut add = limit - flow;
            while v != s {
                let (u, i) = prev[v];
                add = add.min(self.g[u][i].cap);
                v = u;
            }
            flow += add;
            let mut v = t;
            while v != s {
                let (u, i) = prev[v];
                let e = &mut self.g[u][i];
                cost += e.cost * add.into();
                e.cap -= add;
                let rev = e.rev;
                self.g[v][rev].cap += add;
                v = u;
            }
        }
        (flow, cost)
    }
}
