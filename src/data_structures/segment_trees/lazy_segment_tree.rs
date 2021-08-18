use crate::utils::{algebraic_traits::Monoid, bounds::bounds_within};

use std::ops::{Mul, Range, RangeBounds};

// * verified: https://judge.yosupo.jp/submission/28350
// TODO: it might be able to make it faster
// ------------ Lazy Segment Tree start ------------

#[derive(Clone)]
struct Node<T: Monoid + Mul<E, Output = T>, E: Monoid> {
    val: T,
    lazy: E,
}

pub struct LazySegmentTree<T: Monoid + Mul<E, Output = T>, E: Monoid> {
    node: Box<[Node<T, E>]>,
    size: usize,
    dep: usize,
}

impl<T: Monoid + Mul<E, Output = T>, E: Monoid> LazySegmentTree<T, E> {
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        let dep = size.trailing_zeros() as usize + 1;
        let node = vec![
            Node {
                val: T::zero(),
                lazy: E::zero()
            };
            size << 1
        ];
        Self {
            node: node.into_boxed_slice(),
            size,
            dep,
        }
    }

    fn effect(&mut self, i: usize, e: &E) {
        if i < self.size << 1 {
            self.node[i].val = self.node[i].val.clone() * e.clone();
            self.node[i].lazy = self.node[i].lazy.clone() + e.clone();
        }
    }

    fn push(&mut self, i: usize) {
        let e = std::mem::replace(&mut self.node[i].lazy, E::zero());
        self.effect(i << 1, &e);
        self.effect((i << 1) + 1, &e);
    }

    fn infuse(&mut self, mut i: usize) {
        i >>= i.trailing_zeros();
        while i > 1 {
            i >>= 1;
            self.node[i].val = self.node[i << 1].val.clone() + self.node[(i << 1) + 1].val.clone();
        }
    }

    fn infiltrate(&mut self, i: usize) {
        if i < self.size << 1 {
            let d = i.trailing_zeros() as usize;
            for j in (d..self.dep).rev() {
                self.push(i >> j);
            }
        }
    }

    pub fn update(&mut self, rng: Range<usize>, e: E) {
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

    pub fn fold<R: RangeBounds<usize>>(&mut self, rng: R) -> T {
        let Range {
            start: mut l,
            end: mut r,
        } = bounds_within(rng, self.size);
        self.infiltrate(l);
        self.infiltrate(r);
        let mut lx = T::zero();
        let mut rx = T::zero();
        while l < r {
            if l & 1 == 1 {
                lx = lx + self.node[l].val.clone();
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                rx = self.node[r].val.clone() + rx;
            }
            l >>= 1;
            r >>= 1;
        }
        lx + rx
    }
}

impl<T: Monoid + Mul<E, Output = T>, E: Monoid> From<&Vec<T>> for LazySegmentTree<T, E> {
    fn from(arr: &Vec<T>) -> Self {
        let size = arr.len().next_power_of_two();
        let dep = size.trailing_zeros() as usize + 1;
        let mut node = vec![
            Node {
                val: T::zero(),
                lazy: E::zero()
            };
            size << 1
        ];
        for i in 0..arr.len() {
            node[i + size].val = arr[i].clone();
        }
        for i in (1..size).rev() {
            node[i].val = node[i << 1].val.clone() + node[(i << 1) + 1].val.clone();
        }
        Self {
            node: node.into_boxed_slice(),
            size,
            dep,
        }
    }
}

// ------------ Lazy Segment Tree end ------------

#[cfg(test)]
mod rmq_ruq_test {
    use super::*;
    use crate::utils::algebraic_traits::*;
    use std::cmp::min;
    use std::ops::{Add, Mul};

    #[derive(Clone, PartialEq)]
    struct Mm(usize);

    impl Add for Mm {
        type Output = Self;
        fn add(self, right: Self) -> Self {
            Mm(min(self.0, right.0))
        }
    }

    impl Associative for Mm {}

    impl Zero for Mm {
        fn zero() -> Self {
            Mm(std::usize::MAX)
        }
        fn is_zero(&self) -> bool {
            self.0 == std::usize::MAX
        }
    }

    #[derive(Clone, PartialEq)]
    struct Uq(Option<usize>);

    impl Add for Uq {
        type Output = Self;
        fn add(self, right: Self) -> Self {
            if right.0.is_none() {
                self.clone()
            } else {
                right.clone()
            }
        }
    }
    impl Associative for Uq {}

    impl Zero for Uq {
        fn zero() -> Self {
            Uq(None)
        }
        fn is_zero(&self) -> bool {
            self.0.is_none()
        }
    }

    impl Mul<Uq> for Mm {
        type Output = Mm;
        fn mul(self, u: Uq) -> Self::Output {
            if let Some(x) = u.0 {
                Mm(x)
            } else {
                self
            }
        }
    }

    #[test]
    fn rmq_ruq_test() {
        let mut seg = LazySegmentTree::from(&vec![Mm::zero(); 3]);
        seg.update(0..2, Uq(Some(1)));
        seg.update(1..3, Uq(Some(3)));
        seg.update(2..3, Uq(Some(2)));
        assert_eq!(seg.fold(0..3).0, 1);
        assert_eq!(seg.fold(1..3).0, 2);
    }
}
