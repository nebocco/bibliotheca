#![allow(dead_code)]

use crate::data_structures::union_find::UnionFind;

// https://atcoder.jp/contests/agc005/submissions/22277740

pub struct DecrimentalPredecessor {
    n: usize,
    small: Vec<usize>,
    large: UnionFind,
}

impl DecrimentalPredecessor {
    pub fn new(n: usize) -> Self {
        let mut small = vec![!0; (n >> 6) + 1];
        let mut large = UnionFind::new(small.len() + 1);
        let b = n >> 6;
        let t = n & 63;
        small[b] = !((1 << t) - 1);
        if t == 0 {
            large.unite(b, b + 1).ok();
        }
        Self { n, small, large }
    }

    pub fn predecessor(&self, x: usize) -> Option<usize> {
        assert!(x <= self.n);
        let b = self.n >> 5;
        let m = self.small[b] & !((1 << (x & 63)) - 1);
        if m != 0 {
            return Some((x | 63) - x.trailing_zeros() as usize);
        }
        None
    }
}
