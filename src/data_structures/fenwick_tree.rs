use crate::utils::bounds::bounds_within;

// * verified: https://judge.yosupo.jp/submission/28326, https://judge.yosupo.jp/submission/29570
// ------------ FenwickTree start ------------
pub trait Monoid {
    type Val: Clone + PartialEq;
    const ZERO: Self::Val;
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
}

pub trait Group: Monoid {
    fn inv(val: &Self::Val) -> Self::Val;
}

#[derive(Clone, Debug)]
pub struct FenwickTree<M: Monoid>(Vec<M::Val>);

impl<M: Monoid> FenwickTree<M> {
    #[inline]
    fn lsb(x: usize) -> usize {
        x & x.wrapping_neg()
    }

    pub fn new(n: usize) -> Self {
        Self(vec![M::ZERO; n + 1])
    }

    pub fn prefix_sum(&self, i: usize) -> M::Val {
        std::iter::successors(Some(i), |&i| Some(i - Self::lsb(i)))
            .take_while(|&i| i != 0)
            .map(|i| self.0[i].clone())
            .fold(M::ZERO, |sum, x| M::op(&sum, &x))
    }

    pub fn add(&mut self, i: usize, x: M::Val) {
        let n = self.0.len();
        std::iter::successors(Some(i + 1), |&i| Some(i + Self::lsb(i)))
            .take_while(|&i| i < n)
            .for_each(|i| self.0[i] = M::op(&self.0[i], &x));
    }

    /// pred(j, sum(..j)) && !pred(j+1, sum(..j+1))
    pub fn partition(&self, pred: impl Fn(usize, &M::Val) -> bool) -> (usize, M::Val) {
        assert!(pred(0, &self.0[0]), "need to be pred(0, 0)");
        let mut j = 0;
        let mut current = self.0[0].clone();
        let n = self.0.len();
        for d in std::iter::successors(Some(n.next_power_of_two() >> 1), |&d| Some(d >> 1))
            .take_while(|&d| d != 0)
        {
            if j + d < n {
                let next = M::op(&current, &self.0[j + d]);
                if pred(j + d, &next) {
                    current = next;
                    j += d;
                }
            }
        }
        (j, current)
    }
}

impl<M: Monoid> From<&Vec<M::Val>> for FenwickTree<M> {
    fn from(src: &Vec<M::Val>) -> Self {
        let mut table = std::iter::once(M::ZERO)
            .chain(src.iter().cloned())
            .collect::<Vec<M::Val>>();
        let n = table.len();
        (1..n)
            .map(|i| (i, i + Self::lsb(i)))
            .filter(|&(_, j)| j < n)
            .for_each(|(i, j)| {
                table[j] = M::op(&table[j], &table[i]);
            });
        Self(table)
    }
}

impl<G: Group> FenwickTree<G> {
    pub fn sum<R>(&self, rng: R) -> G::Val
    where
        R: std::ops::RangeBounds<usize>,
    {
        let rng = bounds_within(rng, self.0.len() - 1);
        G::op(
            &self.prefix_sum(rng.end),
            &G::inv(&self.prefix_sum(rng.start)),
        )
    }
}

// ------------ FenwickTree with generics end ------------

#[cfg(test)]
mod tests {
    use super::*;

    struct Inner;
    impl Monoid for Inner {
        type Val = i32;
        const ZERO: Self::Val = 0;
        fn op(l: &Self::Val, r: &Self::Val) -> Self::Val {
            l + r
        }
    }

    impl Group for Inner {
        fn inv(val: &Self::Val) -> Self::Val {
            -val
        }
    }

    #[test]
    fn test_fenwick_tree() {
        let mut bit = FenwickTree::<Inner>::new(5);
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
    fn test_fenwick_tree_partition() {
        let a = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        let bit = FenwickTree::<Inner>::from(&a);
        for i in 0..9 {
            assert_eq!(bit.prefix_sum(i), a[..i].iter().sum::<i32>())
        }
        assert_eq!(bit.partition(|_, sum| *sum < 7), (2, 4));
        assert_eq!(bit.partition(|_, sum| *sum < 10), (4, 9));
        assert_eq!(bit.partition(|_, sum| *sum < 13), (4, 9));
        assert_eq!(bit.partition(|_, sum| *sum < 16), (5, 14));
        assert_eq!(bit.partition(|_, sum| *sum < 2), (0, 0));
    }
}
