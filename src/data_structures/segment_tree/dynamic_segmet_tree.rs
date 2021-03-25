use crate::utils::{
	algebraic_traits::Monoid,
	bounds::bounds_within,
};

use std::ops::{ Range, RangeBounds };

// TODO: verify
// ------------ Dynamic Segment Tree start ------------

pub enum Node<T: Monoid> {
    Leaf(Leaf<T>),
    Section(Box<Section<T>>),
    None,
}

pub struct Leaf<T: Monoid> {
    i: usize,
    val: T,
}

pub struct Section<T: Monoid> {
    left: Node<T>,
    right: Node<T>,
    val: T,
}

impl<T: Monoid> Leaf<T> {
    fn new(i: usize, x: T) -> Self { Leaf { i: i, val: x } }
    fn fold(&self) -> T { self.val.clone() }
}

impl<T: Monoid> Section<T> {
    fn new() -> Self {
        Section {
            left: Node::None,
            right: Node::None,
            val: T::zero(),
        }
    }
    fn fold(&self) -> T { self.val.clone() }
    fn update(&mut self, i: usize, x: T, l: usize, r: usize) {
        let m = (l + r) >> 1;
        if i < m {
            let left = self.left.take();
            self.left = left.update(i, x, l, m);
        }
        else {
            let right = self.right.take();
            self.right = right.update(i, x, m, r);
        }
        self.val = self.left.fold() + self.right.fold();
    }
}

impl<T: Monoid> Node<T> {
    fn take(&mut self) -> Node<T> {
        std::mem::replace(self, Node::None)
    }
    fn fold(&self) -> T {
        match self {
            &Node::Section(ref sec) => sec.as_ref().fold(),
            &Node::Leaf(ref leaf) => leaf.fold(),
            &Node::None => T::zero(),
        }
    }
    fn update(self, i: usize, x: T, l: usize, r: usize) -> Self {
        match self {
            Node::Section(mut sec) => {
                sec.as_mut().update(i, x, l, r);
                Node::Section(sec)
            }
            Node::Leaf(leaf) => {
                if leaf.i == i {
                    Node::Leaf(Leaf::new(i, x))
                } else {
                    let mut new_section = Section::new();
                    let m = (l + r) >> 1;
                    if leaf.i < m {
                        new_section.left = Node::Leaf(leaf);
                    } else {
                        new_section.right = Node::Leaf(leaf);
                    }
                    new_section.update(i, x, l, r);
                    Node::Section(Box::new(new_section))
                }
            }
            Node::None => {
                Node::Leaf(Leaf::new(i, x))
            }
        }
    }
    fn range_fold(&self, a: usize, b: usize, l: usize, r: usize) -> T {
        match self {
            &Node::Section(ref sec) => {
                if b <= l || r <= a { T::zero() }
                else if a <= l && r <= b { sec.fold() }
                else {
                    let m = (l + r) >> 1;
                    return sec.left.range_fold(a, b, l, m) + sec.right.range_fold(a, b, m, r);
                }
            }
            &Node::Leaf(ref leaf) => {
                if a <= leaf.i && leaf.i < b { leaf.fold() }
                else { T::zero() }
            }
            &Node::None => T::zero(),
        }
    }
}

pub struct DynamicSegmentTree<T: Monoid> {
    root: Node<T>,
    size: usize,
}

impl<T: Monoid> DynamicSegmentTree<T> {
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        DynamicSegmentTree {
            root: Node::None,
            size,
        }
    }
    pub fn update(&mut self, i: usize, x: T) {
        let r = self.root.take();
        self.root = r.update(i, x, 0, self.size);
    }
    pub fn fold<R: RangeBounds<usize>>(&self, rng: R) -> T {
        let Range { start: l, end: r } = bounds_within(rng, self.size);
        self.root.range_fold(l, r, 0, self.size)
    }
}

// ------------ Dynamic Segment Tree end ------------