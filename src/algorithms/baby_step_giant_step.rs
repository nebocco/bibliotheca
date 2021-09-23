use crate::utils::math::modpow;

use std::collections::HashMap;

// * verified: https://judge.yosupo.jp/submission/30687
/// solve k s.t. x.pow(k) === y (mod M)
pub fn baby_giant(x: i64, y: i64, modulo: i64) -> Option<i64> {
    if (y - 1) % modulo == 0 {
        return Some(0);
    }
    if y == 0 {
        let (mut lo, mut hi) = (0, modulo);
        while hi - lo > 1 {
            let mid = (hi + lo) / 2;
            if modpow(x, mid, modulo) == 0 {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        return if modpow(x, hi, modulo) == 0 {
            Some(hi)
        } else {
            None
        };
    }
    let mut dic: HashMap<i64, i64> = HashMap::new();
    let sq = (modulo as f64).sqrt() as i64 + 1;
    let mut z = y;
    for i in 0..sq {
        // dic[y * x ^ i] = i
        dic.insert(z, i);
        z = z * x % modulo;
    }
    let r = modpow(x, sq, modulo); // r = x ^ (-sq)
    let mut c = 1;
    for i in 1..=sq {
        c = c * r % modulo;
        if let Some(v) = dic.get(&c) {
            let res = i * sq - v;
            return if modpow(x, res, modulo) == y {
                Some(res)
            } else {
                None
            };
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mini_test() {
        let x: i64 = 3;
        let y: i64 = 193;
        const MOD: i64 = 1_000_000_007;
        let r = baby_giant(x, y, MOD).unwrap_or(0);
        assert_eq!(modpow(x, r, MOD), y);
    }

    #[test]
    fn test_random() {
        use rand::prelude::*;
        const MOD: i64 = 998_244_353;
        let mut rng = thread_rng();
        let mut f = Vec::new();
        for _ in 0..20 {
            let x = rng.gen::<i64>() % MOD;
            let y = rng.gen::<i64>() % MOD;
            let r = baby_giant(x, y, MOD);
            if r.is_some() {
                f.push(r.unwrap());
                assert_eq!(modpow(x, r.unwrap(), MOD), y);
            }
        }
    }
}
