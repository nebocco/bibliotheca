use crate::utils::graph::{Edge, Graph};

pub fn detect_cycle<C, G: Graph<C>>(graph: &G) -> Option<Vec<usize>> {
    let n = graph.size();
    let mut seen = vec![0; n];
    let mut from = vec![n; n];
    let mut c = 0;
    let mut st = Vec::new();
    for i in 0..graph.size() {
        if seen[i] > 0 {
            continue;
        }
        c += 1;
        seen[i] = c;
        st.push(i);
        while let Some(v0) = st.pop() {
            for &Edge { to: u, .. } in graph.edges_from(v0) {
                if seen[u] == c {
                    let mut res = Vec::new();
                    let mut v = v0;
                    res.push(v);
                    while v != u {
                        v = from[v];
                        res.push(v);
                    }
                    res.reverse();
                    return Some(res);
                } else if seen[u] > 0 {
                    continue;
                }
                seen[u] = c;
                from[u] = v0;
                st.push(u);
            }
        }
    }
    None
}
