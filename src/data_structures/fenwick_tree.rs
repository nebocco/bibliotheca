use crate::utils::{
    algebraic_traits::{ Monoid, Group },
    bounds::bounds_within,
};
use std::ops::{ Range, RangeBounds };


// * verified: https://judge.yosupo.jp/submission/28326, https://judge.yosupo.jp/submission/29570
// ------------ FenwickTree with generics start ------------

#[derive(Clone, Debug)]
pub struct FenwickTree<T>(Vec<T>);

impl<T: Monoid> FenwickTree<T> {
    #[inline]
    fn lsb(x: usize) -> usize {
        x & x.wrapping_neg()
    }

    pub fn new(n: usize) -> Self {
        Self(vec![T::zero(); n+1])
    }

    pub fn prefix_sum(&self, i: usize) -> T {
        std::iter::successors(Some(i), |&i| Some(i - Self::lsb(i)))
        .take_while(|&i| i != 0)
        .map(|i| self.0[i].clone())
        .fold(T::zero(), |sum, x| sum + x)
    }

    pub fn add(&mut self, i: usize, x: T) {
        let n = self.0.len();
        std::iter::successors(Some(i + 1), |&i| Some(i + Self::lsb(i)))
        .take_while(|&i| i < n)
        .for_each(|i| self.0[i] = self.0[i].clone() + x.clone());
    }

    /// pred(j, sum(..j)) && !pred(j+1, sum(..j+1))
    pub fn partition(&self, pred: impl Fn(usize, &T) -> bool) -> (usize, T) {
        assert!(pred(0, &self.0[0]), "need to be pred(0, 0)");
        let mut j = 0;
        let mut current = self.0[0].clone();
        let n = self.0.len();
        for d in std::iter::successors(Some(n.next_power_of_two() >> 1), |&d| { Some(d >> 1)})
            .take_while(|&d| d != 0)
        {
            if j + d < n {
                let next = current.clone() + self.0[j + d].clone();
                if pred(j + d, &next) {
                    current = next;
                    j += d;
                }
            }
        }
        (j, current)
    }
}

impl<T: Monoid> From<Vec<T>> for FenwickTree<T> {
    fn from(src: Vec<T>) -> Self {
        let mut table = std::iter::once(T::zero())
            .chain(src.into_iter())
            .collect::<Vec<T>>();
        let n = table.len();
        (1..n)
            .map(|i| (i, i + Self::lsb(i)))
            .filter(|&(_, j)| j < n)
            .for_each(|(i, j)| {
                table[j] = table[j].clone() + table[i].clone();
            });
        Self(table)
    }
}

impl<T: Group> FenwickTree<T> {
    pub fn sum<R: RangeBounds<usize>>(&self, rng: R) -> T {
        let Range { start, end } = bounds_within(rng, self.0.len() - 1);
        self.prefix_sum(end) + -self.prefix_sum(start)
    }
}

// ------------ FenwickTree with generics end ------------


// * verified: https://judge.yosupo.jp/submission/28227
// ------------ FenwickTree without generics start ------------

pub struct Fenwick(Vec<i64>);

impl Fenwick {
    #[inline]
    fn lsb(x: usize) -> usize {
        x & x.wrapping_neg()
    }

    pub fn new(len: usize) -> Self {
        Fenwick(vec![0; len + 1])
    }

    pub fn build_from_slice(src: &[i64]) -> Self {
        let mut table = std::iter::once(0)
            .chain(src.iter().cloned())
            .collect::<Vec<i64>>();
        let n = table.len();
        (1..n)
            .map(|i| (i, i + Self::lsb(i)))
            .filter(|&(_, j)| j < n)
            .for_each(|(i, j)| {
                table[j] += table[i];
            });
        Self(table)
    }

    pub fn prefix_sum(&self, i: usize) -> i64 {
        std::iter::successors(Some(i), |&i| Some(i - Self::lsb(i)))
            .take_while(|&i| i != 0)
            .map(|i| self.0[i])
            .sum::<i64>()
    }

    pub fn sum<R: RangeBounds<usize>>(&self, rng: R) -> i64 {
        let Range { start, end } = bounds_within(rng, self.0.len() - 1);
        self.prefix_sum(end) + -self.prefix_sum(start)
    }

    pub fn add(&mut self, i: usize, x: i64) {
        let n = self.0.len();
        std::iter::successors(Some(i + 1), |&i| Some(i + Self::lsb(i)))
            .take_while(|&i| i < n)
            .for_each(|i| self.0[i] += x);
    }

    fn partition(&self, pred: impl Fn(usize, i64) -> bool) -> (usize, i64) {
        assert!(pred(0, self.0[0]), "need to be pred(0, 0)");
        let mut j = 0;
        let mut current = self.0[0];
        let n = self.0.len();
        for d in std::iter::successors(Some(n.next_power_of_two() >> 1), |&d| { Some(d >> 1)})
            .take_while(|&d| d != 0)
        {
            if j + d < n {
                let next = current + self.0[j + d];
                if pred(j + d, next) {
                    current = next;
                    j += d;
                }
            }
        }
        (j, current)
    }

    pub fn lower_bound(&self, x: i64) -> usize {
        self.partition(|_, y| y < x).0
    }

    pub fn upper_bound(&self, x: i64) -> usize {
        self.partition(|_, y| y <= x).0
    }

    pub fn access(&self, i: usize) -> i64 {
        assert!(i < self.0.len() - 1, "index out of range: vector length is {}, but got index {}", self.0.len() - 1, i );
        self.prefix_sum(i + 1) - self.prefix_sum(i)
    }

    pub fn set(&mut self, i: usize, x: i64) {
        self.add(i, x - self.access(i));
    }

}

// ------------ FenwickTree without generics end ------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let mut bit = Fenwick::new(5);
        bit.add(0, 1);
        bit.add(1, 2);
        assert_eq!(bit.prefix_sum(1), 1);
        assert_eq!(bit.prefix_sum(2), 3);
        assert_eq!(bit.prefix_sum(3), 3);
        bit.add(2, 4);
        bit.add(3, 8);
        bit.add(4, 16);
        assert_eq!(bit.prefix_sum(5), 31);
        assert_eq!(bit.sum(1..3), 6);
        assert_eq!(bit.sum(..=3), 15);
        assert_eq!(bit.sum(..), 31);

        bit.set(0, 5);
        assert_eq!(bit.prefix_sum(3), 11);
        assert_eq!(bit.access(0), 5);
        assert_eq!(bit.access(1), 2);
        assert_eq!(bit.access(2), 4);
        assert_eq!(bit.access(3), 8);
        assert_eq!(bit.access(4), 16);
    }

    #[test]
    fn test_abst() {
        let mut bit = FenwickTree::<i32>::new(5);
        bit.add(0, 0);
        bit.add(1, 1);
        bit.add(2, 10);
        bit.add(3, 100);
        bit.add(4, 1000);
        assert_eq!(bit.prefix_sum(0), 0);
        assert_eq!(bit.prefix_sum(1), 0);
        assert_eq!(bit.prefix_sum(2), 1);
        assert_eq!(bit.prefix_sum(3), 11);
        assert_eq!(bit.prefix_sum(4), 111);
        assert_eq!(bit.prefix_sum(5), 1111);
        bit.add(0, 7);
        assert_eq!(bit.sum(1..3), 11);
        assert_eq!(bit.sum(..=3), 118);
        assert_eq!(bit.sum(..), 1118);
    }

    #[test]
    fn test_1() {
        let a = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        let bit = Fenwick::build_from_slice(&a);
        for i in 0..9 {
            assert_eq!(bit.prefix_sum(i), a[..i].iter().sum::<i64>())
        }

        assert_eq!(bit.lower_bound(7), 2);
        assert_eq!(bit.lower_bound(10), 4);
        assert_eq!(bit.lower_bound(14), 4);
        assert_eq!(bit.upper_bound(14), 5);
        assert_eq!(bit.lower_bound(15), 5);
        assert_eq!(bit.lower_bound(200000), 9);
    }
}
