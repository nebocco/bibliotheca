// TODO: verify
// ------------ Kitamasa's algorithm start ------------

/// d_i = d[i] (0 <= i < k),
/// a_k = c_0 * a_0 + c_1 * a_1 + ... + c_{k-1} * a_{k-1}
/// calculate a_n
/// O(k^2 logk)
pub fn kitamasa(a: &[i64], c: &[i64], n: i64, modulo: i64) -> i64 {
    assert_eq!(a.len(), c.len());

    /// x = f(s): a_s = f(s) * a[0..k]
    /// calculate f(s+1) from f(s)
    /// O(k)
    fn plus_one(x: &[i64], c: &[i64], modulo: i64) -> Vec<i64> {
        assert_eq!(x.len(), c.len());
        let k = c.len();
        let mut res = vec![0; k];
        for i in 1..k {
            res[i] = (x[i - 1] + x[k - 1] * c[i]) % modulo;
        }
        res[0] = x[k - 1] * c[0] % modulo;
        res
    }

    /// calculate f(s*2) from f(s), f(s+1), ..., f(s+k-1)
    /// O(k^2)
    fn mult_two(x: &[i64], c: &[i64], modulo: i64) -> Vec<i64> {
        assert_eq!(x.len(), c.len());
        let k = c.len();
        let mut res = vec![0; k];
        let mut v = x.to_owned();
        for &xx in x {
            for j in 0..k {
                res[j] = (res[j] + xx * v[j]) % modulo;
            }
            v = plus_one(&v, c, modulo);
        }
        res
    }
    let k = a.len();
    if (n as usize) < k {
        return a[n as usize];
    }
    assert!(k > 1, "please use modpow().");
    let mut s = 0;
    let mut f = vec![0; k];
    f[0] = 1;
    for i in (0..64 - n.leading_zeros()).rev() {
        if s < k {
            for _ in 0..s {
                f = plus_one(&f, c, modulo);
            }
        } else {
            f = mult_two(&f, c, modulo)
        }
        s <<= 1;
        if n >> i & 1 == 1 {
            f = plus_one(&f, c, modulo);
            s |= 1;
        }
    }
    let mut res = 0;
    for i in 0..k {
        res = (res + f[i] * a[i]) % modulo;
    }
    res
}

// ------------ Kitamasa's algorithm end ------------

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    const MOD: i64 = 998244353;

    #[test]
    fn kitamasa_mini_test() {
        let a = vec![1, 1];
        let c = vec![1, 1];
        let mut x = 1;
        let mut y = 1;
        for s in 0..100 {
            assert_eq!(kitamasa(&a, &c, s, MOD), x);
            x = (x + y) % MOD;
            std::mem::swap(&mut x, &mut y);
        }
    }

    fn calc_brute(a: &[i64], c: &[i64], n: i64) -> i64 {
        let mut a = a.to_owned();
        let k = a.len();
        for _ in k..=n as usize {
            let mut res = 0;
            for i in 0..k {
                res = (res + a[i] * c[i]) % MOD;
            }
            a.rotate_left(1);
            a[k - 1] = res;
        }
        a[k - 1]
    }

    #[test]
    fn kitamasa_random_test() {
        let mut rng = thread_rng();
        for _ in 0..10 {
            let a = (0..60).map(|_| rng.gen_range(0..MOD)).collect::<Vec<_>>();
            let c = (0..60).map(|_| rng.gen_range(0..MOD)).collect::<Vec<_>>();
            let n = rng.gen_range(1_000..100_000);
            assert_eq!(kitamasa(&a, &c, n, MOD), calc_brute(&a, &c, n));
        }
    }
}
