use crate::utils::graph::{Edge, Graph};

pub fn is_biparate<C, G: Graph<C>>(graph: &G) -> Option<Vec<usize>> {
    let n = graph.size();
    let mut color = vec![2 * n; n];
    let mut c = 0;
    for i in 0..n {
        if color[i] < 2 * n {
            continue;
        }
        color[i] = c;
        let mut st = Vec::new();
        st.push(i);
        while let Some(v) = st.pop() {
            for &Edge { to: u, .. } in graph.edges_from(v) {
                if color[u] == 2 * n {
                    color[u] = color[v] ^ 1;
                    st.push(u);
                } else if color[u] == color[v] {
                    return None;
                }
            }
        }
        c += 2;
    }
    Some(color)
}
