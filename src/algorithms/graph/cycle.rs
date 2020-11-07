#![allow(dead_code)]
use crate::utils::graph::Graph;

fn detect_cycle<G: Graph>(graph: &G) -> Option<Vec<usize>> {
    let n = graph.size();
    let mut seen = vec![0; n];
    let mut from = vec![n; n];
    let mut c = 0;
    let mut st = Vec::new();
    for i in 0..graph.size() {
        if seen[i] > 0 { continue; }
        c += 1;
        seen[i] = c;
        st.push(i);
        while let Some(v0) = st.pop() {
            for u in graph.edges_from(v0) {
                if seen[u.to] == c {
                    let mut res = Vec::new();
                    let mut v = v0;
                    res.push(v);
                    while v != u.to {
                        v = from[v];
                        res.push(v);
                    }
                    res.reverse();
                    return Some(res);
                } else if seen[u.to] > 0 {
                    continue;
                }
                seen[u.to] = c;
                from[u.to] = v0;
                st.push(u.to);
            }
        }
    }
    None
}