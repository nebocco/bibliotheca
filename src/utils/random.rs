#[derive(Clone, Debug)]
pub struct XorShift(u64);

impl XorShift {
    pub fn new(seed: u64) -> Self { Self(seed) }
    pub fn gen(&mut self) -> u64 {
        let x = self.0;
        let x = (x << 13) ^ x;
        let x = (x >> 7) ^ x;
        let x = (x << 17) ^ x;
        self.0 = x;
        x
    }
}

impl Default for XorShift {
    fn default() -> Self {
        Self(88172645463325252)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_shift() {
        let mut rand = XorShift::default();
        let m = 100000; 
        let mut result: Vec<u64> = (0..m).map(|_| rand.gen()).collect();
        result.sort();
        result.dedup();
        assert_eq!(result.len(), m)
    }
}