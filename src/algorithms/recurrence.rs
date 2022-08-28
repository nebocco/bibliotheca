use crate::utils::math::modinv;

// verified: https://judge.yosupo.jp/submission/64373
/// for the given sequence a0, a1, ..., aN-1,
/// Find a sequence of integers c_0 = 1, c_1, ..., c_d of the minimum length d s.t.
/// \sum_{j=0}^{d} c[j] * a[i-j] == 0 for d <= i < N.
///
/// for FPS F(x) = P(x) / Q(x),
/// input: F(x) mod x^N
/// output: P(x), Q(x)
pub fn berlekamp_massey(a: &[i64], modulo: i64) -> Vec<i64> {
    let mut res = vec![1]; // c
    let mut prv_c = vec![1];
    let mut len_c = 0; // c.len() - 1
    let mut ptr = 1;
    let mut inv_prv_d = 1;

    for n in 0..a.len() {
        let d = (0..=len_c).map(|i| res[i] * a[n - i] % modulo).sum::<i64>() % modulo;
        if d == 0 {
            ptr += 1;
        } else if 2 * len_c <= n {
            let t = res.clone();
            if res.len() < prv_c.len() + ptr {
                res.resize(prv_c.len() + ptr, 0);
            }
            for i in 0..prv_c.len() {
                res[i + ptr] =
                    (res[i + ptr] - d * prv_c[i] % modulo * inv_prv_d).rem_euclid(modulo);
            }
            len_c = n + 1 - len_c;
            prv_c = t;
            inv_prv_d = modinv(d, modulo);
            ptr = 1;
        } else {
            if res.len() < prv_c.len() + ptr {
                res.resize(prv_c.len() + ptr, 0);
            }
            for i in 0..prv_c.len() {
                res[i + ptr] =
                    (res[i + ptr] - d * prv_c[i] % modulo * inv_prv_d).rem_euclid(modulo);
            }
            ptr += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_res(s: &[i64], res: &[i64], modulo: i64) {
        for i in res.len()..=s.len() {
            let d = s[i - res.len()..]
                .iter()
                .zip(res.iter().rev())
                .map(|(x, y)| x * y % modulo)
                .sum::<i64>()
                % modulo;
            assert_eq!(d, 0)
        }
    }

    #[test]
    fn berlekamp_massey_hand() {
        const MOD: i64 = 998244353;

        let s = vec![3, 4, 6, 10, 18, 34];
        let res = berlekamp_massey(&s, MOD);
        check_res(&s, &res, MOD);

        let s = vec![3, 4, 6, 10, 18, 36];
        let res = berlekamp_massey(&s, MOD);
        check_res(&s, &res, MOD);

        let s = vec![];
        let res = berlekamp_massey(&s, MOD);
        check_res(&s, &res, MOD);

        let s = vec![0, 0, 0, 0, 1];
        let res = berlekamp_massey(&s, MOD);
        check_res(&s, &res, MOD);
    }
}
