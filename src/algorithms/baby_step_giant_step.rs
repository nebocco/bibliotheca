//! solve k s.t. x.pow(k) === y (mod M)
use std::collections::HashMap;
use crate::utils::math::modpow;

pub fn baby_giant(x: u64, y: u64, modulo: u64) -> Option<u64> {
    let mut dic: HashMap<u64, u64> = HashMap::new();
    dic.insert(1, 0);

    let sq = (modulo as f64).sqrt() as u64 + 1;

    // Baby-step
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
    fn it_works() {
        let x = 3;
        let y = 193;
        let modulo = 1_000_000_007;
        let r = baby_giant(x, y, modulo).unwrap_or(0);
        assert_eq!(modpow(x, r, modulo), y);
    }
}
