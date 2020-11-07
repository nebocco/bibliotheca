#![allow(dead_code)]

struct RollingHash {
    base: u64,
    modulo: u64,
    data: Vec<u64>
}

impl RollingHash {
    fn from(string: Vec<u8>) -> Self {
        let n = string.len();
        let mut data = vec![0; n+1];
        let base = 79;
        let modulo = 1_000_000_009;
        for (i, e) in string.into_iter().enumerate() {
            data[i+1] = (data[i] * base + e as u64) % modulo;
        }
        Self {
            base, modulo, data
        }
    }

    fn hash(&self, rng: std::ops::Range<usize>) -> u64 {
        let l = rng.start;
        let r = rng.end;
        (self.data[r] + self.modulo - self.data[l] * self.modpow(r - l) % self.modulo) % self.modulo
    }

    fn modpow(&self, mut s: usize) -> u64 {
        let mut res = 1;
        let mut r = self.base;
        while s > 0 {
            if s & 1 > 0 {
                res = res * r % self.modulo;
            }
            r = r * r % self.modulo;
            s >>= 1;
        }
        res
    }
}