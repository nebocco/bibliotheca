use crate::utils::bounds::bounds_within;
use std::ops::{Range, RangeBounds};

// ------------ Rolling Hash start ------------

#[allow(dead_code)]
pub struct SingleRollingHash {
    base: i64,
    modulo: i64,
    data: Vec<i64>,
    pow: Vec<i64>,
}

impl SingleRollingHash {
    pub fn from(string: &[u8], base: i64, modulo: i64) -> Self {
        let n = string.len();
        let mut data = vec![0; n + 1];
        for (i, &e) in string.iter().enumerate() {
            data[i + 1] = (data[i] * base + e as i64) % modulo;
        }
        let mut pow = vec![1; n + 1];
        for i in 1..=n {
            pow[i] = pow[i - 1] * base % modulo;
        }
        Self {
            base,
            modulo,
            data,
            pow,
        }
    }

    pub fn hash<R: RangeBounds<usize>>(&self, rng: R) -> i64 {
        let Range { start, end } = bounds_within(rng, self.data.len() - 1);
        let res = self.data[end] - self.data[start] * self.pow[end - start] % self.modulo;
        if res < 0 {
            res + self.modulo
        } else {
            res
        }
    }
}

#[allow(dead_code)]
pub struct RollingHash {
    base1: i64,
    base2: i64,
    mod1: i64,
    mod2: i64,
    hash1: SingleRollingHash,
    hash2: SingleRollingHash,
}

impl RollingHash {
    pub fn from(string: &[u8]) -> Self {
        let base1 = 79;
        let base2 = 97;
        let mod1 = 1_000_000_009;
        let mod2 = 998_244_353;
        Self {
            base1,
            base2,
            mod1,
            mod2,
            hash1: SingleRollingHash::from(string, base1, mod1),
            hash2: SingleRollingHash::from(string, base2, mod2),
        }
    }

    pub fn hash<R: Clone + RangeBounds<usize>>(&self, rng: R) -> i64 {
        self.hash1.hash(rng.clone()) * self.mod2 + self.hash2.hash(rng)
    }
}

// ------------ Rolling Hash end ------------

#[cfg(test)]
mod tests {
    // TODO: make tests
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
