use crate::utils::bounds::bounds_within;

// verified:
// range affine range sum: https://judge.yosupo.jp/submission/61973
// ------------ Lazy Segment Tree start ------------

pub trait Ring {
    type Val: Clone + PartialEq;
    type Eff: Clone + PartialEq;
    const ZERO_VAL: Self::Val;
    const ZERO_EFF: Self::Eff;
    fn op_val(left: &Self::Val, right: &Self::Val) -> Self::Val;
    fn op_eff(left: &Self::Eff, right: &Self::Eff) -> Self::Eff;
    fn effect(val: &Self::Val, eff: &Self::Eff) -> Self::Val;
    fn multiply(eff: &Self::Eff, _times: u32) -> Self::Eff { eff.clone() }
}

#[derive(Clone)]
struct Node<T, E> {
    val: T,
    lazy: E,
}

impl<T, E> Node<T, E> {
    fn new(val: T, lazy: E) -> Self {
        Self { val, lazy }
    }
}

pub struct LazySegmentTree<R: Ring> {
    node: Box<[Node<R::Val, R::Eff>]>,
    size: usize,
    dep: u32,
}

impl<R: Ring> LazySegmentTree<R> {
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        let dep = size.trailing_zeros() + 1;
        let node = vec![Node::new(R::ZERO_VAL, R::ZERO_EFF); size << 1]
            .into_boxed_slice();
        Self { node, size, dep }
    }

    #[inline]
    fn degree(&self, i: usize) -> u32 {
        1 << (i.leading_zeros() + self.dep - 64)
    }

    fn effect(&mut self, i: usize, e: &R::Eff) {
        if i < self.size << 1 {
            self.node[i].val = R::effect(&self.node[i].val, &R::multiply(e, self.degree(i)));
            self.node[i].lazy = R::op_eff(&self.node[i].lazy, e);
        }
    }

    fn push(&mut self, i: usize) {
        let e = std::mem::replace(&mut self.node[i].lazy, R::ZERO_EFF);
        if e != R::ZERO_EFF {
            self.effect(i << 1, &e);
            self.effect((i << 1) + 1, &e);
        }
    }

    fn infuse(&mut self, mut i: usize) {
        i >>= i.trailing_zeros();
        while i > 1 {
            i >>= 1;
            self.node[i].val = R::op_val(&self.node[i << 1].val, &self.node[(i << 1) + 1].val);
        }
    }

    fn infiltrate(&mut self, i: usize) {
        if i < self.size << 1 {
            let d = i.trailing_zeros();
            for j in (d..self.dep).rev() {
                self.push(i >> j);
            }
        }
    }

    pub fn update<Rng: std::ops::RangeBounds<usize>>(&mut self, rng: Rng, e: R::Eff) {
        let rng = bounds_within(rng, self.size);
        let mut l = rng.start + self.size;
        let mut r = rng.end + self.size;
        self.infiltrate(l);
        self.infiltrate(r);
        while l < r {
            if l & 1 == 1 {
                self.effect(l, &e);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self.effect(r, &e);
            }
            l >>= 1;
            r >>= 1;
        }
        self.infuse(rng.start + self.size);
        self.infuse(rng.end + self.size);
    }

    pub fn fold<Rng: std::ops::RangeBounds<usize>>(&mut self, rng: Rng) -> R::Val {
        let rng = bounds_within(rng, self.size);
        let mut l = rng.start + self.size;
        let mut r = rng.end + self.size;
        self.infiltrate(l);
        self.infiltrate(r);
        let mut lx = R::ZERO_VAL;
        let mut rx = R::ZERO_VAL;
        while l < r {
            if l & 1 == 1 {
                lx = R::op_val(&lx, &self.node[l].val);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                rx = R::op_val(&self.node[r].val, &rx);
            }
            l >>= 1;
            r >>= 1;
        }
        R::op_val(&lx, &rx)
    }
}

impl<R: Ring> From<&Vec<R::Val>> for LazySegmentTree<R> {
    fn from(arr: &Vec<R::Val>) -> Self {
        let size = arr.len().next_power_of_two();
        let dep = size.trailing_zeros() + 1;
        let mut node = vec![Node::new(R::ZERO_VAL, R::ZERO_EFF); size << 1];
        for i in 0..arr.len() {
            node[i + size].val = arr[i].clone();
        }
        for i in (1..size).rev() {
            node[i].val = R::op_val(&node[i << 1].val, &node[(i << 1) + 1].val);
        }
        Self { node: node.into_boxed_slice(), size, dep }
    }
}

// ------------ Lazy Segment Tree end ------------

#[cfg(test)]
mod rmq_ruq_test {
    use super::*;

    struct RuqRmq;

    impl Ring for RuqRmq {
        type Val = i32;
        type Eff = Option<i32>;
        const ZERO_VAL: Self::Val = std::i32::MAX;
        const ZERO_EFF: Self::Eff = None;
        fn op_val(left: &Self::Val, right: &Self::Val) -> Self::Val {
            *left.min(right)
        }
        fn op_eff(left: &Self::Eff, right: &Self::Eff) -> Self::Eff {
            if right.is_none() { *left } else { *right }
        }
        fn effect(val: &Self::Val, eff: &Self::Eff) -> Self::Val {
            if let Some(v) = eff { *v } else { *val }
        }
    }

    #[test]
    fn lazy_segment_tree_ruq_rmq() {
        let mut lis = vec![3, 1, 4, 1, 5, 9];
        let mut seg = LazySegmentTree::<RuqRmq>::from(&lis);
        for j in 0..lis.len() {
            for i in 0..j {
                assert_eq!(seg.fold(i..j), *lis[i..j].iter().min().unwrap());
            }
        }

        let effs = vec![(2..5, 3), (0..3, 8), (2..3, 9)];
        for (rng, val) in effs {
            seg.update(rng.clone(), Some(val));
            for i in rng {
                lis[i] = val;
            }
        }

        for j in 0..lis.len() {
            for i in 0..j {
                assert_eq!(seg.fold(i..j), *lis[i..j].iter().min().unwrap());
            }
        }
    }
}
