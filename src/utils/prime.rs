use crate::utils::bitset::BitSet;
use crate::utils::math::*;

pub fn atkin_sieve(n: usize) -> Vec<i64> {
    let mut sieve = BitSet::new(n + 1);
    let lim = (n as f64).sqrt() as usize + 1;

    for z in (1..6).step_by(4) {
        for y in (z..lim).step_by(6) {
            for x in 1..lim {
                if 4 * x * x + y * y > n {
                    break;
                }
                sieve.flip(4 * x * x + y * y);
            }
            for x in (y + 1..lim).step_by(2) {
                if 3 * x * x - y * y > n {
                    break;
                }
                sieve.flip(3 * x * x - y * y);
            }
        }
    }

    for z in (2..5).step_by(2) {
        for y in (z..lim).step_by(6) {
            for x in (1..lim).step_by(2) {
                if 3 * x * x + y * y > n {
                    break;
                }
                sieve.flip(3 * x * x + y * y);
            }
            for x in (y + 1..lim).step_by(2) {
                if 3 * x * x - y * y > n {
                    break;
                }
                sieve.flip(3 * x * x - y * y);
            }
        }
    }

    for z in 1..3 {
        for y in (3..lim).step_by(6) {
            for x in (z..lim).step_by(3) {
                if 4 * x * x + y * y > n {
                    break;
                }
                sieve.flip(4 * x * x + y * y);
            }
        }
    }

    for i in 5..lim {
        if sieve.access(i) {
            for j in (i * i..n + 1).step_by(i * i) {
                sieve.set(j, false);
            }
        }
    }
    sieve.set(2, true);
    sieve.set(3, true);
    sieve.collect().into_iter().map(|x| x as i64).collect()
}

pub fn factorize(x: i64) -> Vec<(i64, usize)> {
    let mut res = Vec::new();
    if x < 2 {
        return res;
    }
    let mut y = x;
    if y & 1 == 0 {
        let t = y.trailing_zeros() as usize;
        res.push((2, t));
        y >>= t;
    }
    for i in (3..=y).step_by(2) {
        if i * i > y {
            break;
        }
        if y % i == 0 {
            let mut cnt = 0;
            while y % i == 0 {
                y /= i;
                cnt += 1;
            }
            res.push((i, cnt));
        }
    }
    if y > 1 {
        res.push((y, 1));
    }
    res
}

pub fn divisor(x: i64) -> Vec<i64> {
    let mut res = Vec::new();
    for i in 1..x + 1 {
        if i * i > x {
            break;
        }
        if x % i == 0 {
            res.push(i);
            if i * i < x {
                res.push(x / i);
            }
        }
    }
    res
}

pub fn totient(x: i64) -> i64 {
    let mut res = x;
    for &(i, _) in factorize(x).iter() {
        res = res / i * (i - 1);
    }
    res
}

#[allow(clippy::many_single_char_names)]
pub fn pollard_rho(v: i64, seed: i64) -> i64 {
    if v == 0 {
        return 1;
    }
    let seed = seed.wrapping_mul(v);
    let c = seed & 0xff;
    let u = c & 0x7f;
    let mut r: i64 = 1;
    let mut q: i64 = 1;
    let mut y: i64 = u & 0x0f;
    let mut fac: i64 = 1;
    let mut y_old: i64 = 0;
    let mut x: i64 = 0;
    let func = |x: i64| (x.wrapping_mul(x) + c) % v;
    while fac == 1 {
        x = y;
        for _ in 0..r {
            y = func(y);
        }
        let mut k = 0;
        while k < r && fac == 1 {
            y_old = y;
            for _ in 0..std::cmp::min(u, r - k) {
                y = func(y);

                if x > y {
                    q = q.wrapping_mul(x - y) % v;
                } else {
                    q = q.wrapping_mul(y - x) % v;
                }
            }
            fac = gcd(q, v);
            k += u;
        }
        r <<= 1;
    }
    while fac == v || fac <= 1 {
        y_old = func(y_old);

        if x > y_old {
            q = q.wrapping_mul(x - y_old) % v;
        } else {
            q = q.wrapping_mul(y_old - x) % v;
        }
        fac = gcd(q, v);
    }
    fac
}

// * verified: https://judge.yosupo.jp/submission/30338
// Tonelli-Shanks algorithm
#[allow(clippy::many_single_char_names)]
pub fn mod_sqrt(mut a: i64, p: i64) -> Option<i64> {
    a %= p;
    if a < 2 {
        return Some(a);
    }
    let s = (p - 1).trailing_zeros() as i64;
    let q = (p - 1) >> s;
    let mut z = 1;
    while modpow(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let mut m = s;
    let mut c = modpow(z, q, p);
    let mut t = modpow(a, q, p);
    let mut r = modpow(a, (q + 1) / 2, p);
    while t != 1 {
        let mut cur = t;
        let mut i = p;
        for j in 1..m {
            cur = cur * cur % p;
            if cur == 1 {
                i = j;
                break;
            }
        }
        if i == p {
            return None;
        }
        let b = modpow(c, modpow(2, m - i - 1, p - 1), p);
        m = i;
        c = b * b % p;
        t = t * c % p;
        r = r * b % p;
    }
    Some(r)
}

/// count the number of primes <= n.
/// O(n^(3/4) / log(n)) time, O(n^(1/2)) space.
/// TODO: implement O(n^(2/3) / log(n)) time algorithm
/// ```no_run
/// // S(v, p) = S(v, p-1) - (S(v/p, p-1) - Pi(p-1))
/// fn S(v: i64, p: i64) -> usize {
///     (2..=v).filter(|i| (2..=p).all(|j| i % j != 0)).count()
/// }
///
/// // smalls[i] = S(i, p);
/// // larges[i] = S(n/i, p);
/// ```
pub fn count_primes(n: usize) -> usize {
    let n_sqrt = sqrt_floor(n as i64) as usize;
    let mut larges = vec![0; n_sqrt + 1];
    for i in 1..=n_sqrt {
        larges[i] = n / i - 1;
    }
    let mut smalls: Vec<_> = (0..n / n_sqrt).map(|x| x.saturating_sub(1)).collect();

    for p in 2..=n_sqrt {
        if p < smalls.len() {
            if smalls[p] <= smalls[p - 1] {
                continue;
            }
        } else {
            if larges[n / p] <= smalls[p - 1] {
                continue;
            }
        }
        let pc = smalls[p - 1];
        let q = p * p;
        for i in (1..=n_sqrt).take_while(|&i| n / i >= q) {
            // vi = n / i
            // dp[n / i] -= dp[n / i / p] - pc;
            let ip = i * p;
            let cur = *larges.get(ip).unwrap_or_else(|| &smalls[n / ip]) - pc;
            larges[i] -= cur;
        }
        for i in (1..smalls.len()).rev().take_while(|&i| i >= q) {
            // vi = i
            // dp[i] -= dp[i / p] - pc;
            let cur = smalls[i / p] - pc;
            smalls[i] -= cur;
        }
    }
    larges[1]
}

// TODO: Lehmer's algorithm

///エラトステネスの篩
pub struct Eratosthenes {
    flags: Vec<u8>,
    n: usize,
}
impl Eratosthenes {
    const K_MASK: [[u8; 8]; 8] = [
        [0xfe, 0xfd, 0xfb, 0xf7, 0xef, 0xdf, 0xbf, 0x7f],
        [0xfd, 0xdf, 0xef, 0xfe, 0x7f, 0xf7, 0xfb, 0xbf],
        [0xfb, 0xef, 0xfe, 0xbf, 0xfd, 0x7f, 0xf7, 0xdf],
        [0xf7, 0xfe, 0xbf, 0xdf, 0xfb, 0xfd, 0x7f, 0xef],
        [0xef, 0x7f, 0xfd, 0xfb, 0xdf, 0xbf, 0xfe, 0xf7],
        [0xdf, 0xf7, 0x7f, 0xfd, 0xbf, 0xfe, 0xef, 0xfb],
        [0xbf, 0xfb, 0xf7, 0x7f, 0xfe, 0xef, 0xdf, 0xfd],
        [0x7f, 0xbf, 0xdf, 0xef, 0xf7, 0xfb, 0xfd, 0xfe],
    ];

    const C0: [[usize; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 0, 1, 1, 1, 1],
        [2, 2, 0, 2, 0, 2, 2, 1],
        [3, 1, 1, 2, 1, 1, 3, 1],
        [3, 3, 1, 2, 1, 3, 3, 1],
        [4, 2, 2, 2, 2, 2, 4, 1],
        [5, 3, 1, 4, 1, 3, 5, 1],
        [6, 4, 2, 4, 2, 4, 6, 1],
    ];
    const K_MOD_30: [usize; 8] = [1, 7, 11, 13, 17, 19, 23, 29];
    const C1: [usize; 8] = [6, 4, 2, 4, 2, 4, 6, 2];

    pub fn new(n: usize) -> Self {
        if n > 10_000_000_000 {
            panic!();
        }
        let size = (n + 30 - 1) / 30;
        let mut flags = vec![0xff; size];
        flags[0] = 0xfe;

        let r = n % 30;
        flags[size - 1] = match r {
            1..=1 => 0x0,
            2..=7 => 0x1,
            8..=11 => 0x3,
            12..=13 => 0x7,
            14..=17 => 0xf,
            18..=19 => 0x1f,
            20..=23 => 0x3f,
            24..=29 => 0x7f,
            _ => panic!(),
        };

        let quart_x = sqrt_floor(n as i64) as usize / 30 + 1;

        for i in 0..quart_x {
            let mut f: u8 = flags[i];

            while f != 0 {
                let i_bit = (f & f.wrapping_neg()).trailing_zeros() as usize;
                let m = Eratosthenes::K_MOD_30[i_bit];
                let mut k = i_bit;
                let mut j = i * (30 * i + 2 * m) + (m * m) / 30;
                while j < size {
                    flags[j] &= Eratosthenes::K_MASK[i_bit][k];
                    j += i * Eratosthenes::C1[k] + Eratosthenes::C0[i_bit][k];
                    k = (k + 1) & 7;
                }
                f &= f - 1;
            }
        }

        Eratosthenes { flags, n }
    }

    pub fn count(&self) -> usize {
        if self.n < 6 {
            (self.n + 1) >> 1 // count 2, 3, 5
        } else {
            3 + self.flags.iter().map(|x| x.count_ones()).sum::<u32>() as usize
        }
    }

    pub fn primes(&self) -> Vec<i64> {
        let mut ret = Vec::new();
        [2, 3, 5]
            .iter()
            .take_while(|&&x| self.n > x)
            .for_each(|&x| ret.push(x as i64));

        for (i, &f) in self.flags.iter().enumerate() {
            for (ii, &m) in Eratosthenes::K_MOD_30.iter().enumerate() {
                if (f & (1 << ii)) != 0 {
                    ret.push((30 * i + m) as i64);
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prime_brute(n: usize) -> Vec<i64> {
        let mut primes = Vec::new();
        for i in 2..=n {
            if (2..i).all(|j| i % j > 0) {
                primes.push(i as i64);
            }
        }
        primes
    }

    #[test]
    fn test_atkin() {
        for i in (1..1000).step_by(100) {
            assert_eq!(atkin_sieve(i), prime_brute(i));
        }
    }

    fn mod_sqrt_brute(modulo: usize) -> Vec<Option<i64>> {
        let mut res = vec![None; modulo];
        for i in 0..modulo {
            if res[i * i % modulo].is_none() {
                res[i * i % modulo] = Some(i as i64);
            }
        }
        res
    }

    #[test]
    fn test_mod_sqrt() {
        for modulo in vec![3, 5, 7, 11, 13, 17].into_iter() {
            let ans = mod_sqrt_brute(modulo as usize);
            for i in 0..modulo {
                let x = mod_sqrt(i, modulo);
                if let Some(v) = x {
                    assert_eq!(v * v % modulo, i);
                } else {
                    assert!(ans[i as usize].is_none())
                }
            }
        }
    }
}
