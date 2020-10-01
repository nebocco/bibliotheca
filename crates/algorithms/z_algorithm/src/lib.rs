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
    fn test_hand(s: &str) {
        assert_eq!(z_brute(s.as_bytes()), z_algorithm(s.as_bytes()));
    }

    #[test]
    fn test_random() {
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