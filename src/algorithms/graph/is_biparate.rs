use crate::utils::graph::Graph;

pub fn is_biparate<G: Graph>(graph: &G) -> Option<Vec<usize>> {
    let n = graph.size();
    let mut color = vec![2 * n; n];
    let mut c = 0;
    for i in 0..n {
        if color[i] < 2 * n { continue; }
        color[i] = c;
        let mut st = Vec::new();
        st.push(i);
        while let Some(v) = st.pop() {
            for u in graph.edges_from(v) {
                if color[u.to] == 2 * n {
                    color[u.to] = color[v] ^ 1;
                    st.push(u.to);
                } else if color[u.to] == color[v] {
                    return None;
                }
            }
        }
        c += 2;
    }
    Some(color)
}