use crate::utils::math::modpow;

use std::collections::HashMap;

/// solve k s.t. x.pow(k) === y (mod M)
pub fn baby_giant(x: i64, y: i64, modulo: i64) -> Option<i64> {
    let mut dic: HashMap<i64, i64> = HashMap::new();
    dic.insert(1, 0);
    let sq = (modulo as f64).sqrt() as i64 + 1;
    let mut z = 1;
    for i in 1..sq+1 {
        z = z * x % modulo;
        dic.insert(z, i);
    }
    let mut y = y;
    let r = modpow(z, modulo - 2, modulo); // r = x ^ (-sq)
    for i in 0..sq+1 {
        if let Some(v) = dic.get(&y) {
            return Some(v + i * sq);
        }
        y = y * r % modulo;
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
