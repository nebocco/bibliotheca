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
        st.push();
        while let Some(v) = st.pop() {
            for u in graph.edges_from(v) {
                if seen[u] == c {
                    let mut res = Vec::new();
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
                from[u] = v;
                st.push(u);
            }
        }
    }
    None
}