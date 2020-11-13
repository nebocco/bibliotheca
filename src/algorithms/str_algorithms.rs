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
        while k + ret[k] < p {
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
    let mut ret = vec![0; n];
    let mut j = 0;
    for i in 1..n {
        ret[i] = j;
        if pattern[i] == pattern[j] {
            j += 1;
        } else {
            j = 0;
        }
    }
    ret
}

pub fn kmp_search<T: PartialEq>(pattern: &[T], target: &[T]) -> Option<usize> {
    let n = pattern.len();
    let m = target.len();
    let table = kmp_table(&pattern);
    let mut i = 0;
    let mut p = 0;
    while i < m && p < n {
        if target[i] == pattern[p] {
            i += 1;
            p += 1;
        } else if p == 0 {
            i += 1;
        } else {
            p = table[p];
        }
    }
    if p == n {
        Some(i - p)
    } else {
        None
    }
}

// ------------ KMP algorithm end ------------

// ------------ run length start ------------

pub fn runlength(s: &str) -> Vec<(char, usize)> {
    let mut res = Vec::new();
    let mut cur: char = '$';
    let mut cnt = 0;
    for x in s.chars() {
        if x != cur {
            if cnt > 0 {
                res.push((cur, cnt));
            }
            cur = x;
            cnt = 1;
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
