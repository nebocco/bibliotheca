// * verified: https://judge.yosupo.jp/submission/30784
// TODO: should be faster
// ------------ Li Chao Tree start ------------

use std::ops::{ Add, Mul };

pub trait LineNumber: Add<Output=Self> + Mul<Output=Self> + Copy + PartialOrd {}
impl LineNumber for i64 {}
impl LineNumber for f64 {}

#[derive(Clone, Debug)]
pub struct Line<T> { pub a: T, pub b: T, }

impl<T: LineNumber> Line<T> {
    pub fn new(a: T, b: T) -> Self { Line { a, b } }
    pub fn get(&self, x: T) -> T { self.a * x + self.b }
}

pub struct LiChaoTree<T> {
    node: Box<[Option<Line<T>>]>,
    xs: Box<[T]>,
    sz: usize,
}

impl<T: LineNumber> LiChaoTree<T> {
    pub fn new(xs: &[T]) -> Self {
        let sz = xs.len().next_power_of_two();
        let mut xs = xs.to_vec();
        xs.resize(sz, *xs.last().unwrap());
        Self {
            node: vec![None; sz << 1].into_boxed_slice(),
            xs: xs.into_boxed_slice(),
            sz,
        }
    }

    fn update_range(&mut self, mut i: usize, mut l: usize, mut r: usize, mut line: Line<T>) {
        while i < (self.sz << 1) {
            if let Some(li) = self.node[i].take() {
                let m = (l + r) >> 1;
                let bl = line.get(self.xs[l]) < li.get(self.xs[l]);
                let bm = line.get(self.xs[m]) < li.get(self.xs[m]);
                let br = line.get(self.xs[r - 1]) < li.get(self.xs[r - 1]);
                if bm {
                    self.node[i] = Some(std::mem::replace(&mut line, li));
                } else {
                    self.node[i] = Some(li)
                }
                if bl == br { break; }
                if bl != bm {
                    r = m;
                    i <<= 1;
                } else {
                    l = m;
                    i = (i << 1) + 1;
                }
            } else {
                self.node[i] = Some(line);
                break
            };
        }
    }

    pub fn add_line(&mut self, line: Line<T>) {
        self.update_range(1, 0, self.sz, line);
    }

    pub fn add_segment(&mut self, mut l: usize, mut r: usize, line: Line<T>) {
        let mut left = l;
        let mut right = r;
        l += self.sz;
        r += self.sz;
        let mut len = 1;
        while l < r {
            if l & 1 == 1 {
                self.update_range(l, left, left + len, line.clone());
                l += 1;
                left += len;
            }
            if r & 1 == 1 {
                r -= 1;
                self.update_range(r, right - len, right, line.clone());
                right -= len;
            }
            l >>= 1;
            r >>= 1;
            len <<= 1;
        }
    }

    pub fn get_min(&self, mut i: usize) -> Option<T> {
        let x = self.xs[i];
        i += self.sz;
        let mut ans = None;
        while i > 0 {
            let res = self.node[i].as_ref().map(|l| l.get(x));
            ans = match (ans, res) {
                (Some(a), Some(b)) if a < b => Some(a),
                (Some(_), Some(b)) => Some(b),
                (Some(a), _) => Some(a),
                (None, b) => b,
            };
            i >>= 1;
        }
        ans
    }
}

// ------------ Li Chao Tree end ------------


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lct() {
        let xs = vec![-5, -1, 0, 1, 2];
        let mut lct = LiChaoTree::new(&xs);
        lct.add_line(Line::new(-1, -1));
        lct.add_line(Line::new(0, 1));
        assert_eq!(lct.get_min(1).unwrap(), 0);
        assert_eq!(lct.get_min(0).unwrap(), 1);
        assert_eq!(lct.get_min(2).unwrap(), -1);
        assert_eq!(lct.get_min(4).unwrap(), -3);
        lct.add_line(Line::new(0, -10));
        assert_eq!(lct.get_min(1).unwrap(), -10);
        assert_eq!(lct.get_min(3).unwrap(), -10);
        assert_eq!(lct.get_min(4).unwrap(), -10);
    }
}