// https://atcoder.jp/contests/agc005/submissions/22277740
// ------------ Decrimental Predecessor start ------------
// ------------ Positional UnionFind start ------------
/// store leftist and rightist children of the group
#[derive(Clone, Debug)]
pub struct PositionalUnionFind {
    par: Vec<isize>,
    lr: Vec<(usize, usize)>,
}

impl PositionalUnionFind {
    fn new(len: usize) -> Self {
        let lr = (0..len).map(|x| (x, x)).collect();
        Self {
            par: vec![-1; len],
            lr,
        }
    }

    fn unite(&mut self, l: usize, r: usize) -> bool {
        let l = self.find(l);
        let r = self.find(r);
        if l == r {
            false
        } else if self.par[l] < self.par[r] {
            self.par[l] += self.par[r];
            self.par[r] = l as isize;
            self.lr[l].1 = self.lr[r].1;
            true
        } else {
            self.par[r] += self.par[l];
            self.par[l] = r as isize;
            self.lr[r].0 = self.lr[l].0;
            true
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.par[i] < 0 {
            i
        } else {
            let p = self.find(self.par[i] as usize);
            self.par[i] = p as isize;
            p
        }
    }

    fn left(&mut self, i: usize) -> usize {
        let p = self.find(i);
        self.lr[p].0
    }

    fn right(&mut self, i: usize) -> usize {
        let p = self.find(i);
        self.lr[p].1
    }
}
// ------------ UnionFind end ------------

pub struct DecrimentalPredecessor {
    n: usize,
    small: Vec<usize>,
    large: PositionalUnionFind,
}

impl DecrimentalPredecessor {
    /// Set = (0..n).collect();
    pub fn new(n: usize) -> Self {
        let mut small = vec![!0; (n >> 6) + 1];
        let mut large = PositionalUnionFind::new(small.len() + 1);
        let b = n >> 6;
        let t = n & 63;
        small[b] = !(!0 << t);
        if t == 0 {
            large.unite(b, b + 1);
        }
        Self { n, small, large }
    }

    /// max { v in Set | v < x }
    pub fn predecessor(&mut self, x: usize) -> Option<usize> {
        assert!(x <= self.n);
        let b = x >> 6;
        let m = self.small[b] & !(!0 << (x & 63));
        if m != 0 {
            return Some((b << 6) + 63 - m.leading_zeros() as usize);
        }
        let mut b = self.large.left(b);
        if b == 0 {
            None
        } else {
            b -= 1;
            Some((b << 6) + 63 - self.small[b].leading_zeros() as usize)
        }
    }

    /// min { v in Set | v >= x }
    pub fn successor(&mut self, x: usize) -> Option<usize> {
        assert!(x <= self.n);
        let b = x >> 6;
        let m = self.small[b] & !0 << (x & 63);
        if m != 0 {
            return Some((b << 6) + m.trailing_zeros() as usize);
        }
        let b = self.large.right(b + 1);
        if b == self.small.len() {
            None
        } else {
            Some((b << 6) + self.small[b].trailing_zeros() as usize)
        }
    }

    /// Set <- Set \ {x}
    pub fn erase(&mut self, x: usize) -> bool {
        let b = x >> 6;
        let t = x & 63;
        if self.small[b] & 1 << t == 0 {
            return false;
        }
        self.small[b] ^= 1 << t;
        if self.small[b] == 0 {
            self.large.unite(b, b + 1);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::random::XorShift;

    #[test]
    fn test_decrimental_predecessor() {
        let n = 3000;
        let mut dec = DecrimentalPredecessor::new(n);
        let mut rand = XorShift::new(27);
        let mut indice: Vec<usize> = (0..n).collect();
        for i in (1..n).rev() {
            let j = rand.gen() as usize % (i + 1);
            indice.swap(i, j);
        }
        while let Some(v) = indice.pop() {
            assert_eq!(dec.predecessor(v + 1), Some(v));
            assert_eq!(dec.successor(v), Some(v));
            assert_eq!(dec.erase(v), true);
            let p = indice.iter().filter(|&&x| x < v).max().cloned();
            let s = indice.iter().filter(|&&x| x > v).min().cloned();
            assert_eq!(dec.predecessor(v), p);
            assert_eq!(dec.successor(v), s);
            assert_eq!(dec.erase(v), false);
        }
    }
}
