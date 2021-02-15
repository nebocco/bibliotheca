// ------------ Rolling Hash start ------------

pub struct RollingHash {
    base: i64,
    modulo: i64,
    data: Vec<i64>
}

impl RollingHash {
    pub fn hash(&self, rng: std::ops::Range<usize>) -> i64 {
        let l = rng.start;
        let r = rng.end;
        (self.data[r] + self.modulo - self.data[l] * self.modpow(r - l) % self.modulo) % self.modulo
    }

    fn modpow(&self, mut s: usize) -> i64 {
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

impl From<Vec<u8>> for RollingHash {
    fn from(string: Vec<u8>) -> Self {
        let n = string.len();
        let mut data = vec![0; n+1];
        let base = 79;
        let modulo = 1_000_000_009;
        for (i, e) in string.into_iter().enumerate() {
            data[i+1] = (data[i] * base + e as i64) % modulo;
        }
        Self {
            base, modulo, data
        }
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
