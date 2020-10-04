use std::cmp::min;

pub fn z_algorithm<T: Ord>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    if n == 0 { return Vec::new(); }
    let mut ret = vec![0; n];
    let mut j = 0;
    for i in 1..n {
        ret[i] = (if j + ret[j] <= i {
            0
        } else {
            min(j + ret[j] - i, ret[i - j])
        }..)
            .find(|&k| i + k == n || s[k] != s[i + k])
            .unwrap();
        if j + ret[j] < i + ret[i] { j = i; }
    }
    ret[0] = n;
    ret
}

pub fn kmp_table<T: Ord>(pattern: &[T]) -> Vec<usize> {
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

pub fn kmp_search<T: Ord>(pattern: &[T], target: &[T]) -> Option<usize> {
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

pub fn runlength(s: &String) -> Vec<(char, usize)> {
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



#[cfg(test)]
mod tests {
    use super::z_algorithm;
    use std::iter;
    use rand::prelude::*;
    use test_case::test_case;

    fn z_brute<T: Ord>(s: &[T]) -> Vec<usize> {
        let n = s.len();
        (0..n).map(|i| {
            s.iter().zip(s[i..].iter())
            .take_while(|&(i, j)| i == j)
            .count()
        }).collect()
    }

    #[test_case("abcabca")]
    #[test_case("abracadabra")]
    #[test_case("mississippi")]
    fn test_z_hand(s: &str) {
        assert_eq!(z_brute(s.as_bytes()), z_algorithm(s.as_bytes()));
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
