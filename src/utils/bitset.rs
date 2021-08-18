use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

#[derive(Clone)]
pub struct BitSet {
    data: Vec<u32>,
    size: usize,
}

impl BitSet {
    pub fn new(size: usize) -> Self {
        let data = vec![0; (size >> 5) + 1];
        BitSet { data, size }
    }

    pub fn fill(&mut self) {
        self.data.iter_mut().for_each(|x| *x = 0xffffffff);
    }

    pub fn access(&self, pos: usize) -> bool {
        (self.data[pos >> 5] >> (pos & 31)) & 1 == 1
    }

    pub fn set(&mut self, pos: usize, v: bool) {
        if v {
            self.data[pos >> 5] |= 1 << (pos & 31);
        } else {
            self.data[pos >> 5] &= !(1 << (pos & 31));
        }
    }

    pub fn flip(&mut self, pos: usize) {
        self.data[pos >> 5] ^= 1 << (pos & 31);
    }

    pub fn collect(&self) -> Vec<u64> {
        (0..self.size)
            .filter(|&i| self.access(i))
            .map(|x| x as u64)
            .collect::<Vec<u64>>()
    }

    fn resize(&mut self, l: usize) {
        if self.size > l {
            return;
        }
        self.data.resize((l >> 5) + 1, 0);
        self.size = l;
    }
}

impl BitAnd for BitSet {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self {
        let m = std::cmp::max(self.size, rhs.size);
        self.resize(m);
        for (u, v) in self.data.iter_mut().zip(rhs.data.iter()) {
            *u &= v;
        }
        self
    }
}

impl BitOr for BitSet {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self {
        let m = std::cmp::max(self.size, rhs.size);
        self.resize(m);
        for (u, v) in self.data.iter_mut().zip(rhs.data.iter()) {
            *u |= v;
        }
        self
    }
}

impl BitXor for BitSet {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self {
        let m = std::cmp::max(self.size, rhs.size);
        self.resize(m);
        for (u, v) in self.data.iter_mut().zip(rhs.data.iter()) {
            *u ^= v;
        }
        self
    }
}

impl Not for BitSet {
    type Output = Self;
    fn not(mut self) -> Self {
        for u in self.data.iter_mut() {
            *u = !*u;
        }
        self
    }
}

impl Shr<usize> for BitSet {
    type Output = Self;
    fn shr(mut self, rhs: usize) -> Self::Output {
        let big = rhs >> 5;
        let sml = (rhs & 31) as u32;
        let mask = (1 << sml) - 1;
        for i in 0..self.data.len() {
            self.data[i] = if i + big < self.data.len() {
                self.data[i + big]
            } else {
                0
            };
        }
        let mut r = 0;
        for i in (0..self.data.len()).rev() {
            let u = self.data[i];
            self.data[i] = (u & !mask | r).rotate_right(sml);
            r = u & mask;
        }
        self
    }
}

impl Shl<usize> for BitSet {
    type Output = Self;
    fn shl(mut self, rhs: usize) -> Self::Output {
        let n = self.data.len();
        let big = rhs >> 5;
        let sml = (rhs & 31) as u32;
        let mask = (1 << sml) - 1;
        for i in (0..n).rev() {
            self.data[i] = if i >= big { self.data[i - big] } else { 0 };
        }
        let mut r = 0;
        for i in 0..n {
            let u = self.data[i].rotate_left(sml);
            self.data[i] = u & !mask | r;
            r = u & mask;
        }
        self.data[n - 1] &= (1 << (self.size & 31)) - 1;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::BitSet;
    #[test]
    fn test_bitand() {
        let mut a = BitSet::new(100);
        let mut b = BitSet::new(300);
        a.set(40, true);
        a.set(80, true);
        b.fill();
        let c = a & b;
        assert_eq!(c.collect(), vec![40, 80]);
    }

    #[test]
    fn test_bitor() {
        let mut a = BitSet::new(100);
        let mut b = BitSet::new(300);
        a.set(40, true);
        a.set(80, true);
        b.set(30, true);
        b.set(170, true);
        let c = a | b;
        assert_eq!(c.collect(), vec![30, 40, 80, 170]);
    }

    #[test]
    fn test_bitxor() {
        let mut a = BitSet::new(100);
        let mut b = BitSet::new(300);
        a.set(40, true);
        a.set(80, true);
        b.set(40, true);
        b.set(170, true);
        let c = a ^ b;
        assert_eq!(c.collect(), vec![80, 170]);
    }

    #[test]
    fn test_not() {
        let mut a = BitSet::new(100);
        let mut b = BitSet::new(300);
        a.set(40, true);
        a.set(80, true);
        b.set(30, true);
        b.set(170, true);
        let c = !a & b;
        assert_eq!(c.collect(), vec![30]);
    }

    #[test]
    fn test_shift() {
        let mut a = BitSet::new(100);
        assert_eq!(a.data.len(), 4);
        a.set(1, true);
        a.set(40, true);
        a = a << 20;
        assert_eq!(a.collect(), vec![21, 60]);
        a = a << 60;
        assert_eq!(a.collect(), vec![81]);
        a.set(40, true);
        a.set(53, true);
        a = a >> 30;
        assert_eq!(a.collect(), vec![10, 23, 51]);
        a = a >> 51;
        assert_eq!(a.collect(), vec![0]);
        a = a << 600;
        assert_eq!(a.collect(), vec![]);
    }
}
