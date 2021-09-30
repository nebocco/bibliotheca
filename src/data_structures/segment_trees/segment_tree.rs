use crate::utils::bounds::bounds_within;
use std::ops::{Index, Range, RangeBounds};


// ------------ Segment Tree start ------------
pub trait Monoid {
    type Val: Clone;
    fn zero() -> Self::Val;
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
}

pub struct SegmentTree<M: Monoid> {
    n: usize,
    size: usize,
    node: Vec<M::Val>,
}

impl<M: Monoid> SegmentTree<M> {
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        let node = vec![M::zero(); size * 2];
        SegmentTree { n, size, node }
    }

    pub fn set(&mut self, mut i: usize, x: M::Val) {
        i += self.size;
        self.node[i] = x;
        self.fix(i);
    }

    fn fix(&mut self, mut i: usize) {
        while i > 0 {
            i >>= 1;
            self.node[i] = M::op(&self.node[i << 1], &self.node[(i << 1) + 1]);
        }
    }

    pub fn fold<R: RangeBounds<usize>>(&self, rng: R) -> M::Val {
        let Range { start, end } = bounds_within(rng, self.size);
        let mut vl = M::zero();
        let mut vr = M::zero();
        let mut l = start + self.size;
        let mut r = end + self.size;
        while l < r {
            if l & 1 == 1 {
                vl = M::op(&vl, &self.node[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                vr = M::op(&self.node[r], &vr);
            }
            l >>= 1;
            r >>= 1;
        }
        M::op(&vl, &vr)
    }

    /// (j, Val) => pred(j-1) = true, pred(j) = false
    pub fn partition(&self, pred: impl Fn(usize, &M::Val) -> bool) -> (usize, M::Val) {
        assert!(pred(0, &M::zero()), "need to be pred(0, Val::zero())");
        if pred(self.n - 1, &self.node[1]) {
            return (self.n - 1, self.node[1].clone());
        }
        let mut j = 1;
        let mut current = M::zero();
        let mut idx = 0;
        let mut f = self.size;
        while j < self.size {
            j <<= 1;
            f >>= 1;
            let next = M::op(&current, &self.node[j]);
            if pred(idx + f - 1, &next) {
                current = next;
                j |= 1;
                idx += f;
            }
        }
        (idx, current)
    }
}

impl<M: Monoid> From<&Vec<M::Val>> for SegmentTree<M> {
    fn from(vec: &Vec<M::Val>) -> Self {
        let n = vec.len();
        let size = n.next_power_of_two();
        let mut node = vec![M::zero(); size << 1];
        for (i, e) in vec.iter().cloned().enumerate() {
            node[i + size] = e;
        }
        for i in (1..size).rev() {
            node[i] = M::op(&node[i << 1], &node[(i << 1) + 1]);
        }
        SegmentTree { n, size, node }
    }
}

impl<M: Monoid> Index<usize> for SegmentTree<M> {
    type Output = M::Val;
    fn index(&self, i: usize) -> &Self::Output {
        assert!(
            i < self.size,
            "index out of range: length is {}, but given {}.",
            self.size,
            i
        );
        &self.node[i + self.size]
    }
}
// ------------ Segment Tree end ------------

#[cfg(test)]
mod tests {
    use super::*;

    struct Rmq;
    impl Monoid for Rmq {
        type Val = i64;
        fn zero() -> Self::Val { std::i64::MAX }
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            *left.min(right)
        }
    }
    
    #[test]
    fn segment_tree_rmq() {
        let vec = vec![1, 2, 3];
        let mut seg = SegmentTree::<Rmq>::from(&vec);
        assert!(seg.fold(0..2) == 1);
        assert!(seg.fold(1..2) == 2);
        seg.set(1, 5);
        assert!(seg.fold(0..2) == 1);
        assert!(seg.fold(2..3) == 3);
        seg.set(2, 0);
        assert!(seg.fold(0..2) == 1);
        assert!(seg.fold(1..3) == 0);
    }

    struct Raq;
    impl Monoid for Raq {
        type Val = i32;
        fn zero() -> Self::Val { 0 }
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left + right
        }
    }

    #[test]
    fn segment_tree_raq() {
        let vec = vec![1, 2, 3];
        let mut seg = SegmentTree::<Raq>::from(&vec);
        assert!(seg.fold(0..2) == 3);
        assert!(seg.fold(1..2) == 2);
        seg.set(1, 5);
        assert!(seg.fold(0..2) == 6);
        assert!(seg.fold(1..2) == 5);
        seg.set(1, seg[1] + 5);
        assert!(seg.fold(0..2) == 11);
        assert!(seg.fold(1..2) == 10);
    }
}
