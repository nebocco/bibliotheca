pub struct MinCostFlow {
    g: Vec<Vec<__InternalEdge>>,
}

// #[derive(Clone, Copy)]
// struct Edge {
//     to: usize,
//     from: usize,
//     flow: i64,
//     cap: i64,
//     cost: i64,
// }

#[derive(Clone, Copy)]
struct __InternalEdge {
    to: usize,
    rev: usize,
    cap: i64,
    cost: i64,
}

impl MinCostFlow {
    pub fn new(n: usize) -> Self {
        Self {
            g: vec![Vec::new(); n],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, cap: i64, cost: i64) {
        let rev_u = self.g[u].len();
        let rev_v = self.g[v].len() + if u == v { 1 } else { 0 };
        self.g[u].push(__InternalEdge {
            to: v,
            rev: rev_u,
            cap,
            cost,
        });
        self.g[v].push(__InternalEdge {
            to: u,
            rev: rev_v,
            cap: 0,
            cost: -cost,
        })
    }

    pub fn flow(&mut self, source: usize, sink: usize, limit: i64) -> (i64, i64) {
        let n = self.g.len();
        let mut flow = 0;
        let mut cost = 0;
        let mut p = vec![0; n];
        let mut dist = vec![0; n];
        let mut que = std::collections::BinaryHeap::new();
        let mut prev = vec![(0, 0); n];
        while flow < limit {
            dist.clear();
            dist.resize(n, std::i64::MAX);
            dist[source] = 0;
            que.push((0, source));
            prev[sink].0 = !0;
            while let Some((d, u)) = que.pop() {
                let d = -d;
                if dist[u] < d {
                    continue;
                }
                for (i, e) in self.g[u].iter().enumerate() {
                    let dd = d + e.cost - p[e.to] + p[u];
                    if e.cap > 0 && dd < dist[e.to] {
                        dist[e.to] = dd;
                        que.push((0 - dd, e.to));
                        prev[e.to] = (u, i);
                    }
                }
            }
            if prev[sink].0 == !0 {
                break;
            }

            for u in 0..n {
                if dist[u] != std::i64::MAX {
                    p[u] += dist[u];
                }
            }
            let mut v = sink;
            let mut add = limit - flow;
            while v != source {
                let (u, i) = prev[v];
                add = add.min(self.g[u][i].cap);
                v = u;
            }
            flow += add;
            let mut v = sink;
            while v != source {
                let (u, i) = prev[v];
                let e = &mut self.g[u][i];
                cost += e.cost * add;
                e.cap -= add;
                let rev = e.rev;
                self.g[v][rev].cap += add;
                v = u;
            }
        }
        (flow, cost)
    }
}
