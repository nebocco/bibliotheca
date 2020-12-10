// * verified: https://judge.yosupo.jp/submission/28369
// ------------ Z algorithm start ------------

pub fn z_algorithm<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let mut ret = vec![0; s.len()];
    ret[0] = s.len();
    let mut p = 0;
    let mut i = 1;
    while i < s.len() {
        while s.get(p) == s.get(i + p) {
            p += 1;
        }
        ret[i] = p;
        let mut k = 1;
        while i + k < s.len() && k + ret[k] < p {
            ret[i+k] = ret[k];
            k += 1;
        }
        i += k;
        p = p.saturating_sub(k);
    }
    ret
}

// ------------ Z algorithm end ------------

// ------------ KMP algorithm start ------------

pub fn kmp_table<T: PartialEq>(pattern: &[T]) -> Vec<usize> {
    let n = pattern.len();
    let mut table = vec![n; n+1];
    let mut j = 0;
    for i in 1..n {
        if pattern[i] == pattern[j] {
            table[i] = table[j];
        } else {
            table[i] = j;
            j = table[j];
            while j < n && pattern[i] != pattern[j] {
                j = table[j];
            }
        }
        j += 1; if j > n { j = 0; }
    }
    table[n] = j; table
}

pub fn kmp_search<T: PartialEq>(pattern: &[T], target: &[T]) -> Vec<usize> {
    let n = pattern.len();
    let m = target.len();
    let table = kmp_table(&pattern);
    let (mut j, mut k) = (0, 0);
    let mut res = Vec::new();
    while j < m {
        if pattern[k] == target[j] {
            j += 1; k += 1;
            if k == n {
                res.push(j - k);
                k = table[k];
            }
        } else {
            k = table[k];
            if k > n { j += 1; k = 0; }
        }
    }
    res
}

// ------------ KMP algorithm end ------------

// ------------ run length start ------------
pub fn runlength<T>(l: &[T]) -> Vec<(T, usize)> where
    T: PartialEq + Copy,
{
    let mut res = Vec::new();
    let mut s = l.iter();
    let cur = s.next();
    if cur.is_none() { return res; }
    let mut cur = *cur.unwrap();
    let mut cnt = 1;
    for &x in s {
        if x != cur {
            res.push((cur, cnt));
            cur = x; cnt = 1;
        } else {
            cnt += 1;
        }
    }
    if cnt > 0 {
        res.push((cur, cnt));
    }
    res
}
// ------------ run length end ------------

#[cfg(test)]
mod tests {
    use super::z_algorithm;
    use std::iter;
    use rand::prelude::*;

    fn z_brute<T: Ord>(s: &[T]) -> Vec<usize> {
        let n = s.len();
        (0..n).map(|i| {
            s.iter().zip(s[i..].iter())
            .take_while(|&(i, j)| i == j)
            .count()
        }).collect()
    }

    #[test]
    fn test_z_hand() {
        let cases = ["abcabca", "abracadabra", "mississippi"];
        for &s in &cases {
            assert_eq!(z_brute(s.as_bytes()), z_algorithm(s.as_bytes()));
        }
    }

    #[test]
    fn test_z_random() {
        let mut rng = StdRng::seed_from_u64(137);
        for _ in 0..20 {
            let n = rng.gen_range(30, 100);
            let s = iter::repeat_with(|| rng.sample(rand::distributions::Alphanumeric))
                .take(n)
                .collect::<String>();
            assert_eq!(z_brute(s.as_bytes()), z_algorithm(s.as_bytes()));
        }
    }

}
