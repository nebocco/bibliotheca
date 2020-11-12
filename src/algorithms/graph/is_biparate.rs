use crate::utils::graph::Graph;

pub fn is_biparate<G: Graph>(graph: &G) -> bool {
    let n = graph.size();
    let mut color = vec![2; n];
    for i in 0..n {
        if color[i] < 2 { continue; }
        color[i] = 0;
        let mut st = Vec::new();
        st.push(i);
        while let Some(v) = st.pop() {
            for u in graph.edges_from(v) {
                if color[u.to] == 2 {
                    color[u.to] = color[v] ^ 1;
                    st.push(u.to);
                } else if color[u.to] == color[v] {
                    return false;
                }
            }
        }
    }
    true
}