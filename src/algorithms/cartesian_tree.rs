// verified: https://judge.yosupo.jp/submission/61844

/// construct cartesian tree for Vector lis.
/// lis should be disinct.
pub fn cartesian_tree<T: Ord>(lis: &[T]) -> Vec<Option<usize>> {
    let n = lis.len();
    let mut parent = vec![None; n];
    for i in 1..n {
        let mut p = Some(i - 1); // parent
        let mut l = None; // left child
        while let Some(v) = p {
            if lis[i] > lis[v] {
                break;
            }
            let pp = parent[v];
            if let Some(u) = l {
                parent[u] = p;
            }
            parent[v] = Some(i);
            l = p;
            p = pp;
        }
        parent[i] = p;
    }
    parent
}
