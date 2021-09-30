// verified: 
// point add range sum: https://judge.yosupo.jp/submission/61855
// point set range composite: https://judge.yosupo.jp/submission/61856
// range affine range sum: https://judge.yosupo.jp/submission/61858
// dynamic sequence range affine range sum: https://judge.yosupo.jp/submission/61859

use std::ops::{Mul, Range};
use crate::utils::algebraic_traits::{Monoid, Pow};
use crate::utils::random::XorShift;

struct Node<T, E> {
    value: T,
    acc: T,
    lazy: E,
    priority: usize,
    cnt: usize,
    rev: bool,
    left: Option<Box<Node<T, E>>>,
    right: Option<Box<Node<T, E>>>,
}

impl<T: Monoid + Mul<E, Output=T>, E: Monoid + Pow> Node<T, E> {
    fn new(value: T, priority: usize) -> Self {
        Self {
            value,
            acc: T::zero(),
            lazy: E::zero(),
            priority,
            cnt: 0,
            rev: false,
            left: None,
            right: None,
        }
    }

    fn count(&self) -> usize { self.cnt }

    fn accumulate(&self) -> T { self.acc.clone() }

    fn update_cnt(&mut self) {
        self.cnt = self.left.as_ref().map(|c| c.count()).unwrap_or(0)
            + 1
            + self.right.as_ref().map(|c| c.count()).unwrap_or(0);
    }

    fn update_acc(&mut self) {
        self.acc = self.left.as_ref().map(|c| c.accumulate()).unwrap_or_else(T::zero)
            + self.value.clone()
            + self.right.as_ref().map(|c| c.accumulate()).unwrap_or_else(T::zero);
    }

    fn push_up(&mut self) {
        self.update_cnt();
        self.update_acc();
    }

    fn push_down(&mut self) {
        if self.rev {
            self.rev = false;
            std::mem::swap(&mut self.left, &mut self.right);
            if let Some(ref mut node) = self.left {
                node.rev ^= true;
            }
            if let Some(ref mut node) = self.right {
                node.rev ^= true;
            }
        }
        if !self.lazy.is_zero() {
            if let Some(ref mut node) = self.left {
                node.lazy = node.lazy.clone() + self.lazy.clone();
                node.acc = node.acc.clone() * self.lazy.pow(node.count() as u32);
            }

            if let Some(ref mut node) = self.right {
                node.lazy = node.lazy.clone() + self.lazy.clone();
                node.acc = node.acc.clone() * self.lazy.pow(node.count() as u32);
            }

            self.value = self.value.clone() * self.lazy.clone(); // Why can't we remove clone() to self.value?
            self.lazy = E::zero();
        }

        self.push_up();
    }

    fn split(mut self: Box<Node<T, E>>, key: usize) -> (Option<Box<Node<T, E>>>, Option<Box<Node<T, E>>>) {
        self.push_down();
        let implicit_key = self.left.as_ref().map(|c| c.count()).unwrap_or(0) + 1;
        if key < implicit_key {
            let (l, left) = if let Some(node) = self.left.take() {
                node.split(key)
            } else {
                (None, None)
            };
            
            // if let Some(ref mut node) = left {
            //     node.push_up();
            // }

            self.left = left;
            self.push_up();
            (l, Some(self))
        } else {
            let (right, r) = if let Some(node) = self.right.take() {
                node.split(key - implicit_key)
            } else {
                (None, None)
            };

            // if let Some(ref mut node) = right {
            //     node.push_up();
            // }

            self.right = right;
            self.push_up();
            (Some(self), r)
        }
    }

    fn merge(l: Option<Box<Node<T, E>>>, r: Option<Box<Node<T, E>>>) -> Option<Box<Node<T, E>>> {
        match (l, r) {
            (Some(mut left), Some(mut right)) => {
                left.push_down();
                right.push_down();
                if left.priority > right.priority {
                    left.right = Node::merge(left.right, Some(right));
                    left.push_up();
                    Some(left)
                } else {
                    right.left = Node::merge(Some(left), right.left);
                    right.push_up();
                    Some(right)
                }
            }
            (Some(left), None) => {
                Some(left)
            }
            (None, Some(right)) => {
                Some(right)
            }
            (None, None) => {
                None
            }
        }
    }

    fn insert(self: Box<Node<T, E>>, key: usize, node: Node<T, E>) -> Box<Node<T, E>> {
        let (l, r) = self.split(key);
        Node::merge(Node::merge(l, Some(Box::new(node))), r).unwrap()
    }

    fn remove(self: Box<Node<T, E>>, key: usize) -> Option<Box<Node<T, E>>> {
        let (rest, r) = self.split(key + 1);
        if let Some(node) = rest {
            let (l, _) = node.split(key);
            Node::merge(l, r)
        } else {
            r
        }
    }

    fn update(self: &mut Box<Node<T, E>>, rng: Range<usize>, eff: E) {
        let root = std::mem::replace(self, Box::new(Node::<T, E>::new(T::zero(), 0)));
        
        let (rest, r) = root.split(rng.end);
        *self = if let Some(node) = rest {
            let (l, target) = node.split(rng.start);
            if let Some(mut node) = target {
                node.lazy = node.lazy + eff;
                node.acc = node.acc.clone() * node.lazy.pow(node.count() as u32);
                Node::merge(Node::merge(l, Some(node)), r)
            } else {
                Node::merge(l, r)
            }
        } else {
            r
        }.unwrap();
    }

    fn fold(self: &mut Box<Node<T, E>>, rng: Range<usize>) -> T {
        let root = std::mem::replace(self, Box::new(Node::<T, E>::new(T::zero(), 0)));
        let mut ret = T::zero();
        let (rest, r) = root.split(rng.end);
        *self = if let Some(node) = rest {
            let (l, target) = node.split(rng.start);
            if let Some(node) = target {
                ret = node.accumulate();
                Node::merge(Node::merge(l, Some(node)), r)
            } else {
                Node::merge(l, r)
            }
        } else {
            r
        }.unwrap();
        ret
    }

    fn reverse(self: &mut Box<Node<T, E>>, rng: Range<usize>) {
        let root = std::mem::replace(self, Box::new(Node::<T, E>::new(T::zero(), 0)));
        
        let (rest, r) = root.split(rng.end);
        *self = if let Some(node) = rest {
            let (l, target) = node.split(rng.start);
            if let Some(mut node) = target {
                node.rev ^= true;
                Node::merge(Node::merge(l, Some(node)), r)
            } else {
                Node::merge(l, r)
            }
        } else {
            r
        }.unwrap();
    }

    fn rotate_left(self: &mut Box<Node<T, E>>, rng: Range<usize>, amount: usize) {
        assert!(amount < rng.len());
        let Range{start, end} = rng;
        self.reverse(start..end);
        self.reverse(start..end - amount);
        self.reverse(end-amount..end);
    }

    fn bisect(self: &mut Box<Node<T, E>>, val: T, offset: usize) -> Option<usize> {
        if self.accumulate() + val.clone() == val {
            return None;
        } else {
            let left_count = self.left.as_ref().map(|node| node.count()).unwrap_or(0);

            if let Some(ref mut right) = self.right {
                if right.accumulate() + val.clone() != val {
                    return right.bisect(val, offset + left_count + 1);
                }
            }

            if self.value.clone() + val.clone() != val {
                Some(offset + left_count)
            } else if let Some(ref mut left) = self.left {
                left.bisect(val, offset)
            } else {
                None
            }
        }
    }

    // very slow
    fn dump(&mut self) -> Vec<T> {
        self.push_down();
        let mut res = Vec::with_capacity(self.count());
        if let Some(ref mut node) = self.left {
            res.extend(node.dump());
        }
        res.push(self.value.clone());
        if let Some(ref mut node) = self.right {
            res.extend(node.dump());
        }
        res
    }
}

pub struct Treap<T, E> {
    rand: XorShift,
    root: Option<Box<Node<T, E>>>
}

impl<T: Monoid + Mul<E, Output=T>, E: Monoid + Pow> Treap<T, E> {
    pub fn new() -> Self {
        Self {
            rand: XorShift::default(),
            root: None
        }
    }

    pub fn from(lis: Vec<T>) -> Self {
        let mut treap = Self::new();
        for (i, x) in lis.into_iter().enumerate() {
            treap.insert(i, x);
        }
        treap
    }

    pub fn len(&self) -> usize {
        match self.root {
            Some(ref node) => node.count(),
            None => 0
        }
    }

    pub fn insert(&mut self, pos: usize, val: T) {
        let root = self.root.take();
        let new_node = Node::new(val, self.rand.gen() as usize);
        self.root = if let Some(node) = root {
            Some(node.insert(pos, new_node))
        } else {
            Some(Box::new(new_node))
        };
    }

    pub fn update(&mut self, rng: Range<usize>, eff: E) {
        if let Some(ref mut node) = self.root {
            node.update(rng, eff);
        };
    }

    pub fn fold(&mut self, rng: Range<usize>) -> T {
        if let Some(ref mut node) = self.root {
            node.fold(rng)
        } else {
            T::zero()
        }
    }

    /// leftist position which satisfies lis[pos] + val != val.
    pub fn binary_search(&mut self, rng: Range<usize>, val: T) -> Option<usize> {
        let mut res = None;
        let root = self.root.take();
        if let Some(node) = root {
            let (rest, right) = node.split(rng.end);
            if let Some(node) = rest {
                let (left, mut target) = node.split(rng.start);
                if let Some(ref mut node) = target {
                    res = node.bisect(val, rng.start);
                }
                self.root = Node::merge(Node::merge(left, target), right);
            } else {
                self.root = Node::merge(rest, right);
            }
        }
        res
    }

    pub fn remove(&mut self, pos: usize) {
        let root = self.root.take();
        if let Some(node) = root {
            self.root = node.remove(pos);
        };
    }

    pub fn reverse(&mut self, rng: Range<usize>) {
        if let Some(ref mut node) = self.root {
            node.reverse(rng);
        };
    }

    pub fn rotate_left(&mut self, rng: Range<usize>, amount: usize) {
        if let Some(ref mut node) = self.root {
            node.rotate_left(rng, amount);
        };
    }

    pub fn dump(&mut self) -> Vec<T> {
        if let Some(ref mut node) = self.root {
            node.dump()
        } else {
            Vec::new()
        }
    }

    pub fn access(&mut self, pos: usize) -> T {
        self.root.as_mut().unwrap().fold(pos..pos+1)
    }
}

// ------------ Implicit Treap end ------------

#[cfg(test)]
mod test {
    use std::ops::*;
    use super::*;
    use crate::utils::algebraic_traits::*;

    #[test]
    fn test_min_update() {
        #[derive(Clone, PartialEq, Debug)]
        struct MinT(i64);

        impl Associative for MinT {}

        impl Zero for MinT {
            fn zero() -> Self { Self(std::i64::MAX) }
            fn is_zero(&self) -> bool { self.0 == std::i64::MAX }
        }

        impl Add<Self> for MinT {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0.min(rhs.0))
            }
        }

        #[derive(Clone, PartialEq, Debug)]
        struct UpdE(Option<i64>);

        impl Associative for UpdE {}

        impl Zero for UpdE {
            fn zero() -> Self { Self(None) }
            fn is_zero(&self) -> bool { self.0.is_none() }
        }

        impl Add<Self> for UpdE {
            type Output = Self;
            // rhs has priority
            fn add(self, rhs: Self) -> Self::Output {
                if rhs.0.is_none() {
                    self
                } else {
                    rhs
                }
            }
        }

        impl Mul<UpdE> for MinT {
            type Output = MinT;
            fn mul(self, rhs: UpdE) -> Self::Output {
                if let Some(val) = rhs.0 {
                    MinT(val)
                } else {
                    self
                }
            }
        }

        impl Pow for UpdE {
            fn pow(&self, _: u32) -> Self {
                self.clone()
            }
        }

        let n = 10;
        let mut lis: Vec<MinT> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3].into_iter().map(|v| MinT(v)).collect();
        let mut treap = Treap::<MinT, UpdE>::from(lis.clone());
        assert_eq!(treap.dump(), lis);
        for j in 0..n {
            assert_eq!(treap.access(j), lis[j].clone());
            for i in 0..j {
                assert_eq!(
                    treap.fold(i..j),
                    (i..j).map(|i| lis[i].clone()).fold(MinT::zero(), |s, x| s + x),
                    "{}..{}", i, j
                );
            }
        }

        let effects = vec![
            (3..6, UpdE(Some(4))),
            (2..7, UpdE(Some(7))),
            (4..6, UpdE(Some(5))),
        ];

        for (rng, eff) in effects {
            for i in rng.clone() {
                lis[i] = lis[i].clone() * eff.clone();
            }
            treap.update(rng, eff);
        }

        for j in 0..n {
            for i in 0..j {
                assert_eq!(
                    treap.fold(i..j),
                    (i..j).map(|i| lis[i].clone()).fold(MinT::zero(), |s, x| s + x),
                    "{}..{}", i, j
                );
            }
        }

        treap.reverse(2..6);
        for i in 2..4 {
            lis.swap(i, 7 - i);
        }

        for j in 0..n {
            for i in 0..j {
                assert_eq!(
                    treap.fold(i..j),
                    (i..j).map(|i| lis[i].clone()).fold(MinT::zero(), |s, x| s + x),
                    "{}..{}", i, j
                );
            }
        }

        treap.rotate_left(1..8, 3);
        lis[1..8].rotate_left(3);

        for j in 0..n {
            for i in 0..j {
                assert_eq!(
                    treap.fold(i..j),
                    (i..j).map(|i| lis[i].clone()).fold(MinT::zero(), |s, x| s + x),
                    "{}..{}", i, j
                );
            }
        }
    }
}