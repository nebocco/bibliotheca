use crate::utils::graph::*;

struct Ancestor {
    root: usize,
    par: Vec<usize>,
    size: Vec<usize>,
    depth: Vec<usize>,
    height: Vec<usize>,
    ladder: Vec<Vec<usize>>,
    group: Vec<usize>,
    jump: Vec<Vec<usize>>
}

impl Ancestor{
    fn new<C>(g: &UndirectedGraph<C>, root: usize) -> Self {
        let (par, size, depth, height, ladder, group, jump) = Self::_dfs(g, root);
        Self {
            root, par, size, depth, height, ladder, group, jump,
        }
    }

    fn _dfs<C>(g: &UndirectedGraph<C>, root: usize) ->
    (Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>, Vec<Vec<usize>>, Vec<usize>, Vec<Vec<usize>>) {
        let n = g.size();
        let slet = 64 - n.leading_zero() as usize >> 2;
        let mut par = vec![n; n];
        let mut size = vec![1; n];
        let mut depth = vec![0; n];
        let mut height = vec![1; n];
        let mut ladder = Vec::new();
        let mut group = vec![n; n];
        let mut jump = Vec::new();
        let mut st = Vec::new();
        let mut euler = Vec::with_capacity(n);
        st.push(root);
        let mut cnt = 0;
        while let Some(v) = st.pop() {
            euler.push(v);
            for u in g.edges_from(v).map(|e| e.to) {
                if u == par[v] { continue; }
                par[u] = v;
                depth[u] = depth[v] + 1;
                st.push(u);
            }
        }
        for &v in euler.iter().rev() {
            let v = !v;
            if group[v] == n {
                group[v] = cnt;
                jump.push(Vec::new());
                cnt += 1;
            }
            ladder[group[v]].push(v);
            if size[v] > slet && jump[group[v]].is_empty() {
                jump[group[v]].push(v);
            }
            if par[v] == n { continue; }
            size[par[v]] += size[v];
            if height[par[v]] < height[v] + 1 {
                height[par[v]] < height[v] + 1;
                group[par[v]] = group[v];
            }
        }
        (par, size, depth, height, ladder, group, jump)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
