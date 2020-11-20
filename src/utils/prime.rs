use crate::utils::bitset::BitSet;
use crate::utils::math::*;

pub fn atkin_sieve(n: usize) -> Vec<i64> {
    let mut sieve = BitSet::new(n+1);
    let lim = (n as f64).sqrt() as usize + 1;

    for z in (1..6).step_by(4) {
        for y in (z..lim).step_by(6) {
            for x in 1..lim {
                if 4 * x * x + y * y > n { break; }
                sieve.flip(4 * x * x + y * y);
            }
            for x in (y+1..lim).step_by(2) {
                if 3 * x * x - y * y > n { break; }
                sieve.flip(3 * x * x - y * y);
            }
        }
    }

    for z in (2..5).step_by(2) {
        for y in (z..lim).step_by(6) {
            for x in (1..lim).step_by(2) {
                if 3 * x * x + y * y > n { break; }
                sieve.flip(3 * x * x + y * y);
            }
            for x in (y+1..lim).step_by(2) {
                if 3 * x * x - y * y > n { break; }
                sieve.flip(3 * x * x - y * y);
            }
        }
    }

    for z in 1..3 {
        for y in (3..lim).step_by(6) {
            for x in (z..lim).step_by(3) {
                if 4 * x * x + y * y > n { break; }
                sieve.flip(4 * x * x + y * y);
            }
        }
    }

    for i in 5..lim {
        if sieve.access(i) {
            for j in (i*i..n+1).step_by(i*i) {
                sieve.set(j, false);
            }
        }
    }
    sieve.set(2, true);
    sieve.set(3, true);
    sieve.collect().into_iter().map(|x| x as i64).collect()
}


pub fn factorize(x: i64) -> Vec<(i64, usize)> {
    let mut y = x;
    let mut res = Vec::new();
    for i in 2..x+1 {
        if i * i > x { break; }
        if y % i == 0 {
            let mut cnt = 0;
            while y % i == 0 {
                y /= i;
                cnt += 1;
            }
            res.push((i, cnt));
        }
    }
    if y > 1 { res.push((y, 1)); }
    res
}

pub fn divisor(x: i64) -> Vec<i64> {
    let mut res = Vec::new();
    for i in 1..x+1 {
        if i * i > x { break; }
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
        res = res * (i - 1) / i;
    }
    res
}

#[allow(clippy::many_single_char_names)]
pub fn pollard_rho(v: i64, seed: i64) -> i64 {
    if v == 0 { return 1;}
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
    if a < 2 { return Some(a); }
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
        if i == p { return None; }
        let b = modpow(c, modpow(2, m - i - 1, p - 1), p);
        m = i;
        c = b * b % p;
        t = t * c % p;
        r = r * b % p;
    }
    Some(r)
}

// TODO: Lehmer's algorithm

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