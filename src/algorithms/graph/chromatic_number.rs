// verified: https://judge.yosupo.jp/submission/64225

/// compute chromatic number of a general undirected graph
/// O(n*2^n)
/// g := (u, v) in E iff g[u] & 1 << v
pub fn chromatic_number(g: &[usize]) -> usize {
    let n = g.len();
    if n < 2 {
        return n;
    }

    // a randomly chosen large prime
    const MOD: i64 = 1_077_563_119;
    let all = 1 << n;
    let mut ind = vec![1; all];
    let mut s = vec![0; all];
    for i in 0..all {
        s[i] = if (n - i.count_ones() as usize) & 1 == 1 {
            -1
        } else {
            1
        };
    }
    for i in 1..all {
        let ctz = i.trailing_zeros() as usize;
        ind[i] = ind[i - (1 << ctz)] + ind[(i - (1 << ctz)) & !g[ctz]];
        if ind[i] >= MOD {
            ind[i] -= MOD;
        }
    }

    for k in 1..n {
        let mut sum = 0;
        for i in 0..all {
            s[i] = s[i] * ind[i] % MOD;
            sum += s[i];
        }
        if sum % MOD != 0 {
            return k;
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::chromatic_number;

    #[test]
    fn test_chromatic_number() {
        let n = 5;
        let edges = vec![
            (0, 1),
            (0, 2),
            (0, 4),
            (1, 3),
            (2, 3),
            (2, 4),
            (3, 4),
        ];
        let mut g = vec![0; n];
        for (u, v) in edges {
            g[u] |= 1 << v;
            g[v] |= 1 << u;
        }
        assert_eq!(chromatic_number(&g), 3);
    }

    #[test]
    fn test_chromatic_number_empty_graph() {
        let g = vec![0; 20];
        assert_eq!(chromatic_number(&g), 1);
    }
}